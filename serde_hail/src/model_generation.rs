use anyhow::{anyhow, Result};
use quote::{
    __private::{Ident, TokenStream},
    format_ident, quote,
};

use parser::types::{encoding::VirtualHint, EType, ETypeShape};

#[allow(dead_code)]
type RustType = TokenStream;
#[allow(dead_code)]
type RustTypeDefinition = TokenStream;

pub fn encoded_type_to_rust_type(name: &str, e: &EType, derivations: &[String]) -> Result<String> {
    let derivations: Vec<Ident> = derivations.iter().map(|s| format_ident!("{}", s)).collect();

    let (_, type_definitions) = process_type(name, e, &derivations)?;

    let type_definitions: String = type_definitions
        .iter()
        .map(|def| def.to_string())
        .collect::<Vec<String>>()
        .concat();

    Ok(type_definitions)
}

fn process_type(
    name: &str,
    e: &EType,
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let mut type_definitions: Vec<RustTypeDefinition> = vec![];

    let mut rust_type = match &e.shape {
        ETypeShape::BaseStruct(fields) => {
            let (rust_type, new_typedefs) = match &e.virtual_hint {
                Some(VirtualHint::Locus(_)) => (quote! { Locus }, vec![]),
                Some(VirtualHint::Interval) => handle_interval(name, fields, derivations)?,
                Some(VirtualHint::Tuple) => handle_tuple(fields, derivations)?,
                None => handle_plain_struct(name, fields, derivations)?,
                Some(_) => return Err(unexpected_combination("EBaseStruct", &e.virtual_hint)),
            };
            type_definitions = new_typedefs;
            rust_type
        }

        ETypeShape::Array(field) => {
            let (rust_type, new_typedefs) = match &e.virtual_hint {
                Some(VirtualHint::Dict) => handle_dict(name, field, derivations)?,
                Some(VirtualHint::Set) => handle_set(name, field, derivations)?,
                None => handle_list(name, field, derivations)?,
                Some(_) => return Err(unexpected_combination("EArray", &e.virtual_hint)),
            };
            type_definitions = new_typedefs;
            rust_type
        }

        ETypeShape::NdArrayColumnMajor(field, d) => {
            let d = *d as usize;
            let (inner_rust_type, new_typedefs) = process_type("[[e]]", field, derivations)?;
            type_definitions = new_typedefs;
            quote! { NDArray<#inner_rust_type, #d> }
        }

        ETypeShape::Binary => {
            assert_eq!(e.virtual_hint, Some(VirtualHint::String));
            quote! { String }
        }
        ETypeShape::Float32 => quote! { f32 },
        ETypeShape::Float64 => quote! { f64 },
        ETypeShape::Int32 => match &e.virtual_hint {
            Some(VirtualHint::Call) => quote! { Call },
            None => quote! { u32 },
            _ => return Err(unexpected_combination("EInt32", &e.virtual_hint)),
        },
        ETypeShape::Int64 => quote! { i64 },
        ETypeShape::Boolean => quote! { bool },
    };

    if !e.required {
        rust_type = quote! { Option<#rust_type> }
    }

    Ok((rust_type, type_definitions))
}

fn handle_list(
    name: &str,
    field: &EType,
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let field_name = format!("{}_element", name);
    let (inner_rust_type, new_typedefs) = process_type(&field_name, field, derivations)?;
    Ok((quote! { Vec<#inner_rust_type> }, new_typedefs))
}

fn handle_set(
    name: &str,
    field: &EType,
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let field_name = format!("{}_element", name);
    let (inner_rust_type, new_typedefs) = process_type(&field_name, field, derivations)?;
    Ok((quote! { BTreeSet<#inner_rust_type> }, new_typedefs))
}

fn handle_interval(
    name: &str,
    fields: &[(String, EType)],
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let (start_name, start) = &fields[0];
    let (end_name, end) = &fields[1];

    assert_eq!(start_name, "start");
    assert_eq!(end_name, "end");
    assert_eq!(start, end);

    let interval_bound_name = format!("{}_bound", name);

    let (rust_type, new_typedefs) = process_type(&interval_bound_name, start, derivations)?;

    Ok((quote! { Interval<#rust_type> }, new_typedefs))
}

fn handle_tuple(
    fields: &[(String, EType)],
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let (rust_fields, typedefs) = process_fields(fields, derivations)?;

    Ok((quote! { (#(#rust_fields),*) }, typedefs))
}

fn handle_plain_struct(
    name: &str,
    fields: &[(String, EType)],
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let struct_name = to_struct_name(name);

    if fields.is_empty() {
        return Ok((
            quote! { #struct_name },
            vec![quote! {
                #[derive(#(#derivations),*)]
                pub struct #struct_name;
            }],
        ));
    }

    let (rust_fields, mut typedefs) = process_fields(fields, derivations)?;

    // Generate the type definition for the new struct
    let new_struct_def = {
        let field_names: Vec<Ident> = fields
            .iter()
            .map(|(field_name, _)| to_field_name(field_name))
            .collect();
        quote! {
            #[derive(#(#derivations),*)]
            pub struct #struct_name {
                #(#field_names: #rust_fields,)*
            }
        }
    };

    typedefs.push(new_struct_def);

    Ok((quote! { #struct_name }, typedefs))
}

fn process_fields(
    fields: &[(String, EType)],
    derivations: &[Ident],
) -> Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    // Process each field individually
    let rust_fields: Vec<(TokenStream, Vec<TokenStream>)> = fields
        .iter()
        .map(|(field_name, field_encoded_type)| {
            process_type(field_name, field_encoded_type, derivations)
        })
        .collect::<Result<Vec<(TokenStream, Vec<TokenStream>)>>>()?;

    let rust_field_types: Vec<TokenStream> =
        rust_fields.iter().map(|(t, _)| t.to_owned()).collect();

    // All the type defs go in a vector
    let typedefs: Vec<TokenStream> = rust_fields
        .iter()
        .map(|(_, defs)| defs.to_owned())
        .flatten()
        .collect();

    Ok((rust_field_types, typedefs))
}

fn handle_dict(
    name: &str,
    entry: &EType,
    derivations: &[Ident],
) -> Result<(RustType, Vec<RustTypeDefinition>)> {
    let (key, value) = match &entry.shape {
        ETypeShape::BaseStruct(fields) => {
            let (key_name, key) = &fields[0];
            let (value_name, value) = &fields[1];

            assert_eq!(key_name, "key");
            assert_eq!(value_name, "value");

            (key, value)
        }
        _ => panic!(),
    };

    let key_bound_name = format!("{}_key", name);
    let value_bound_name = format!("{}_value", name);

    let (key_rust_type, key_typedefs) = process_type(&key_bound_name, key, derivations)?;
    let (value_rust_type, value_typedefs) = process_type(&value_bound_name, value, derivations)?;

    let mut additional = key_typedefs;
    additional.extend(value_typedefs);

    Ok((
        quote! { BTreeMap<#key_rust_type, #value_rust_type> },
        additional,
    ))
}

fn to_field_name(name: &str) -> Ident {
    match name {
        "`the entries! [877f12a8827e18f61222c6c8c5fb04a8]`" => {
            quote::format_ident!("entries")
        }
        _ => quote::format_ident!("{}", inflector::cases::snakecase::to_snake_case(name)),
    }
}

fn to_struct_name(name: &str) -> Ident {
    match name {
        "`the entries! [877f12a8827e18f61222c6c8c5fb04a8]`_element" => {
            quote::format_ident!("Entry")
        }
        _ => quote::format_ident!("{}", inflector::cases::pascalcase::to_pascal_case(name)),
    }
}

fn unexpected_combination(encoded: &str, hint: &Option<VirtualHint>) -> anyhow::Error {
    anyhow!(
        "unexpected encoded and virtual type combination: {} + {:?}",
        encoded,
        hint
    )
}

// Currently rustfmt-nightly (the crate) is not compiling.
// /// Programmatically runs rustfmt on a `String`.
// /// https://github.com/bcmyers/num-format/blob/v0.1.2/num-format-dev/src/rustfmt.rs
// pub fn rustfmt<S>(module: S) -> Result<String, failure::Error>
// where
//     S: Into<String>,
// {
//     let input = Input::Text(module.into());

//     let mut config = Config::default();
//     config.set().edition(Edition::Edition2018);
//     config.set().emit_mode(EmitMode::Stdout);

//     let mut output = Vec::new();
//     {
//         let mut session = Session::new(config, Some(&mut output));
//         let _format_report = session.format(input)?;
//     }
//     let s = String::from_utf8(output)?;
//     Ok(s)
// }

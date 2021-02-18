# ***Hail Table Parser***

This crate is a parser for hail tables and hail matrix tables from the [Hail project].


> [Hail] is an open-source, general-purpose, Python-based data analysis tool with additional data types and methods for working with genomic data.

> Hail is built to scale and has first-class support for multi-dimensional structured data, like the genomic data in a genome-wide association study (GWAS).


**About this explorative side project:**
- *Tagline:* parse hail tables and matrix tables.
- *Explores:* parsing of binary files using a runtime schema.


**Features:**
- Parses normal tables, matrix tables, or individual components.
- Handles both V1 and V2 component types.
- Easy-to-extend building blocks.
- Parses all tables in the [resources folder] in the Hail project source, with the exception of the [backward compatibility folder] (from which only some are parsed).
- Parsing of virtual and encoded schemas.
- Generic physical encoding to support both LEB128 and little endian integer types at no runtime cost.


**Not supported:**
- Full backward compatibility
- Indexes & references
- Partial loading (the entire table is loaded into memory)


**Non goals:**
- Extensive correctness [testing].
  > While important, this is meant to be a learning project not production software.
- Helpful error reporting.
  > While the parser takes every opportunity it has to find unexpected behaviour and report an error, those errors are somewhat opaque.




[Hail project]: https://github.com/hail-is/hail
[Hail]: https://hail.is/
[resources folder]: https://github.com/hail-is/hail/tree/main/hail/src/test/resources
[backward compatibility folder]: https://github.com/hail-is/hail/tree/main/hail/src/test/resources/backward_compatability
[testing]: parser/tests
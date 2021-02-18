# ***Parse module***

***Data:***

The [./data] submodule is the core of the crate, it handles the conversion of the raw (decompressed) data into the appropriate representation using the schema present in the metadata.

***Schema:***

The [./schema] submodule handles the conversion of the schemas present in the metadata (which are in the form of strings) to the appropriate types.

It also implements the deserialization of those sections of the metadata, and the conversion from the specific component metadata versions to an harmonised one with a fleshed out EType (it involves some schema manipulation).

[./data]: data
[./schema]: schema
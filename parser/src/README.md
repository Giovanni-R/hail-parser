# ***The Parser***

This is the parsing crate, it contains all the pieces needed to load a Hail (matrix) table into memory.

***Types***

The [./types] subfolder holds the base types including:
- HailValue: holds the parsed data.
- EType: the encoded schema, uses basic primitives.
- VType: the virtual schema, it describes how the EType primitives translate to a HailValue.
- Metadata: models the shape of the json documents that describe the (matrix) tables and their components.


***Load***

The [./load] subfolder holds the logic needed to navigate a table's folder, handle compression, and load data and metadata into memory.

***Parse***

The [./parse] subfolder is the core of the crate. It handles the parsing of both the virtual and encoded schemas as well as the data itself.


[./types]: types
[./load]: load
[./parse]: parse
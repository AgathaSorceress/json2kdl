# json2kdl

json2kdl is a program that generates KDL files from JSON

![A terminal screenshot of me running cat to show the contents of an example json file, then running json2kdl on it and showing the kdl output in a new file](https://eldritchcafe.files.fedi.monster/media_attachments/files/109/972/564/372/013/139/original/ab2e73d17967d369.png)

## Intended Use

json2kdl was made specifically for [Nixpkgs Issue #198655](https://github.com/NixOS/nixpkgs/issues/198655),
 which means, these features are currently out of scope:
- Parsing arbitrary JSON  
  Currently, the input file structure must follow a specific schema:
  - [Nodes](https://github.com/kdl-org/kdl/blob/main/SPEC.md#node) can be defined as fields of the root JSON object (`{}`)
  - Each node can have these optional fields:
    - `arguments` (Array of [Values](https://github.com/kdl-org/kdl/blob/main/SPEC.md#value))
    - `properties` (Object with [Identifier](https://github.com/kdl-org/kdl/blob/main/SPEC.md#identifier):[Value](https://github.com/kdl-org/kdl/blob/main/SPEC.md#value) pairs)
    - `children` (Object with fields representing [Nodes](https://github.com/kdl-org/kdl/blob/main/SPEC.md#node))  
    
  See `examples/example.json`
- Comments (As JSON does not support them)
- Type Annotations
- Types that KDL has but JSON does not and vice versa
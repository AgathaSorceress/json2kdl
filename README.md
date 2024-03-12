# json2kdl

json2kdl is a program that generates KDL files from JSON

```sh
json2kdl input.json output.kdl
```

## Intended Use

json2kdl was made specifically for [Nixpkgs Issue #198655](https://github.com/NixOS/nixpkgs/issues/198655),
 which means, these features are currently out of scope:
- Parsing arbitrary JSON  
  Currently, the input file structure must follow a specific schema:
  - [Nodes](https://github.com/kdl-org/kdl/blob/main/SPEC.md#node) can be defined as elements of the root JSON array (`[]`)
  - Each node must have the `identifier` field of type [Identifier](https://github.com/kdl-org/kdl/blob/main/SPEC.md#identifier) and can have these optional fields:
    - `arguments` (Array of [Values](https://github.com/kdl-org/kdl/blob/main/SPEC.md#value))
    - `properties` (Object with [Identifier](https://github.com/kdl-org/kdl/blob/main/SPEC.md#identifier):[Value](https://github.com/kdl-org/kdl/blob/main/SPEC.md#value) pairs)
    - `children` (Array of objects representing [Nodes](https://github.com/kdl-org/kdl/blob/main/SPEC.md#node))  
    
  See `examples/example.json`
- Comments (As JSON does not support them)
- Type Annotations
- Types that KDL has but JSON does not and vice versa

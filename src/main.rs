use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue, NodeKey};
use miette::{miette, IntoDiagnostic, Result};
use serde_json::Value;
use std::{env, fs};

fn main() -> Result<()> {
    // Get paths for input and output files from first 2 passed arguments
    let input = env::args()
        .nth(1)
        .ok_or_else(|| miette!("Please provide an input JSON file path"))?;

    let output = env::args()
        .nth(2)
        .ok_or_else(|| miette!("Please provide an output KDL file path"))?;

    // Read input file to string
    let input = fs::read_to_string(&input)
        .into_diagnostic()
        .map_err(|err| err.context(format!("Could not read file `{input}`")))?;

    // Parse input file into JSON
    let input: Value = serde_json::from_str(&input).into_diagnostic()?;

    // Parse JSON to KDL and write to output file
    fs::write(output, json_to_kdl(input)?.to_string()).into_diagnostic()?;

    Ok(())
}

fn json_to_kdl(json: Value) -> Result<KdlDocument> {
    let nodes: Vec<Result<KdlNode>> = json
        .as_array()
        .ok_or_else(|| miette!("Document root must be a JSON array"))?
        .iter()
        .map(|value| {
            let mut node = KdlNode::new(
                value
                    .get("name")
                    .and_then(|ident| ident.as_str())
                    .ok_or_else(|| miette!("`name` must exist and be a String"))?,
            );

            if let Some(arguments) = value.get("arguments") {
                let args: Vec<KdlEntry> = arguments
                    .as_array()
                    .ok_or_else(|| miette!("`arguments` must be an Array"))?
                    .iter()
                    .filter_map(|v| entry_to_kdl(v.to_owned()).ok())
                    .collect();

                for entry in args {
                    node.push(entry);
                }
            };

            if let Some(properties) = value.get("properties") {
                let properties: Vec<(NodeKey, KdlEntry)> = properties
                    .as_object()
                    .ok_or_else(|| miette!("`properties` must be an Object"))?
                    .iter()
                    .filter_map(|(key, value)| match entry_to_kdl(value.to_owned()) {
                        Ok(val) => Some((key.to_owned().into(), val)),
                        Err(_) => None,
                    })
                    .collect();

                for (key, entry) in properties {
                    node.insert(key, entry);
                }
            };

            if let Some(children) = value.get("children") {
                node.set_children(json_to_kdl(children.to_owned())?);
            };

            if let Some(ty) = value.get("type") {
                if !ty.is_null() {
                    node.set_ty(
                        ty.as_str()
                            .ok_or_else(|| miette!("`type` must be a String"))?,
                    );
                }
            }

            Ok(node)
        })
        .collect();

    let mut document = KdlDocument::new();

    for node in nodes {
        document.nodes_mut().push(node?);
    }

    Ok(document)
}

/// Try converting a JSON Value into a KDL entry
fn entry_to_kdl(value: Value) -> Result<KdlEntry> {
    let ty = value
        .get("type")
        .and_then(|t| t.as_str())
        .map(|t| t.to_owned());
    let value = value.get("value").map(|v| v.to_owned()).unwrap_or(value);
    let kdl_value = match value {
        Value::Null => Ok(KdlValue::Null),
        Value::Bool(bool) => Ok(KdlValue::Bool(bool)),
        Value::Number(num) => {
            if num.is_f64() {
                Ok(KdlValue::Base10Float(num.as_f64().ok_or_else(|| {
                    miette!("{num} cannot be parsed into a float")
                })?))
            } else {
                Ok(KdlValue::Base10(num.as_i64().ok_or_else(|| {
                    miette!("{num} cannot be parsed into a number")
                })?))
            }
        }
        Value::String(string) => Ok(KdlValue::String(string)),
        _ => Err(miette!("Type cannot be represented as a KDL value")),
    }?;
    let mut entry = KdlEntry::new(kdl_value);
    if let Some(t) = ty {
        entry.set_ty(t);
    }
    Ok(entry)
}

#[test]
fn test_conversion() -> Result<()> {
    let input = serde_json::json!(
    [
      {
        "name": "bees",
        "arguments": [
          true,
          42,
          {
            "value": 3.1415,
            "type": "my-neat-float"
          },
          null,
          "how many eggs are you currently holding?"
        ],
        "properties": {
          "how many": "uhhh like 40?",
          "state?": "quite upset"
        }
      },
      {
        "name": "lemon",
        "children": [
          {
            "name": "child",
            "properties": {
              "age": {
                "value": 3,
                "type": "my-super-cool-int"
               },
             }
          },
          {
            "name": "child-eater",
            "arguments": [
              ":^)"
            ],
            "type": null
          }
        ]
      },
      {
        "name": "ohno",
        "type": "ohnono"
      }
    ]);

    assert_eq!(
        json_to_kdl(input)?.to_string(),
        "bees true 42 (my-neat-float)3.1415 null \"how many eggs are you currently holding?\" \"how many\"=\"uhhh like 40?\" state?=\"quite upset\"\nlemon {\n    child age=(my-super-cool-int)3\n    child-eater \":^)\"\n}\n(ohnono)ohno\n"
    );

    Ok(())
}

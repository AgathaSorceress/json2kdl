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
        .as_object()
        .ok_or_else(|| miette!("Document root must be a JSON object"))?
        .iter()
        .map(|(key, value)| {
            let mut node = KdlNode::new(key.as_str());

            if let Some(arguments) = value.get("arguments") {
                let args: Vec<KdlValue> = arguments
                    .as_array()
                    .ok_or_else(|| miette!("`arguments` must be an Array"))?
                    .iter()
                    .filter_map(|v| value_to_kdl(v.to_owned()).ok())
                    .collect();

                for arg in args {
                    node.push(KdlEntry::new(arg));
                }
            };

            if let Some(properties) = value.get("properties") {
                let properties: Vec<(NodeKey, KdlEntry)> = properties
                    .as_object()
                    .ok_or_else(|| miette!("`properties` must be an Object"))?
                    .iter()
                    .filter_map(|(key, value)| match value_to_kdl(value.to_owned()) {
                        Ok(val) => Some((key.to_owned().into(), KdlEntry::new(val))),
                        Err(_) => None,
                    })
                    .collect();

                for (key, value) in properties {
                    node.insert(key, value);
                }
            };

            if let Some(children) = value.get("children") {
                node.set_children(json_to_kdl(children.to_owned())?);
            };

            Ok(node)
        })
        .collect();

    let mut document = KdlDocument::new();

    for node in nodes {
        document.nodes_mut().push(node?);
    }

    Ok(document)
}

/// Try converting a JSON Value into a KDL Value
fn value_to_kdl(value: Value) -> Result<KdlValue> {
    match value {
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
    }
}

#[test]
fn test_conversion() -> Result<()> {
    let input = serde_json::json!(
    {
      "bees": {
        "arguments": [
          true,
          42,
          3.1415,
          null,
          "how many eggs are you currently holding?"
        ],
        "properties": {
          "how many": "uhhh like 40?",
          "state?": "quite upset"
        }
      },
      "lemon": {
        "children": {
          "child": {
            "properties": {
              "age": 3
            }
          },
          "child-eater": {
            "arguments": [
              ":^)"
            ]
          }
        }
      },
      "ohno": {}
    });

    assert_eq!(
        json_to_kdl(input)?.to_string(),
        "bees true 42 3.1415 null \"how many eggs are you currently holding?\" \"how many\"=\"uhhh like 40?\" state?=\"quite upset\"\nlemon {\n    child age=3\n    child-eater \":^)\"\n}\nohno\n"
    );

    Ok(())
}

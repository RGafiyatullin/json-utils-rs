use std::collections::HashSet;

use super::*;

#[test]
fn test_serialize_schema_node_without_required() {
    let schema_all_optional_str = r#"
        {
            "type": "object",
            "properties": {
                "opt_one": {
                    "type": "integer"
                },
                "opt_two": {
                    "type": "integer"
                }
            }
        }
    "#;

    let schema_all_optional: SchemaNode = serde_json::from_str(schema_all_optional_str).unwrap();
    assert!(
        if let SchemaNode::ValidNode(ValidNode::ObjectNode(ref object_node)) = schema_all_optional {
            assert_eq!(object_node.required, HashSet::new());
            true
        } else {
            false
        }
    );
}

#[test]
fn test_serialize_schema_node_into_invalid_node() {
    let invalid_schema_str = r#"
        {
            "type": "object",
            "required": []
        }
    "#;

    let invalid_schema_str: SchemaNode = serde_json::from_str(invalid_schema_str).unwrap();
    assert!(
        if let SchemaNode::InvalidNode(ref _invalid_node) = invalid_schema_str {
            true
        } else {
            false
        }
    );
}

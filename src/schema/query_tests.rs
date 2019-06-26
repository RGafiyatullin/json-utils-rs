use crate::query::Query;

use super::*;

#[test]
fn test_lookup_01() {
    let schema = test_schema();

    assert_eq!(schema.lookup("a_null/nothing"), None);
    assert_eq!(
        schema.lookup("a_null").map(|ref qn| qn.schema),
        Some(&SchemaNode::null().into())
    );
    assert_eq!(schema.lookup("").map(|ref qn| qn.schema), Some(&schema));
    assert_eq!(
        schema.lookup("an_object/a_string").map(|ref qn| qn.schema),
        Some(&SchemaNode::string().into())
    );
}

fn test_schema() -> SchemaNode {
    SchemaNode::object()
        .add_property("a_null", SchemaNode::null())
        .add_property("a_string", SchemaNode::string())
        .add_property("a_boolean", SchemaNode::boolean())
        .add_property("a_number", SchemaNode::number())
        .add_property("an_integer", SchemaNode::integer())
        .add_property("an_array", SchemaNode::array(SchemaNode::integer()))
        .add_property(
            "an_object",
            SchemaNode::object()
                .add_property("a_null", SchemaNode::null())
                .add_property("a_string", SchemaNode::string())
                .add_property("a_boolean", SchemaNode::boolean())
                .add_property("a_number", SchemaNode::number())
                .add_property("an_integer", SchemaNode::integer())
                .add_property("an_array", SchemaNode::array(SchemaNode::string()))
                .add_required("a_boolean")
                .add_required("a_number"),
        )
        .add_required("a_string")
        .add_required("an_object")
        .into()
}

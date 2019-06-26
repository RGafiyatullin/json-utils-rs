use super::*;

macro_rules! valid_node_from {
    ($e: ident) => {
        impl From<$e> for ValidNode {
            fn from(inner: $e) -> Self {
                ValidNode::$e(inner)
            }
        }
    };
}

macro_rules! schema_node_via_valid_node_from {
    ($e: ident) => {
        impl From<$e> for SchemaNode {
            fn from(inner: $e) -> Self {
                SchemaNode::ValidNode(inner.into())
            }
        }
    };
}

impl From<ValidNode> for SchemaNode {
    fn from(inner: ValidNode) -> Self {
        SchemaNode::ValidNode(inner)
    }
}

valid_node_from!(NullNode);
valid_node_from!(BooleanNode);
valid_node_from!(StringNode);
valid_node_from!(IntegerNode);
valid_node_from!(NumberNode);
valid_node_from!(ArrayNode);
valid_node_from!(ObjectNode);

schema_node_via_valid_node_from!(NullNode);
schema_node_via_valid_node_from!(BooleanNode);
schema_node_via_valid_node_from!(StringNode);
schema_node_via_valid_node_from!(IntegerNode);
schema_node_via_valid_node_from!(NumberNode);
schema_node_via_valid_node_from!(ArrayNode);
schema_node_via_valid_node_from!(ObjectNode);

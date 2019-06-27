use super::*;

macro_rules! valid_node_into_result {
    ($e: ident) => {
        enum_variant_into_result!(ValidNode, $e, $e);
    };
}

macro_rules! node_into_result_via_valid_node {
    ($e: ident) => {
        impl Into<Result<$e, SchemaNode>> for SchemaNode {
            fn into(self) -> Result<$e, SchemaNode> {
                match self {
                    SchemaNode::ValidNode(valid_node) => {
                        Into::<Result<$e, ValidNode>>::into(valid_node)
                            .map_err(SchemaNode::ValidNode)
                    }

                    other => Err(other),
                }
            }
        }
    };
}

valid_node_into_result!(NullNode);
valid_node_into_result!(BooleanNode);
valid_node_into_result!(StringNode);
valid_node_into_result!(IntegerNode);
valid_node_into_result!(NumberNode);
valid_node_into_result!(ArrayNode);
valid_node_into_result!(ObjectNode);

node_into_result_via_valid_node!(NullNode);
node_into_result_via_valid_node!(BooleanNode);
node_into_result_via_valid_node!(StringNode);
node_into_result_via_valid_node!(IntegerNode);
node_into_result_via_valid_node!(NumberNode);
node_into_result_via_valid_node!(ArrayNode);
node_into_result_via_valid_node!(ObjectNode);

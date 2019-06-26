use super::*;

impl SchemaNode {
    pub fn null() -> NullNode {
        NullNode::default()
    }
    pub fn string() -> StringNode {
        StringNode::default()
    }
    pub fn integer() -> IntegerNode {
        IntegerNode::default()
    }
    pub fn number() -> NumberNode {
        NumberNode::default()
    }
    pub fn boolean() -> BooleanNode {
        BooleanNode::default()
    }
    pub fn object() -> ObjectNode {
        ObjectNode::default()
    }
    pub fn array<S: Into<SchemaNode>>(items: S) -> ArrayNode {
        ArrayNode::new(items)
    }
}

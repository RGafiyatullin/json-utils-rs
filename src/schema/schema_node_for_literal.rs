use crate::json::JsValue;

use super::SchemaNode;

impl SchemaNode {
    pub fn for_literal(literal_value: &JsValue) -> SchemaNode {
        match *literal_value {
            JsValue::Null => SchemaNode::null().into(),
            JsValue::Bool(_) => SchemaNode::boolean().into(),
            JsValue::Number(_) => SchemaNode::number().into(),
            JsValue::String(_) => SchemaNode::string().into(),
            JsValue::Array(_) => SchemaNode::array(SchemaNode::any()).into(),
            JsValue::Object(_) => SchemaNode::object().into(),
        }
    }
}

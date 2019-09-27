use crate::json::JsValue;

use super::SchemaNode;

impl SchemaNode {
    pub fn for_literal(literal_value: &JsValue) -> SchemaNode {
        match *literal_value {
            JsValue::Null => SchemaNode::null().into(),
            JsValue::Bool(_) => SchemaNode::boolean().into(),
            JsValue::Number(ref value) => 
                if value.is_i64() {
                    SchemaNode::integer().into()
                } else {
                    SchemaNode::number().into()
                },
            JsValue::String(_) => SchemaNode::string().into(),
            JsValue::Array(_) => SchemaNode::array(SchemaNode::any()).into(),
            JsValue::Object(ref props) => {
                props.into_iter().fold(
                    SchemaNode::object(),
                    |acc, (key, value)| {
                        acc.add_property(key, Self::for_literal(value)).add_required(key)
                    }
                ).into()
            },
        }
    }
}

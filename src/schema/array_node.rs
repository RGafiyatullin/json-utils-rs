use std::collections::HashMap;

use crate::json::JsValue;

use super::SchemaNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayNode {
    items: Box<SchemaNode>,

    #[serde(flatten)]
    extra: HashMap<String, JsValue>,
}

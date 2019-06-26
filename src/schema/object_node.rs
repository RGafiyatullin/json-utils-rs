use std::collections::HashMap;

use crate::json::JsValue;

use super::SchemaNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectNode {
    properties: HashMap<String, SchemaNode>,

    #[serde(flatten)]
    extra: HashMap<String, JsValue>,
}

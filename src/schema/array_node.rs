use std::collections::HashMap;

use crate::json::JsValue;

use super::SchemaNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayNode {
    pub items: Box<SchemaNode>,

    #[serde(flatten)]
    pub extra: HashMap<String, JsValue>,
}

impl ArrayNode {
    pub fn new<S: Into<SchemaNode>>(items: S) -> Self {
        let items = Box::new(items.into());
        Self {
            items,
            extra: hashmap! {},
        }
    }
}

use std::collections::HashMap;

use crate::json::JsValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberNode {
    #[serde(flatten)]
    extra: HashMap<String, JsValue>,
}

use std::collections::HashMap;

use crate::json::JsValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringNode {
    #[serde(flatten)]
    extra: HashMap<String, JsValue>,
}

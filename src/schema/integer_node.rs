use std::collections::HashMap;

use crate::json::JsValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerNode {
    #[serde(flatten)]
    extra: HashMap<String, JsValue>,
}

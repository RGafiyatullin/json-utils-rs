use std::collections::HashMap;

use crate::json::JsValue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BooleanNode {
    #[serde(flatten)]
    pub extra: HashMap<String, JsValue>,
}

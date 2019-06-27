use std::collections::HashMap;

use crate::json::JsValue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringNode {
    #[serde(flatten)]
    pub extra: HashMap<String, JsValue>,
}

impl_extra_props!(StringNode, extra);

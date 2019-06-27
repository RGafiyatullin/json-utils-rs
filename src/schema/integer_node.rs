use std::collections::HashMap;

use crate::json::JsValue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerNode {
    #[serde(flatten)]
    pub extra: HashMap<String, JsValue>,
}

impl_extra_props!(IntegerNode, extra);

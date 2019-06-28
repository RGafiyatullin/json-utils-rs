use std::collections::HashMap;

use crate::json::JsValue;

use super::SchemaNode;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnyNode {
    #[serde(flatten)]
    pub extra: HashMap<String, JsValue>,
}

impl_extra_props!(AnyNode, extra);

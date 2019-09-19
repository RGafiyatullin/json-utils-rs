use std::collections::HashMap;
use std::collections::HashSet;

use crate::json::JsValue;

use super::SchemaNode;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectNode {
    pub properties: HashMap<String, SchemaNode>,

    #[serde(default = "HashSet::new", skip_serializing_if = "HashSet::is_empty")]
    pub required: HashSet<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(flatten)]
    pub extra: HashMap<String, JsValue>,
}

impl ObjectNode {
    pub fn with_properties(self, properties: HashMap<String, SchemaNode>) -> Self {
        Self { properties, ..self }
    }
    pub fn add_property<S: Into<SchemaNode>>(mut self, key: &str, schema: S) -> Self {
        self.properties.insert(key.to_owned(), schema.into());
        self
    }
    pub fn rm_property(mut self, key: &str) -> Self {
        self.properties.remove(key);
        self
    }

    pub fn with_required(self, required: HashSet<String>) -> Self {
        Self { required, ..self }
    }
    pub fn add_required(mut self, key: &str) -> Self {
        self.required.insert(key.to_owned());
        self
    }
    pub fn rm_required(mut self, key: &str) -> Self {
        self.required.remove(key);
        self
    }
}

impl_extra_props!(ObjectNode, extra);

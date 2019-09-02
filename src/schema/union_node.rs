use super::SchemaNode;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnionNode {
    AnyOfNode {
        #[serde(rename = "anyOf")]
        variants: Vec<SchemaNode>,
    },

    OneOfNode {
        #[serde(rename = "oneOf")]
        variants: Vec<SchemaNode>,
    }
}

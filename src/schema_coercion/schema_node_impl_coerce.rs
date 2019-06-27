
use crate::schema::SchemaNode;
use super::Coercion;
use super::CoercionError;

impl SchemaNode {
    pub fn coerce(&self, target: &SchemaNode) -> Result<Coercion, CoercionError> {
        Ok(Coercion::Identity) // XXX: this is a stub for now
    }
}

use super::Coercion;
use super::CoercionError;
use crate::schema::SchemaNode;

impl SchemaNode {
    pub fn coerce(&self, _target: &SchemaNode) -> Result<Coercion, CoercionError> {
        Ok(Coercion::Identity) // XXX: this is a stub for now
    }
}


use crate::schema::SchemaNode;

#[derive(Debug, Fail)]
pub enum CoercionError {
    #[fail(display = "CoercionError::IncompatibleSchemas: {:?} -> {:?}", source, target)]
    IncompatibleSchemas { source: SchemaNode, target: SchemaNode },
}

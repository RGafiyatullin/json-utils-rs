use super::SchemaNode;

use serde_json::Error as SerializeError;
use valico::json_schema::Scope;
use valico::json_schema::SchemaError;

use super::SchemaCompiled;

#[derive(Debug, Fail)]
pub enum CompileError {
    #[fail(display = "CompileError::SerializeError: {:?}", _0)]
    SerializeError(#[cause] SerializeError),

    #[fail(display = "CompileError::SchemaError: {:?}", _0)]
    SchemaError(SchemaError),
}

enum_variant_from!(CompileError, SerializeError, SerializeError);
enum_variant_from!(CompileError, SchemaError, SchemaError);

impl SchemaNode {
    pub fn into_compiled(self) -> Result<SchemaCompiled, CompileError> {
        let json = serde_json::to_value(self)?;

        let schema = {
            let mut scope = Scope::new();
            scope.compile_with_id(SchemaCompiled::root_url(), json, /*ban_unknown: */ false)
                .map(|_url| scope)
                .map(SchemaCompiled)
        }?;

        Ok(schema)
    }
}

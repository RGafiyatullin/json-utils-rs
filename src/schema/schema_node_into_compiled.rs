use super::SchemaNode;

use serde_json::Error as SerializeError;
use valico::json_schema::schema::CompilationSettings;
use valico::json_schema::Schema;
use valico::json_schema::SchemaError;

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
    pub fn into_compiled(self) -> Result<Schema, CompileError> {
        let json = serde_json::to_value(self)?;
        let schema = {
            let keywords = valico::json_schema::keywords::default();
            valico::json_schema::schema::compile(
                json,
                None,
                CompilationSettings::new(&keywords, false),
            )
        }?;

        Ok(schema)
    }
}

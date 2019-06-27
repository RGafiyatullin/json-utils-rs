use crate::json::JsValue;

use valico::json_schema;
use valico::json_schema::schema::Schema;
use valico::json_schema::schema::ScopedSchema;

#[derive(Debug, Fail)]
#[fail(display = "ValidationError: {}", _0)]
pub struct ValidationError(String);

pub trait ValidateJsValue {
    fn validate_js_value(&self, js_value: &JsValue) -> Result<(), ValidationError>;
}

impl ValidateJsValue for Schema {
    fn validate_js_value(&self, js_value: &JsValue) -> Result<(), ValidationError> {
        let validation_scope = json_schema::Scope::new();
        let scoped_schema = ScopedSchema::new(&validation_scope, self);
        let validation_state = scoped_schema.validate(js_value);
        if validation_state.is_valid() {
            Ok(())
        } else {
            Err(ValidationError(format!("{:?}", validation_state)))
        }
    }
}

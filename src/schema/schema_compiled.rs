
lazy_static! {
    static ref ROOT_URL: url::Url = {
        use std::str::FromStr;
        url::Url::from_str("json-schema://root").expect("failed to create schema root-url")
    };
}

use valico::json_schema::Scope;

use crate::json::JsValue;

#[derive(Debug, Fail)]
#[fail(display = "ValidationError: {:?}", _0)]
pub struct ValidationError(pub JsValue);

#[derive(Debug)]
pub struct SchemaCompiled(pub Scope);

impl SchemaCompiled {
    pub fn root_url() -> &'static url::Url {
        &*ROOT_URL
    }
    pub fn validate(&self, value: &JsValue) -> Result<(), ValidationError> {
        let schema = self.0.resolve(Self::root_url()).expect("Failed to resolve schema root-url");
        let validation_state = schema.validate(value);
        if validation_state.is_valid() {
            Ok(())
        } else {
            let validation_error_json = serde_json::to_value(validation_state).unwrap();
            let validation_error = ValidationError(validation_error_json);
            Err(validation_error)
        }
    }
}

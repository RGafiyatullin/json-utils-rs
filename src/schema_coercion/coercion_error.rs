use std::collections::HashSet;

use crate::json::JsValue;
use crate::schema::SchemaNode;

use super::Coercion;

#[derive(Debug, Fail)]
pub enum CoercionError {
    #[fail(
        display = "CoercionError::IncompatibleSchemas: {:?} -> {:?}",
        source, target
    )]
    IncompatibleSchemas {
        source: SchemaNode,
        target: SchemaNode,
    },

    #[fail(display = "CoercionError::UnexpectedInput: {:?} {:?}", input, coercion)]
    UnexpectedInput { input: JsValue, coercion: Coercion },

    #[fail(display = "CoercionError::ObjectFieldsMissing: {:?}", _0)]
    ObjectFieldsMissing(HashSet<String>),

    #[fail(display = "CoercionError::JsNumberError")]
    JsNumberError,
}

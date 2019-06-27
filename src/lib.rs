#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate failure;

#[macro_use]
mod macros;

pub mod json;
pub mod query;
pub mod schema;

pub mod prelude {
    pub use crate::json::JsValue;
    pub use crate::schema::SchemaNode;
    pub use crate::query::Query;

    pub use crate::schema::CompileError;
    pub use crate::schema::ValidateJsValue;
    pub use crate::schema::ValidationError;
}

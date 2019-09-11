#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod json;
pub mod query;
pub mod schema;
pub mod schema_coercion;

pub mod prelude {
    pub use crate::query::Query;

    pub use crate::json::JsValue;

    pub use crate::schema::QueryNode;
    pub use crate::schema::SchemaNode;
    pub use crate::schema::SchemaCompiled;

    pub use crate::schema::ExtraProps;
    pub use crate::schema::SchemaDescription;

    pub use crate::schema::CompileError;
    pub use crate::schema::ValidationError;

    pub use crate::schema_coercion::Coercion;
    pub use crate::schema_coercion::CoercionError;
}

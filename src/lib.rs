#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate maplit;

pub mod json;
pub mod query;
pub mod schema;

pub mod prelude {
    pub use crate::json::JsValue;
    pub use crate::schema::SchemaNode;
    pub use crate::query::Query;
}

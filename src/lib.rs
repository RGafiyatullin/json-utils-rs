#[cfg(test)]
#[macro_use]
extern crate serde_json;

pub mod json;
pub mod query;
pub mod schema;

pub mod prelude {
    pub use crate::json::JsValue;
    pub use crate::query::JsValueQuery;
}

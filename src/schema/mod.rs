#[macro_use]
mod extra_props;

mod query_for_schema_node;
mod schema_node;
mod schema_node_constructors;
mod schema_node_for_literal;
mod schema_node_from;
mod schema_node_into;
mod schema_node_into_compiled;
mod schema_compiled;

mod schema_description;

mod any_node;
mod array_node;
mod boolean_node;
mod integer_node;
mod null_node;
mod number_node;
mod object_node;
mod string_node;

pub use extra_props::ExtraProps;
pub use schema_description::SchemaDescription;

pub use query_for_schema_node::QueryNode;

pub use schema_node::InvalidNode;
pub use schema_node::SchemaNode;
pub use schema_node::ValidNode;

pub use schema_compiled::SchemaCompiled;
pub use schema_compiled::ValidationError;

pub use any_node::AnyNode;
pub use array_node::ArrayNode;
pub use boolean_node::BooleanNode;
pub use integer_node::IntegerNode;
pub use null_node::NullNode;
pub use number_node::NumberNode;
pub use object_node::ObjectNode;
pub use string_node::StringNode;

pub use schema_node_into_compiled::CompileError;


#[cfg(test)]
mod query_tests;

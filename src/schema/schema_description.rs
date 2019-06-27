
use crate::json::JsValue;

use super::ExtraProps;

const KEY: &'static str = "description";

pub trait SchemaDescription {
    fn description(&self) -> Option<&str>;
    fn with_description(self, description: &str) -> Self;
}

impl<S: ExtraProps> SchemaDescription for S {
    fn description(&self) -> Option<&str> {
        self.extra_props().get(KEY).and_then(JsValue::as_str)
    }
    fn with_description(mut self, description: &str) -> Self {
        let extra_props = self.extra_props_mut();
        let _ = extra_props.insert(KEY.to_owned(), JsValue::String(description.to_owned()));
        self
    }
}

use std::collections::HashMap;

use crate::json::JsValue;

pub trait ExtraProps {
    fn extra_props(&self) -> &HashMap<String, JsValue>;
    fn extra_props_mut(&mut self) -> &mut HashMap<String, JsValue>;
    fn with_extra_props(self, extra_props: HashMap<String, JsValue>) -> Self;
}

macro_rules! impl_extra_props {
    ($type: ident, $field: ident) => {
        impl $crate::schema::ExtraProps for $type {
            fn extra_props(&self) -> &HashMap<String, JsValue> {
                &self.$field
            }

            fn extra_props_mut(&mut self) -> &mut HashMap<String, JsValue> {
                &mut self.$field
            }

            fn with_extra_props(self, extra_props: HashMap<String, JsValue>) -> Self {
                Self {
                    $field: extra_props,
                    .. self
                }
            }
        }
    };
}

use crate::json::JsValue;

use super::JsPath;
use super::JsPathComponent;
use super::JsValueQuery;

impl JsValueQuery for JsValue {
    fn lookup<'v, 'p, P>(&'v self, path: P) -> Option<&'v Self>
    where
        P: JsPath<'p>,
    {
        lookup(self, path.path())
    }
}

fn lookup<'v, 'p, P: JsPathComponent<'p>, I: Iterator<Item = P>>(
    v: &'v JsValue,
    mut components: I,
) -> Option<&'v JsValue> {
    match components.next() {
        None => Some(v),
        Some(component) => match *v {
            JsValue::Object(ref fields) => fields
                .get(component.as_str_slice())
                .and_then(move |child| lookup(child, components)),
            _ => None,
        },
    }
}

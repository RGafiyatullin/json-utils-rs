use crate::json::JsValue;

use crate::query::Path;
use crate::query::PathComponent;
use crate::query::Query;

impl Query for JsValue {
    fn lookup<'v, 'p, P>(&'v self, path: P) -> Option<&'v Self>
    where
        P: Path<'p>,
    {
        lookup(self, path.path())
    }

    fn take<'p, P>(self, path: P) -> (Option<Self>, Option<Self>)
    where
        P: Path<'p>,
    {
        take(self, path.path())
    }

    fn insert<'p, P>(&mut self, path: P, insertee: Self) -> Option<Self>
    where
        P: Path<'p>,
    {
        insert(self, path.path(), insertee)
    }
}

fn lookup<'v, 'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
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

fn take<'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: JsValue,
    mut components: I,
) -> (Option<JsValue>, Option<JsValue>) {
    match components.next() {
        None => (None, Some(v)),
        Some(component) => match v {
            JsValue::Object(mut fields) => {
                let child_key = component.as_str_slice();
                match fields.remove(child_key) {
                    None => (Some(JsValue::Object(fields)), None),
                    Some(child) => {
                        let (child_opt, taken_opt) = take(child, components);
                        if let Some(child) = child_opt {
                            fields.insert(child_key.to_owned(), child);
                        };
                        (Some(JsValue::Object(fields)), taken_opt)
                    }
                }
            }
            as_is => (Some(as_is), None),
        },
    }
}

fn insert<'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: &mut JsValue,
    mut components: I,
    insertee: JsValue,
) -> Option<JsValue> {
    match components.next() {
        None => {
            *v = insertee;
            None
        }
        Some(component) => match *v {
            JsValue::Object(ref mut fields) => {
                let child_key = component.as_str_slice();
                match fields.get_mut(child_key) {
                    Some(ref mut child) => insert(child, components, insertee),
                    None => {
                        let mut child = json!({});

                        let rejected_opt = insert(&mut child, components, insertee);
                        assert!(rejected_opt.is_none());

                        fields.insert(child_key.to_owned(), child);
                        None
                    }
                }
            }
            _ => Some(insertee),
        },
    }
}

use crate::json::JsValue;

use crate::query::Path;
use crate::query::PathComponent;
use crate::query::Query;

impl<'v> Query<'v> for JsValue {
    type Item = Self;
    type ItemRef = &'v Self::Item;

    fn lookup<'p, P>(&'v self, path: P) -> Option<Self::ItemRef>
    where
        P: Path<'p>,
    {
        lookup(self, path.path())
    }

    fn take<'p, P>(self, path: P) -> (Option<Self>, Option<Self::Item>)
    where
        P: Path<'p>,
    {
        take(self, path.path())
    }

    fn insert<'p, P>(&mut self, path: P, insertee: Self::Item) -> Result<(), Self::Item>
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
    if let Some(component) = components.next() {
        match *v {
            JsValue::Object(ref fields) => fields
                .get(component.as_str_slice())
                .and_then(move |child| lookup(child, components)),
            _ => None,
        }
    } else {
        Some(v)
    }
}

fn take<'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: JsValue,
    mut components: I,
) -> (Option<JsValue>, Option<JsValue>) {
    if let Some(component) = components.next() {
        match v {
            JsValue::Object(mut fields) => {
                let child_key = component.as_str_slice();
                if let Some(child) = fields.remove(child_key) {
                    let (child_opt, taken_opt) = take(child, components);
                    
                    if let Some(child) = child_opt {
                        fields.insert(child_key.to_owned(), child);
                    };

                    if fields.is_empty() {
                        (None, taken_opt)
                    } else {
                        (Some(JsValue::Object(fields)), taken_opt)
                    }
                } else {
                    (Some(JsValue::Object(fields)), None)
                }
            }
            as_is => (Some(as_is), None),
        }
    } else {
        (None, Some(v))
    }
}

fn insert<'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: &mut JsValue,
    mut components: I,
    insertee: JsValue,
) -> Result<(), JsValue> {
    if let Some(component) = components.next() {
        match *v {
            JsValue::Object(ref mut fields) => {
                let child_key = component.as_str_slice();

                if let Some(ref mut child) = fields.get_mut(child_key) {
                    insert(child, components, insertee)
                } else {
                    let mut child = json!({});

                    let () = insert(&mut child, components, insertee)
                        .expect("Failed to insert into a newly created ObjectNode");

                    fields.insert(child_key.to_owned(), child);
                    Ok(())
                }
            }
            _ => Err(insertee),
        }
    } else {
        *v = insertee;
        Ok(())
    }
}

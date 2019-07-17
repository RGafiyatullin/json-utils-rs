use std::fmt;

use crate::query::Path;
use crate::query::PathComponent;
use crate::query::Query;

use super::SchemaNode;
use super::ValidNode;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryNode<T: fmt::Debug + Clone + PartialEq> {
    pub schema: T,
    pub is_required: bool,
}

impl<'v> Query<'v> for SchemaNode {
    type Item = QueryNode<SchemaNode>;
    type ItemRef = QueryNode<&'v SchemaNode>;

    fn lookup<'p, P>(&'v self, path: P) -> Option<Self::ItemRef>
    where
        P: Path<'p>,
    {
        lookup(self, true, path.path())
    }

    fn take<'p, P>(self, path: P) -> (Option<Self>, Option<Self::Item>)
    where
        P: Path<'p>,
    {
        take(self, true, path.path())
    }

    fn insert<'p, P>(&mut self, path: P, insertee: Self::Item) -> Result<(), Self::Item>
    where
        P: Path<'p>,
    {
        insert(self, path.path(), insertee)
    }
}

fn lookup<'v, 'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: &'v SchemaNode,
    is_required: bool,
    mut components: I,
) -> Option<QueryNode<&'v SchemaNode>> {
    if let Some(component) = components.next() {
        if let SchemaNode::ValidNode(ValidNode::ObjectNode(ref object_node)) = *v {
            let child_key = component.as_str_slice();
            let child_is_required = object_node.required.contains(child_key);
            object_node
                .properties
                .get(child_key)
                .and_then(move |ref child| lookup(child, child_is_required, components))
        } else {
            None
        }
    } else {
        let qn = QueryNode {
            is_required,
            schema: v,
        };
        Some(qn)
    }
}

fn take<'v, 'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: SchemaNode,
    is_required: bool,
    mut components: I,
) -> (Option<SchemaNode>, Option<QueryNode<SchemaNode>>) {
    if let Some(component) = components.next() {
        match v {
            SchemaNode::ValidNode(ValidNode::ObjectNode(mut object_node)) => {
                let child_key = component.as_str_slice();
                if let Some(child) = object_node.properties.remove(child_key) {
                    let child_is_required = object_node.required.contains(child_key);
                    let (child_opt, taken_opt) = take(child, child_is_required, components);
                    if let Some(child) = child_opt {
                        object_node.properties.insert(child_key.to_owned(), child);
                    };
                    if object_node.properties.is_empty() {
                        (None, taken_opt)
                    } else {
                        (Some(object_node.into()), taken_opt)
                    }
                } else {
                    (Some(object_node.into()), None)
                }
            }
            as_is => (Some(as_is), None),
        }
    } else {
        let qn = QueryNode {
            is_required,
            schema: v,
        };
        (None, Some(qn))
    }
}

fn insert<'v, 'p, P: PathComponent<'p>, I: Iterator<Item = P>>(
    v: &'v mut SchemaNode,
    mut components: I,
    insertee: QueryNode<SchemaNode>,
) -> Result<(), QueryNode<SchemaNode>> {
    if let Some(component) = components.next() {
        match *v {
            SchemaNode::ValidNode(ValidNode::ObjectNode(ref mut object_node)) => {
                let child_key = component.as_str_slice();

                if let Some(ref mut child) = object_node.properties.get_mut(child_key) {
                    insert(child, components, insertee)
                } else {
                    let mut child = SchemaNode::object().into();
                    let () = insert(&mut child, components, insertee)
                        .expect("Failed to insert into a newly created node");

                    object_node.properties.insert(child_key.to_owned(), child);
                    Ok(())
                }
            }
            _ => Err(insertee),
        }
    } else {
        *v = insertee.schema;
        Ok(())
    }
}

use std::collections::HashSet;

use crate::json::JsValue;
use crate::schema::*;

use super::Coercion;
use super::CoercionError;

type CoercionResult = Result<Coercion, CoercionError>;

impl SchemaNode {
    pub fn coerce(&self, target: &SchemaNode) -> CoercionResult {
        match (self, target) {
            (SchemaNode::ValidNode(ref source), SchemaNode::ValidNode(ref target)) => {
                coerce_valid_nodes(source, target)
            }

            (source, target) => Err(CoercionError::IncompatibleSchemas {
                source: source.clone(),
                target: target.clone(),
            }),
        }
    }
}

fn coerce_valid_nodes(source: &ValidNode, target: &ValidNode) -> CoercionResult {
    match (source, target) {
        (source, ValidNode::AnyNode(any_node)) => coerce_into_any_node(source, any_node),

        (source, ValidNode::NullNode(null_node)) => coerce_into_null_node(source, null_node),

        (source, ValidNode::BooleanNode(bool_node)) => coerce_into_bool_node(source, bool_node),

        (source, ValidNode::IntegerNode(integer_node)) => {
            coerce_into_integer_node(source, integer_node)
        }

        (source, ValidNode::NumberNode(number_node)) => {
            coerce_into_number_node(source, number_node)
        }

        (source, ValidNode::StringNode(string_node)) => {
            coerce_into_string_node(source, string_node)
        }

        (source, ValidNode::ArrayNode(array_node)) => coerce_into_array_node(source, array_node),

        (source, ValidNode::ObjectNode(object_node)) => {
            coerce_into_object_node(source, object_node)
        }
    }
}

fn coerce_into_object_node(source: &ValidNode, object_node: &ObjectNode) -> CoercionResult {
    match *source {
        ValidNode::ObjectNode(ObjectNode {
            properties: ref source_properties,
            required: ref source_required,
            ..
        }) => {
            let &ObjectNode {
                properties: ref target_properties,
                required: ref target_required,
                ..
            } = object_node;

            let props_missing = target_required - source_required;
            if !props_missing.is_empty() {
                Err(CoercionError::ObjectFieldsMissing(props_missing))?;
            }

            let mut source_prop_names: HashSet<&String> = source_properties.keys().collect();
            let mut prop_coercions: Vec<(String, Coercion)> = Vec::new();

            for (target_prop_name, target_prop_schema) in target_properties {
                if let Some(source_prop_schema) = source_properties.get(target_prop_name) {
                    let prop_coercion = source_prop_schema.coerce(target_prop_schema)?;
                    let pair = (target_prop_name.to_owned(), prop_coercion);
                    prop_coercions.push(pair);
                    source_prop_names.remove(target_prop_name);
                }
            }

            let no_source_props_left_unused = source_prop_names.is_empty();
            let only_identity_coercions_for_props = prop_coercions
                .iter()
                .all(|(_, ref c)| *c == Coercion::Identity);

            if no_source_props_left_unused && only_identity_coercions_for_props {
                ok_identity()
            } else {
                ok_object(prop_coercions)
            }
        }

        ref source => err_incompatible(source, object_node),
    }
}

fn coerce_into_array_node(source: &ValidNode, array_node: &ArrayNode) -> CoercionResult {
    let &ArrayNode {
        items: ref target_items,
        ..
    } = array_node;

    match *source {
        ValidNode::ArrayNode(ref source_array_node) => {
            let &ArrayNode {
                items: ref source_items,
                ..
            } = source_array_node;

            match source_items.coerce(target_items)? {
                Coercion::Identity => ok_identity(),

                coercion => ok_array(coercion),
            }
        }
        ref source => err_incompatible(source, array_node),
    }
}

fn coerce_into_string_node(source: &ValidNode, string_node: &StringNode) -> CoercionResult {
    match *source {
        ValidNode::StringNode(_) => ok_identity(),
        ValidNode::NumberNode(_) => ok_number_to_string(),
        ValidNode::IntegerNode(_) => ok_number_to_string(),
        ref source => err_incompatible(source, string_node),
    }
}

fn coerce_into_number_node(source: &ValidNode, number_node: &NumberNode) -> CoercionResult {
    match *source {
        ValidNode::NumberNode(_) => ok_identity(),
        ValidNode::IntegerNode(_) => ok_identity(),
        ref source => err_incompatible(source, number_node),
    }
}

fn coerce_into_integer_node(source: &ValidNode, integer_node: &IntegerNode) -> CoercionResult {
    match *source {
        ValidNode::IntegerNode(_) => ok_identity(), // TODO: check ranges
        ref source => err_incompatible(source, integer_node),
    }
}

fn coerce_into_bool_node(source: &ValidNode, bool_node: &BooleanNode) -> CoercionResult {
    match *source {
        ValidNode::BooleanNode(_) => ok_identity(),
        ref source => err_incompatible(source, bool_node),
    }
}

fn coerce_into_null_node(source: &ValidNode, _target: &NullNode) -> CoercionResult {
    match *source {
        ValidNode::NullNode(_) => ok_identity(),
        _ => ok_replace_with_literal(JsValue::Null),
    }
}

fn coerce_into_any_node(_source: &ValidNode, _target: &AnyNode) -> CoercionResult {
    ok_identity()
}

fn ok_identity() -> CoercionResult {
    Ok(Coercion::Identity)
}

fn ok_array(items_coercion: Coercion) -> CoercionResult {
    Ok(Coercion::Array(Box::new(items_coercion)))
}

fn ok_replace_with_literal(literal_value: JsValue) -> CoercionResult {
    Ok(Coercion::ReplaceWithLiteral(literal_value))
}

fn ok_number_to_string() -> CoercionResult {
    Ok(Coercion::NumberToString)
}

fn ok_object<
    I: Iterator<Item = (String, Coercion)>,
    II: IntoIterator<Item = (String, Coercion), IntoIter = I>,
>(
    ii: II,
) -> CoercionResult {
    Ok(Coercion::Object(ii.into_iter().collect()))
}

fn err_incompatible<Source, Target>(source: &Source, target: &Target) -> CoercionResult
where
    Source: Clone + Into<SchemaNode>,
    Target: Clone + Into<SchemaNode>,
{
    Err(CoercionError::IncompatibleSchemas {
        source: source.clone().into(),
        target: target.clone().into(),
    })
}

#[test]
fn basic_coercions() {
    let inputs = basic_inputs();

    for (source, target, coercion_opt) in inputs {
        eprintln!("trying {:?} into {:?}", source, target);
        assert_eq!(source.coerce(&target).ok(), coercion_opt);
    }
}

#[test]
fn array_coercions() {
    let inputs: Vec<(SchemaNode, SchemaNode, Option<Coercion>)> = basic_inputs()
        .into_iter()
        .map(|(source, target, coercion_opt)| {
            (
                SchemaNode::array(source).into(),
                SchemaNode::array(target).into(),
                coercion_opt,
            )
        })
        .collect();

    for (source, target, coercion_opt) in inputs {
        let coercion_opt = coercion_opt.map(|coercion| match coercion {
            Coercion::Identity => Coercion::Identity,
            other => Coercion::Array(Box::new(other)),
        });

        eprintln!("trying {:?} into {:?}", source, target);
        assert_eq!(source.coerce(&target).ok(), coercion_opt);
    }
}

#[test]
fn object_identity_coercion() {
    let inputs: Vec<SchemaNode> = vec![
        SchemaNode::object().into(),
        SchemaNode::object()
            .add_property("an_int", SchemaNode::integer())
            .into(),
        basic_inputs()
            .into_iter()
            .enumerate()
            .fold(SchemaNode::object(), |acc, (idx, (p, _, _))| {
                let prop_name = format!("prop_{}", idx);
                acc.add_property(&prop_name, p).add_required(&prop_name)
            })
            .into(),
    ];

    for schema in inputs {
        assert_eq!(schema.coerce(&schema).ok(), Some(Coercion::Identity));
    }
}

#[test]
fn object_failing_coercion() {
    let inputs: Vec<(SchemaNode, SchemaNode)> = vec![
        (SchemaNode::any().into(), SchemaNode::object().into()),
        (SchemaNode::null().into(), SchemaNode::object().into()),
        (SchemaNode::boolean().into(), SchemaNode::object().into()),
        (SchemaNode::integer().into(), SchemaNode::object().into()),
        (SchemaNode::number().into(), SchemaNode::object().into()),
        (SchemaNode::string().into(), SchemaNode::object().into()),
        (
            SchemaNode::array(SchemaNode::string()).into(),
            SchemaNode::object().into(),
        ),
        (
            SchemaNode::object().into(),
            SchemaNode::object()
                .add_property("a_field", SchemaNode::any())
                .add_required("a_field")
                .into(),
        ),
        (
            SchemaNode::object()
                .add_property("a_field", SchemaNode::any())
                .into(),
            SchemaNode::object()
                .add_property("a_field", SchemaNode::any())
                .add_required("a_field")
                .into(),
        ),
    ];

    for (source, target) in inputs {
        eprintln!("coercing {:?} to {:?}", source, target);
        assert!(source.coerce(&target).is_err())
    }
}

#[test]
fn object_non_trivial_coercion() {
    let inputs: Vec<(SchemaNode, SchemaNode)> = vec![
        (
            SchemaNode::object()
                .add_property("a_bool", SchemaNode::boolean())
                .into(),
            SchemaNode::object().into(),
        ),
        (
            SchemaNode::object()
                .add_property("a_bool", SchemaNode::boolean())
                .into(),
            SchemaNode::object()
                .add_property("a_bool", SchemaNode::boolean())
                .into(),
        ),
        (
            SchemaNode::object()
                .add_property("a_bool", SchemaNode::boolean())
                .add_required("a_bool")
                .into(),
            SchemaNode::object()
                .add_property("a_bool", SchemaNode::boolean())
                .add_required("a_bool")
                .into(),
        ),
        (
            SchemaNode::object()
                .add_property("to_string", SchemaNode::integer())
                .add_required("to_string")
                .into(),
            SchemaNode::object()
                .add_property("to_string", SchemaNode::string())
                .add_required("to_string")
                .into(),
        ),
    ];

    for (source, target) in inputs {
        assert!(source.coerce(&target).is_ok())
    }
}

#[cfg(test)]
fn basic_inputs() -> Vec<(SchemaNode, SchemaNode, Option<Coercion>)> {
    vec![
        (
            SchemaNode::null().into(),
            SchemaNode::null().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::any().into(),
            SchemaNode::any().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::boolean().into(),
            SchemaNode::boolean().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::integer().into(),
            SchemaNode::integer().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::number().into(),
            SchemaNode::number().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::string().into(),
            SchemaNode::string().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::integer().into(),
            SchemaNode::null().into(),
            Some(Coercion::ReplaceWithLiteral(JsValue::Null)),
        ),
        (
            SchemaNode::integer().into(),
            SchemaNode::number().into(),
            Some(Coercion::Identity),
        ),
        (
            SchemaNode::number().into(),
            SchemaNode::integer().into(),
            None,
        ),
    ]
}

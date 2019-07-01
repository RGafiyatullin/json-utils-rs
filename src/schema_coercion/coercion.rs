
use crate::json::JsValue;

use super::CoercionError;

type CoercionResult = Result<JsValue, CoercionError>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Coercion {
    #[serde(rename = "identity")]
    Identity,

    #[serde(rename = "replace_with_literal")]
    ReplaceWithLiteral(JsValue),

    #[serde(rename = "number_to_string")]
    NumberToString,

    #[serde(rename = "array")]
    Array(Box<Coercion>),

    #[serde(rename = "object")]
    Object(Vec<(String, Coercion)>),
}

impl Coercion {
    pub fn coerce(self, value: JsValue) -> CoercionResult {
        match self {
            Coercion::Identity => ok_value(value),
            Coercion::ReplaceWithLiteral(literal_value) => ok_value(literal_value),
            Coercion::NumberToString => number_to_string(self, value),
            Coercion::Array(item_coercion) => array_coercion(item_coercion, value),
            Coercion::Object(prop_coercions) => object_coercion(prop_coercions, value),
        }
    }

    pub fn is_subtyping_only(&self) -> bool {
        match *self {
            Coercion::Identity => true,
            Coercion::NumberToString => false,
            Coercion::ReplaceWithLiteral(_) => false,
            Coercion::Array(ref items) => items.is_subtyping_only(),
            Coercion::Object(ref props) => props.iter().all(|(_, prop)| prop.is_subtyping_only()),
        }
    }
}

fn number_to_string(coercion: Coercion, value: JsValue) -> CoercionResult {
    if let JsValue::Number(js_number) = value {
        js_number
            .as_f64()
            .map(|f| format!("{}", f))
            .map(JsValue::String)
            .ok_or(CoercionError::JsNumberError)
    } else {
        err_unexpectd_input(coercion, value)
    }
}

fn array_coercion(item_coercion: Box<Coercion>, value: JsValue) -> CoercionResult {
    if let JsValue::Array(items) = value {
        let item_coercion = *item_coercion;
        let items = items
            .into_iter()
            .map(move |item| item_coercion.clone().coerce(item))
            .collect::<Result<Vec<_>, _>>()?;
        ok_value(JsValue::Array(items))
    } else {
        err_unexpectd_input(Coercion::Array(item_coercion), value)
    }
}

fn object_coercion(prop_coercions: Vec<(String, Coercion)>, value: JsValue) -> CoercionResult {
    if let JsValue::Object(mut input_props) = value {
        let props_count = prop_coercions.len();
        let mut output_props = Vec::<(String, JsValue)>::with_capacity(props_count);

        for (prop_name, prop_coercion) in prop_coercions {
            let input_prop_value =
                input_props
                    .remove(&prop_name)
                    .ok_or(CoercionError::ObjectFieldsMissing(hashset![
                        prop_name.clone()
                    ]))?;
            let output_prop_value = prop_coercion.coerce(input_prop_value)?;

            output_props.push((prop_name, output_prop_value));
        }

        ok_value(JsValue::Object(output_props.into_iter().collect()))
    } else {
        err_unexpectd_input(Coercion::Object(prop_coercions), value)
    }
}

fn ok_value(js_value: JsValue) -> CoercionResult {
    Ok(js_value)
}

fn err_unexpectd_input(coercion: Coercion, input: JsValue) -> CoercionResult {
    Err(CoercionError::UnexpectedInput { coercion, input })
}

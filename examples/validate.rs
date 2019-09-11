
#[macro_use]
extern crate serde;

use std::error;

use serde_json::Value as JsValue;
use valico::json_schema::Schema as JsonSchema;
use valico::json_schema::SchemaError as JsonSchemaError;
use valico::json_schema::Scope as JsonSchemaScope;
use valico::json_schema::schema::ScopedSchema as JsonSchemaScoped;

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_stdin()?;

    let schema = compile_schema(&input)?;
    println!("Validating against the following schema:\n{:#?}", schema);

    let values = input.values;

    for value in values {
        println!("====\ninput:\n{:#?}", value);
        match validate_against_schema(&schema, &value) {
            Ok(()) => println!("result: OK"),
            Err(err) => println!("error: \n{}", err),
        }
    }

    Ok(())
}

fn read_stdin() -> Result<Input, Failure> {
    serde_json::from_reader::<_, Input>(std::io::stdin())
        .map_err(Failure::StdinSerde)
}

fn compile_schema(input: &Input) -> Result<JsonSchema, Failure> {
    use valico::json_schema::schema::CompilationSettings;

    let schema_json = input.schema.clone();
    let keywords = valico::json_schema::keywords::default();
    valico::json_schema::schema::compile(
                schema_json,
                None,
                CompilationSettings::new(&keywords, false),
            ).map_err(Failure::SchemaCompile)
}

fn validate_against_schema(schema: &JsonSchema, value: &JsValue) -> Result<(), String> {
    let validation_scope = JsonSchemaScope::new();
    let scoped_schema = JsonSchemaScoped::new(&validation_scope, schema);
    let validation_state = scoped_schema.validate(value);
    if validation_state.is_valid() {
        Ok(())
    } else {
        Err(format!("{:#?}", validation_state))
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    schema: JsValue,
    values: Vec<JsValue>,
}

#[derive(Debug)]
enum Failure {
    StdinSerde(serde_json::Error),
    SchemaCompile(JsonSchemaError),
}

#[derive(Debug)]
struct ValidationError(String);


impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Failure {}

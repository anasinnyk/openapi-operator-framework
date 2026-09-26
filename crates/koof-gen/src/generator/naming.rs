use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::Ident;

use crate::generator::errors::GeneratorError;

pub fn type_ident(value: &str) -> Result<Ident, GeneratorError> {
    parse_ident(&value.to_upper_camel_case())
}

pub fn kind_ident(value: &str) -> Result<Ident, GeneratorError> {
    parse_ident(value)
}

pub fn field_ident(value: &str) -> Result<Ident, GeneratorError> {
    let name = value.to_snake_case();

    syn::parse_str::<Ident>(&name)
        .or_else(|_| syn::parse_str::<Ident>(&format!("r#{name}")))
        .map_err(|_| GeneratorError::InvalidIdentifier(value.into()))
}

fn parse_ident(value: &str) -> Result<Ident, GeneratorError> {
    let value = if value
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        format!("Schema{value}")
    } else {
        value.to_owned()
    };

    syn::parse_str(&value).map_err(|_| GeneratorError::InvalidIdentifier(value))
}

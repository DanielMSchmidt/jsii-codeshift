pub mod typescript;

use std::fmt::Display;

use crate::{
    ast::base::{Expression, AST},
    languages::Language,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExpressionGenerateError {
    #[error("Unknown expression: {0}")]
    UnknownExpression(Expression),
}

#[derive(Error, Debug)]
pub enum GenerateError {
    UnsupportedLanguage,
    CouldNotGenerateCodeForExpressions(Vec<ExpressionGenerateError>),
}

impl Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::UnsupportedLanguage => write!(f, "Unsupported language"),
            GenerateError::CouldNotGenerateCodeForExpressions(errors) => {
                write!(
                    f,
                    "Could not generate code for expressions: {}",
                    errors
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

pub type GenerateResult = Result<String, GenerateError>;

pub fn generate(lang: Language, ast: AST) -> GenerateResult {
    match lang {
        Language::Typescript => typescript::generate(ast),
        _ => Err(GenerateError::UnsupportedLanguage),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    type TestResult = Result<(), GenerateError>;

    #[test]
    fn empty_ast() -> TestResult {
        let result = generate(
            Language::Typescript,
            AST {
                expressions: vec![],
            },
        )?;
        assert_eq!(result, "");

        Ok(())
    }
}

use thiserror::Error;

use crate::{ast::ast::AST, languages::Language};

#[derive(Error, Debug)]
pub enum ParseError {}

pub type ParseResult = Result<AST, ParseError>;

pub fn parse<S: Into<String>>(_lang: Language, _content: S) -> ParseResult {
    Ok(AST {
        expressions: vec![],
    })
}

#[cfg(test)]
mod tests {
    use crate::ast::import::{ImportDeclaration, ImportSpecifier};

    use super::*;

    type TestResult = Result<(), ParseError>;

    #[test]
    fn empty_content() -> TestResult {
        let result = parse(Language::Typescript, "")?;
        assert_eq!(result.expressions.len(), 0);

        Ok(())
    }

    #[test]
    fn namespace_import() -> TestResult {
        let expected = ImportDeclaration {
            source: "developers".to_string(),
            specifiers: vec![ImportSpecifier::Namespace("daniel".to_string())],
        };
        let result = parse(
            Language::Typescript,
            "import * as daniel from 'developers';",
        )?;

        assert_eq!(result.expressions.len(), 1);
        assert_eq!(
            format!("{}", result.expressions[0]),
            format!("{}", expected)
        );

        Ok(())
    }
}

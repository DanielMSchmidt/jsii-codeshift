mod typescript;

use crate::{ast::ast::AST, languages::Language};
use thiserror::Error;

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
}

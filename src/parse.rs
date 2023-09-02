use thiserror::Error;

use crate::{ast::ast::AST, languages::Language};

#[derive(Error, Debug)]
pub enum ParseError {}

pub fn parse<S: Into<String>>(_lang: Language, _content: S) -> Result<AST, ParseError> {
    Ok(AST {
        expressions: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_content() -> Result<(), ParseError> {
        let result = parse(Language::Typescript, "")?;
        assert_eq!(result.expressions.len(), 0);

        Ok(())
    }
}

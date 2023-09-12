use crate::ast::base::{Expression, AST};

use super::{ExpressionGenerateError, GenerateError, GenerateResult};

fn generate_code_for_expression(expr: Expression) -> Result<String, ExpressionGenerateError> {
    match expr {
        Expression::Comment(comment) => Ok(format!("// {}", comment)),
        Expression::ImportDeclaration(_import_declaration) => todo!("implement imports"),
        Expression::UnknownExpression(unknown_expression) => Ok(unknown_expression),
    }
}

pub fn generate(AST { expressions }: AST) -> GenerateResult {
    let (code, errors) = expressions
        .into_iter()
        .map(|e| generate_code_for_expression(*e))
        .partition::<Vec<_>, _>(|c| c.is_ok());

    if errors.len() > 0 {
        Err(GenerateError::CouldNotGenerateCodeForExpressions(
            errors.into_iter().map(Result::unwrap_err).collect(),
        ))
    } else {
        Ok(code
            .into_iter()
            .map(|c| c.unwrap())
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::{ast::import::ImportSpecifier, generate::GenerateError};

    use super::*;
    type TestResult = Result<(), GenerateError>;

    #[test]
    fn empty_content() -> TestResult {
        let result = generate(AST {
            expressions: vec![],
        })?;

        assert_eq!(result, "");

        Ok(())
    }

    #[test]
    fn comment() -> TestResult {
        let result = generate(AST {
            expressions: vec![Box::new(Expression::Comment("Hello".to_string()))],
        })?;

        assert_eq!(result, "// Hello");

        Ok(())
    }

    #[test]
    fn import_declaration() -> TestResult {
        let result = generate(AST {
            expressions: vec![
                Box::new(Expression::ImportDeclaration(
                    crate::ast::import::ImportDeclaration {
                        source: "./.gen/item-no-rename".to_string(),
                        specifiers: vec![ImportSpecifier::Item {
                            local: "foo".to_string(),
                            imported: "foo".to_string(),
                        }],
                    },
                )),
                Box::new(Expression::ImportDeclaration(
                    crate::ast::import::ImportDeclaration {
                        source: "./.gen/item-with-rename".to_string(),
                        specifiers: vec![ImportSpecifier::Item {
                            local: "bar".to_string(),
                            imported: "baz".to_string(),
                        }],
                    },
                )),
                Box::new(Expression::ImportDeclaration(
                    crate::ast::import::ImportDeclaration {
                        source: "./.gen/namespace".to_string(),
                        specifiers: vec![ImportSpecifier::Namespace("myLib".to_string())],
                    },
                )),
                Box::new(Expression::ImportDeclaration(
                    crate::ast::import::ImportDeclaration {
                        source: "./.gen/default".to_string(),
                        specifiers: vec![ImportSpecifier::Default("myLibsDefault".to_string())],
                    },
                )),
                Box::new(Expression::ImportDeclaration(
                    crate::ast::import::ImportDeclaration {
                        source: "./.gen/all-of-the-above".to_string(),
                        specifiers: vec![
                            ImportSpecifier::Item {
                                local: "foo".to_string(),
                                imported: "foo".to_string(),
                            },
                            ImportSpecifier::Item {
                                local: "bar".to_string(),
                                imported: "baz".to_string(),
                            },
                            ImportSpecifier::Namespace("myLib".to_string()),
                            ImportSpecifier::Default("myLibsDefault".to_string()),
                        ],
                    },
                )),
            ],
        })?;

        assert_eq!(result, "TODO: imports");

        Ok(())
    }
}

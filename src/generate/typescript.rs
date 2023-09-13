use crate::ast::{
    base::{Expression, AST},
    import::ImportDeclaration,
};

use super::{ExpressionGenerateError, GenerateError, GenerateResult};

fn generate_code_for_expression(expr: Expression) -> Result<String, ExpressionGenerateError> {
    match expr {
        Expression::Comment(comment) => Ok(format!("// {}", comment)),
        Expression::ImportDeclaration(ImportDeclaration { specifiers, source }) => {
            let item_specifiers = specifiers
                .iter()
                .filter_map(|spec| match spec {
                    crate::ast::import::ImportSpecifier::Item { imported, local } => {
                        if imported == local {
                            Some(format!("{}", imported))
                        } else {
                            Some(format!("{} as {}", imported, local))
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<String>>()
                .join(", ");

            let other_specifier_code = specifiers
                .into_iter()
                .filter_map(|spec| match spec {
                    crate::ast::import::ImportSpecifier::Namespace(name) => {
                        Some(format!("* as {}", name))
                    }
                    crate::ast::import::ImportSpecifier::Default(name) => Some(format!("{}", name)),
                    _ => None, // we ignore these because we handle them above
                })
                .collect::<Vec<String>>()
                .join(", ");

            if item_specifiers.len() > 0 && other_specifier_code.len() > 0 {
                return Ok(format!(
                    "import {{ {} }}, {} from \"{}\"",
                    item_specifiers, other_specifier_code, source
                ));
            }

            if item_specifiers.len() > 0 {
                return Ok(format!(
                    "import {{ {} }} from \"{}\"",
                    item_specifiers, source
                ));
            }

            Ok(format!(
                "import {} from \"{}\"",
                other_specifier_code, source
            ))
        }
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

        let lines = result.lines().collect::<Vec<&str>>();
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], "import { foo } from \"./.gen/item-no-rename\"");
        assert_eq!(
            lines[1],
            "import { baz as bar } from \"./.gen/item-with-rename\""
        );
        assert_eq!(lines[2], "import * as myLib from \"./.gen/namespace\"");
        assert_eq!(lines[3], "import myLibsDefault from \"./.gen/default\"");
        assert_eq!(
            lines[4],
            "import { foo, baz as bar }, * as myLib, myLibsDefault from \"./.gen/all-of-the-above\""
        );

        Ok(())
    }
}

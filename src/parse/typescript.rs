use crate::ast::ast::Expression;
use crate::ast::import::{ImportDeclaration, ImportSpecifier};
use nom::branch::alt;
// use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::{map, value};
use nom::error::VerboseError;
use nom::sequence::{pair, tuple};
use nom::IResult;

pub fn parse_single_line_comment(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(pair(tag("//"), is_not("\n\r")), |(_, c)| {
        Expression::Comment(String::from(c))
    })(input)
}

pub fn parse_multi_line_comment(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
        |(_, c, _)| Expression::Comment(String::from(c)),
    )(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((parse_single_line_comment, parse_multi_line_comment))(input)
}

// TODO: handle ' and backtick in source string
pub fn parse_star_import(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        tuple((
            tag("import"),
            take_until("*"),
            tag("*"),
            take_until("as"),
            tag("as"),
            take_until("from"),
            tag("from"),
            take_until("\""),
            tag("\""),
            take_until("\""),
            tag("\""),
            tag(";"),
        )),
        |x| {
            let namespace: &str = x.5;
            let source: &str = x.9;
            Expression::ImportDeclaration(ImportDeclaration {
                specifiers: vec![ImportSpecifier::Namespace(String::from(namespace.trim()))],
                source: String::from(source.trim()),
            })
        },
    )(input)
}

pub fn parse_import(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((parse_star_import, parse_star_import))(input)
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn single_line_string() {
        let (_, expr) = parse_comment("// Foo").unwrap();
        println!("E: {}", expr);
        let s = assert_matches!(expr, Expression::Comment(s) => s);
        assert_eq!(s, " Foo");
    }

    #[test]
    fn multi_line_string() {
        let (_, expr) = parse_comment(
            "/*
Hello there
*/",
        )
        .unwrap();
        let s = assert_matches!(expr, Expression::Comment(s) => s);
        assert_eq!(s, "\nHello there\n");
    }

    #[test]
    fn namespace_import() {
        let expected = ImportDeclaration {
            source: "developers".to_string(),
            specifiers: vec![ImportSpecifier::Namespace("daniel".to_string())],
        };
        let (_, expr) = parse_import("import * as daniel from \"developers\";").unwrap();

        let actual = assert_matches!(expr, Expression::ImportDeclaration(i) => i);
        assert_eq!(actual, expected);
    }

    #[test]
    fn default_import() {
        let expected = ImportDeclaration {
            source: "developers".to_string(),
            specifiers: vec![ImportSpecifier::Default("peter".to_string())],
        };
        let (_, expr) = parse_import("import peter from \"developers\";").unwrap();

        let actual = assert_matches!(expr, Expression::ImportDeclaration(i) => i);
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_specific_import() {
        let expected = ImportDeclaration {
            source: "developers".to_string(),
            specifiers: vec![ImportSpecifier::Item {
                local: "thorsten".to_string(),
                imported: "thorsten".to_string(),
            }],
        };
        let (_, expr) = parse_import("import { thorsten } from \"developers\";").unwrap();

        let actual = assert_matches!(expr, Expression::ImportDeclaration(i) => i);
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_renamed_specific_import() {
        let expected = ImportDeclaration {
            source: "developers".to_string(),
            specifiers: vec![ImportSpecifier::Item {
                local: "sabine".to_string(),
                imported: "thorsten".to_string(),
            }],
        };
        let (_, expr) = parse_import("import { thorsten as sabine } from \"developers\";").unwrap();

        let actual = assert_matches!(expr, Expression::ImportDeclaration(i) => i);
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_renamed_specific_import() {
        let expected = ImportDeclaration {
            source: "developers".to_string(),
            specifiers: vec![
                ImportSpecifier::Item {
                    local: "katrin".to_string(),
                    imported: "katrin".to_string(),
                },
                ImportSpecifier::Item {
                    local: "sabine".to_string(),
                    imported: "thorsten".to_string(),
                },
            ],
        };
        let (_, expr) =
            parse_import("import { katrin, thorsten as sabine } from \"developers\";").unwrap();

        let actual = assert_matches!(expr, Expression::ImportDeclaration(i) => i);
        assert_eq!(actual, expected);
    }
}

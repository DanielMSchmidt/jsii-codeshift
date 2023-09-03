use crate::ast::base::Expression;
use crate::ast::import::{ImportDeclaration, ImportSpecifier};
use nom::branch::alt;
// use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::separated_list1;
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
fn parse_star_import(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
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

fn parse_default_import(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        tuple((
            tag("import"),
            space0,
            alphanumeric1,
            space0,
            tag("from"),
            take_until("\""),
            tag("\""),
            take_until("\""),
            tag("\""),
            tag(";"),
        )),
        |x| {
            let namespace: &str = x.2;
            let source: &str = x.7;
            Expression::ImportDeclaration(ImportDeclaration {
                specifiers: vec![ImportSpecifier::Default(String::from(namespace.trim()))],
                source: String::from(source.trim()),
            })
        },
    )(input)
}

fn parse_import_specification(
    input: &str,
) -> IResult<&str, Vec<ImportSpecifier>, VerboseError<&str>> {
    // Get rid of starting and ending {}, maybe ouside
    // Sequence of foo / foo as bar
    separated_list1(
        tag(","),
        alt(
            (
                map(
                    tuple((
                        space0,
                        alphanumeric1,
                        space0,
                        tag("as"),
                        space0,
                        alphanumeric1,
                        space0,
                    )),
                    |x| ImportSpecifier::Item {
                        imported: String::from(x.1),
                        local: String::from(x.5),
                    },
                ),
                map(tuple((space0, alphanumeric1, space0)), |x| {
                    ImportSpecifier::Item {
                        imported: String::from(x.1),
                        local: String::from(x.1),
                    }
                }),
            ),
            // tag(",")
        ),
    )(input)
}

fn parse_specific_import(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        tuple((
            tag("import"),
            take_until("{"),
            tag("{"),
            take_until("}"),
            tag("}"),
            take_until("from"),
            tag("from"),
            take_until("\""),
            tag("\""),
            take_until("\""),
            tag("\""),
            tag(";"),
        )),
        |x| {
            let import_specification: &str = x.3;
            let source: &str = x.9;
            let (_, specifiers) = parse_import_specification(import_specification).unwrap();
            Expression::ImportDeclaration(ImportDeclaration {
                specifiers,
                source: String::from(source.trim()),
            })
        },
    )(input)
}

pub fn parse_import(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((
        parse_star_import,
        parse_default_import,
        parse_specific_import,
    ))(input)
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

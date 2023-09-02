use crate::ast::ast::Expression;
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
}

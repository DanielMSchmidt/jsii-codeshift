use std::fmt::{Debug, Display, Formatter, Result};

use super::import::ImportDeclaration;

pub trait AstNode: Display + Debug {}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Comment(String),
    ImportDeclaration(ImportDeclaration),
    UnknownExpression(String),
}
impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

pub struct AST {
    pub expressions: Vec<Box<Expression>>,
}

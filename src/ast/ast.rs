use std::fmt::{Debug, Display};

pub trait AstNode: Display + Debug {}

pub struct AST {
    pub expressions: Vec<Box<dyn AstNode>>,
}

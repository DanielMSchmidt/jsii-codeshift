use super::ast::AstNode;
use std::fmt::{self, Debug, Display};

#[derive(Debug, PartialEq)]
pub enum ImportSpecifier {
    Namespace(String),
    Item { imported: String, local: String },
}

impl Display for ImportSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct ImportDeclaration {
    pub source: String,
    // importKind (value vs type) omitted for now
    pub specifiers: Vec<ImportSpecifier>,
}

impl Display for ImportDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "import(source: {}, specifiers: {})",
            self.source,
            self.specifiers
                .iter()
                .fold(String::new(), |a, spec| format!("{}, {}", a, spec))
        )
    }
}
impl AstNode for ImportDeclaration {}

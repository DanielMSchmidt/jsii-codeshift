use crate::ast::base::AST;

use super::GenerateResult;

pub fn generate(_ast: AST) -> GenerateResult {
    Ok(String::from(""))
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::generate::GenerateError;

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
}

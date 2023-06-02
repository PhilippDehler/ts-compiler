use crate::ast::AST;
use crate::ast::AST::*;

pub fn string(s: &str) -> Box<AST> {
    Box::new(Identifier(s.to_string()))
}

pub fn number(n: f64) -> Box<AST> {
    Box::new(NumericLiteral(n))
}

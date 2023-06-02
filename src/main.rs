mod ast;
mod utils;

use core::panic;
use std::collections::HashMap;
use std::collections::LinkedList;
use ast::*;
use ast::AST::*;
use utils::*;
use crate::Operator::*;

type ScopeContextMap = HashMap<String, Box<AST>>;
type ScopeContext = LinkedList<ScopeContextMap>;

fn main() {
    let ast: AST = Block {
        parent: None,
        statements: vec![
            // let a: number = 5;
            VariableStatement {
                name: String::from("a"),
                value: number(5.0),
                is_mutable: true,
                type_info: PrimitveType::Num,
            },
            // a = 4;
            // BinaryExpression {
            //     left: string("a"),
            //     right: number(4.0),
            //     operator: Eq,
            // },
            BinaryExpression {
                left: number(4.0),
                right: string("I'm a string!"),
                operator: Plus,
            }
            // a = "hi";
            // BinaryExpression {
            //     left: string("a"),
            //     right: string("hi"),
            //     operator: Eq,
            // }
        ],
        this: Box::new(NullKeyword),
    };
    let mut global_scope = LinkedList::<ScopeContextMap>::new();
    global_scope.push_back(ScopeContextMap::new());
    type_check(&ast, &mut global_scope);
}

fn type_check(ast: &AST, ctx: &mut ScopeContext) {
    match ast {
        // { ... }
        Block { statements, .. } => {
            // TODO: find all function declaration and put them into scope
            ctx.push_back(HashMap::new());

            for statement in statements.iter() {
                type_check(statement, ctx);
            }
        }
        FunctionBlock { statements, .. } => {
            ctx.push_back(HashMap::new());
            for statement in statements.iter() {
                type_check(statement, ctx);
            }
        }

        // let a: number = "45";
        VariableStatement { name, value, type_info, .. } => {
            if value.as_ref().get_type() != *type_info {
                panic!("Type mismatch: {:?} is not of type {:?}", value, type_info);
            }
            // let and const
            ctx.back_mut().expect("no scope context").insert(name.clone(), Box::new(ast.clone()));
        }

        // a + b
        BinaryExpression { left, right, operator } => {
            match operator {
                Eq => {
                    if let Identifier(name) = left.as_ref() {
                        let i = resolve_scope(&name, &ctx);
                        let map = ctx.iter_mut().nth(i).unwrap();
                        let left_box = map.get(name).unwrap();
                        match left_box.as_ref() {
                            VariableStatement { type_info, .. } => {
                                if right.as_ref().get_type() != *type_info {
                                    panic!(
                                        "Type mismatch: {:?} is not of type {:?}",
                                        right,
                                        type_info
                                    );
                                }
                            }
                            _ => panic!("left hand side of assignment is not a variable statement"),
                        };
                    } else {
                        panic!("left hand side of assignment is not an identifier");
                    }
                }
                Plus | Minus | Multiply | Division | Percentage => {
                    if left.as_ref().get_type() != right.as_ref().get_type() {
                        panic!("Type mismatch: {:?} is not of type {:?}", left, right)
                    }
                }
                _ => unimplemented!(),
            }
        }

        /* Do not require type checking */
        NumericLiteral(..) => (),
        StringLiteral(..) => (),
        BooleanLiteral(..) => (),
        NullKeyword => (),
        Identifier(..) => (),

        _ => unimplemented!("type checking not possible"),
    }
}

fn resolve_scope(name: &String, ctx: &ScopeContext) -> usize {
    ctx
        .iter()
        .enumerate()
        .rev()
        .skip_while(|(_, scope)| { !scope.contains_key(name) })
        .next()
        .expect(&format!("Variable {:?} not found in scope", name)).0
}

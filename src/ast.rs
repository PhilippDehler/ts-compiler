#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Eq,
    Plus,
    Minus,
    Multiply,
    Division,
    Percentage,
    LogicalOr, // ||
    LogicalAnd, // &&
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitveType {
    Num,
    Str,
    Null,
    Never,
    Undefined,
    Boolean,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Type(PrimitveType),
    NullKeyword,
    NumericLiteral(f64), // 3.32
    BooleanLiteral(bool), // true, false
    StringLiteral(String), // "hello"
    Identifier(String), // x
    VariableStatement {
        name: String, // variable name (identifier)
        value: Box<AST>, // e.g. NumericLiteral
        is_mutable: bool,
        type_info: PrimitveType, // e.g. number or never
    }, // let x = 1;
    BinaryExpression {
        left: Box<AST>,
        right: Box<AST>,
        operator: Operator,
    }, // 1 + 2 or a = 23
    Block {
        parent: Option<Box<AST>>,
        statements: Vec<AST>,
        this: Box<AST>,
    },
    FunctionBlock {
        name: String,
        parent: Option<Box<AST>>,
        statements: Vec<AST>,
        this: Box<AST>,
    },
    Parameter {
        name: String, // Identifier
        type_info: Box<PrimitveType>, // Type
        default: Box<AST>,
    },
}

impl AST {
    pub fn get_type(&self) -> PrimitveType {
        match self {
            AST::NumericLiteral(_) => PrimitveType::Num,
            AST::StringLiteral(_) => PrimitveType::Str,
            AST::BooleanLiteral(_) => PrimitveType::Boolean,
            AST::NullKeyword => PrimitveType::Null,
            _ => panic!("Not a primitive type"),
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SpanPosition {
    pub start: usize,
    pub end: usize,
}

// Base AST node
#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub sp: SpanPosition,   // contains information about the node's position (position of span) to be matched to string in the source code
    pub data: T,            // contains the data, wrapped into an inner type
}

#[macro_export]
macro_rules! ok_build_node {
    ($pair:expr, $data:expr) => {
        Ok(
            Node {
                sp: SpanPosition {
                    start: $pair.as_span().start(),
                    end: $pair.as_span().end(),
                },
                data: $data,
            }
        )
    }
}

pub type AST = Node<TranslationUnit>;

// AST nodes
#[derive(Debug, PartialEq)]
pub struct TranslationUnit {
    pub functions: Option<Vec<Node<Function>>>,
    pub main_function: Node<Function>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: Identifier,
    pub return_type: TypeSpecifier,
    pub params: Option<Vec<Node<Declaration>>>,
    pub body: Node<Block>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeSpecifier {
    Bool,
    Float,
    Char,
    Int,
}

impl TypeSpecifier {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "bool" => Some(TypeSpecifier::Bool),
            "float" => Some(TypeSpecifier::Float),
            "char" => Some(TypeSpecifier::Char),
            "int" => Some(TypeSpecifier::Int),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TypeSpecifier::Bool => "bool",
            TypeSpecifier::Float => "float",
            TypeSpecifier::Char => "char",
            TypeSpecifier::Int => "int",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub type_specifier: TypeSpecifier,
    pub identifier: Identifier,
    pub array_size: Option<usize>,  // For "[" ~ integer ~ "]" in the grammar
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub declarations: Vec<Node<Declaration>>,
    pub statements: Vec<Node<Statement>>,
}

// WARN: in Ctiny, statements cannot contain declarations
// this is made to ensure that in a given block, all declarations are at the top
#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(AssignmentStatement),
    If(IfStatement),
    While(WhileStatement),
    Jump(JumpStatement),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStatement {
    pub set_value: GetOrSetValue,
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum JumpStatement {
    Return(Expression),
    Break,
    Continue,
}






// Expressions
#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal), // direct value
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    FunctionCall(FunctionCall),
    TypeCast(TypeCast),
    GetOrSetValue(GetOrSetValue),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct GetOrSetValue {
    pub identifier: Identifier,
    pub index: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct TypeCast {
    pub type_specifier: TypeSpecifier,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Char(char),
    Bool(bool),
}



// Operators
#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
}

// trait Operator
pub trait Operator {
    fn is_binary(&self) -> bool;
    fn is_unary(&self) -> bool;
    fn from_str(s: &str) -> Option<Self> where Self: Sized;
    fn as_str(&self) -> &'static str;
}

impl Operator for BinaryOperator {
    fn is_binary(&self) -> bool {
        true
    }

    fn is_unary(&self) -> bool {
        false
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(BinaryOperator::Plus),
            "-" => Some(BinaryOperator::Minus),
            "*" => Some(BinaryOperator::Multiply),
            "/" => Some(BinaryOperator::Divide),
            "%" => Some(BinaryOperator::Modulo),
            "<" => Some(BinaryOperator::Less),
            ">" => Some(BinaryOperator::Greater),
            "<=" => Some(BinaryOperator::LessOrEqual),
            ">=" => Some(BinaryOperator::GreaterOrEqual),
            "==" => Some(BinaryOperator::Equal),
            "!=" => Some(BinaryOperator::NotEqual),
            "&&" => Some(BinaryOperator::LogicalAnd),
            "||" => Some(BinaryOperator::LogicalOr),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Less => "<",
            BinaryOperator::Greater => ">",
            BinaryOperator::LessOrEqual => "<=",
            BinaryOperator::GreaterOrEqual => ">=",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LogicalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Negation,
    Not,
}

impl Operator for UnaryOperator {
    fn is_binary(&self) -> bool {
        false
    }

    fn is_unary(&self) -> bool {
        true
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "-" => Some(UnaryOperator::Negation),
            "!" => Some(UnaryOperator::Not),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            UnaryOperator::Negation => "-",
            UnaryOperator::Not => "!",
        }
    }
}

use pest::Span;

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct SpanPosition {
//     pub start: usize,
//     pub end: usize,
// }

// Base AST node
#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a, T> {
    pub sp: Span<'a>,   // contains information about the node's position (position of span) to be matched to string in the source code
    pub data: T,            // contains the data, wrapped into an inner type
}

#[macro_export]
macro_rules! ok_build_node {
    ($pair:expr, $data:expr) => {
        Ok(
            Node {
                sp: $pair.as_span(),
                data: $data,
            }
        )
    }
}

pub type AST<'a> = Node<'a, TranslationUnit<'a>>;

// AST nodes
#[derive(Debug, PartialEq)]
pub struct TranslationUnit<'a> {
    pub functions: Option<Vec<Node<'a, Function<'a>>>>,
    pub main_function: Node<'a, Function<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub name: Node<'a, Identifier>,
    pub return_type: TypeSpecifier,
    pub params: Option<Vec<Node<'a, Declaration<'a>>>>,
    pub body: Node<'a, Block<'a>>,
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
pub struct Declaration<'a> {
    pub type_specifier: TypeSpecifier,
    pub identifier: Node<'a, Identifier>,
    pub array_size: Option<usize>,  // For "[" ~ integer ~ "]" in the grammar
}

impl Declaration<'_> {
    pub fn is_array(&self) -> bool {
        self.array_size.is_some()
    }
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub declarations: Vec<Node<'a, Declaration<'a>>>,
    pub statements: Vec<Node<'a, Statement<'a>>>,
}

// WARN: in Ctiny, statements cannot contain declarations
// this is made to ensure that in a given block, all declarations are at the top
#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Assignment(AssignmentStatement<'a>),
    If(IfStatement<'a>),
    While(WhileStatement<'a>),
    Jump(JumpStatement<'a>),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStatement<'a> {
    pub set_value: Node<'a, GetOrSetValue<'a>>,
    pub expression: Node<'a, Expression<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement<'a> {
    pub condition: Node<'a, Expression<'a>>,
    pub if_body: Vec<Node<'a, Statement<'a>>>,
    pub else_body: Option<Vec<Node<'a, Statement<'a>>>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement<'a> {
    pub condition: Node<'a, Expression<'a>>,
    pub body: Vec<Node<'a, Statement<'a>>>,
}

#[derive(Debug, PartialEq)]
pub enum JumpStatement<'a> {
    Return(Expression<'a>),
    Break(),
    Continue,
}






// Expressions
#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Literal(Literal), // direct value
    UnaryExpression(UnaryExpression<'a>),
    BinaryExpression(BinaryExpression<'a>),
    FunctionCall(FunctionCall<'a>),
    TypeCast(TypeCast<'a>),
    GetOrSetValue(GetOrSetValue<'a>),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct GetOrSetValue<'a> {
    pub identifier: Node<'a, Identifier>,
    pub index: Option<Box<Node<'a, Expression<'a>>>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: BinaryOperator,
    pub right: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression<'a> {
    pub operator: UnaryOperator,
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall<'a> {
    pub name: Node<'a, Identifier>,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TypeCast<'a> {
    pub type_specifier: TypeSpecifier,
    pub expression: Box<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i16),
    Float(f32),
    Char(u8),
    Bool(bool),
}

impl Literal {
    pub fn as_type_specifier(&self) -> TypeSpecifier {
        match self {
            Literal::Int(_) => TypeSpecifier::Int,
            Literal::Float(_) => TypeSpecifier::Float,
            Literal::Char(_) => TypeSpecifier::Char,
            Literal::Bool(_) => TypeSpecifier::Bool,
        }
    }
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
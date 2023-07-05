use pest::iterators::Pair;

use crate::syntax_parsing::parser::Rule;

// Base AST node
#[derive(Debug, PartialEq)]
pub struct Node<'a, T> {
    pub pair: Pair<'a, Rule>,
    pub inner: T,
}

// Convenient function to create Node
pub fn make_node<T>(pair: Pair<Rule>, inner: T) -> Node<T> {
    Node { pair, inner }
}

impl<'a, T> Node<'a, T> {
    pub fn get_line_and_column(&self) -> (usize, usize) {
        let pos = self.pair.clone().as_span().start_pos();
        pos.line_col()
    }
}

// AST nodes
#[derive(Debug, PartialEq)]
pub enum Program<'a> {
    TranslationUnit(Vec<Node<'a, FunctionDefinition<'a>>>),
}

#[derive(Debug, PartialEq)]
pub enum FunctionDefinition<'a> {
    EntryPoint(StatementBlock),
    Function {
        name: String,
        return_type: TypeSpecifier,
        params: Vec<Node<'a, Declaration>>,
        body: StatementBlock,
    },
}

#[derive(Debug, PartialEq)]
pub enum TypeSpecifier {
    Bool,
    Float,
    Char,
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub type_specifier: TypeSpecifier,
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub enum StatementBlock {
    Statements(Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(StatementBlock),
    Assignment(AssignmentStatement),
    If(IfStatement),
    While(WhileStatement),
    Jump(JumpStatement),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStatement {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum JumpStatement {
    Return(Expression),
    Break,
    Continue,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    TypeCast {
        type_specifier: TypeSpecifier,
        expression: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    Char(char),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    EqualTo,
    NotEqualTo,
    LogicalAnd,
    LogicalOr,
    Negation,
    Not,
}

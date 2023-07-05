use pest::{iterators::Pair, pratt_parser::Op};

use crate::syntax_parsing::parser::Rule;

// Base AST node
#[derive(Debug, PartialEq)]
pub struct Node<'a, T> {
    pub pair: Pair<'a, Rule>,   // contains information about the node's position and string in the source code
    pub inner: T,               // contains the data, wrapped into an inner type
}

// Convenient function to create Node
pub fn make_node<T>(pair: Pair<Rule>, inner: T) -> Node<T> {
    Node { pair, inner: inner }
}

impl<'a, T> Node<'a, T> {
    pub fn get_line_and_column(&self) -> (usize, usize) {
        let pos = self.pair.clone().as_span().start_pos();
        pos.line_col()
    }
}



// AST nodes
#[derive(Debug, PartialEq)]
pub struct ProgramAST<'a> {
    pub translation_unit: TranslationUnitNode<'a>,
}

// node alias
pub type TranslationUnitNode<'a> = Node<'a, TranslationUnit<'a>>;
pub type FunctionDefinitionNode<'a> = Node<'a, FunctionDefinition<'a>>;
// pub type TypeSpecifierNode<'a> = Node<'a, TypeSpecifier>;
pub type DeclarationNode<'a> = Node<'a, Declaration<'a>>;
pub type ParameterListNode<'a> = Node<'a, ParameterList<'a>>;
pub type BlockNode<'a> = Node<'a, Block<'a>>;
pub type StatementNode<'a> = Node<'a, Statement<'a>>;
pub type AssignmentStatementNode<'a> = Node<'a, AssignmentStatement<'a>>;
pub type IfStatementNode<'a> = Node<'a, IfStatement<'a>>;
pub type WhileStatementNode<'a> = Node<'a, WhileStatement<'a>>;
pub type JumpStatementNode<'a> = Node<'a, JumpStatement<'a>>;
pub type MultiDeclarationNode<'a> = Node<'a, MultiDeclaration<'a>>;
pub type ExpressionNode<'a> = Node<'a, Expression<'a>>;
pub type BinaryExpressionNode<'a> = Node<'a, BinaryExpression<'a>>;
pub type OperatorNode<'a> = Node<'a, Operator>;
pub type LiteralNode<'a> = Node<'a, Literal>;
pub type IdentifierNode<'a> = Node<'a, Identifier>;
pub type FunctionCallNode<'a> = Node<'a, FunctionCall<'a>>;
pub type TypeCastNode<'a> = Node<'a, TypeCast<'a>>;


#[derive(Debug, PartialEq)]
pub struct TranslationUnit<'a> {
    pub functions: Vec<FunctionDefinitionNode<'a>>,
}



#[derive(Debug, PartialEq)]
pub enum FunctionDefinition<'a> {
    EntryPoint(Block<'a>),
    Function {
        name: IdentifierNode<'a>,
        return_type: TypeSpecifier,
        params: Vec<DeclarationNode<'a>>,
        body: Block<'a>,
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
pub struct Declaration<'a> {
    pub type_specifier: TypeSpecifier,
    pub identifier: IdentifierNode<'a>,
    pub array_size: Option<i32>,  // For "[" ~ integer ~ "]" in the grammar
}

#[derive(Debug, PartialEq)]
pub struct ParameterList<'a> {
    pub parameters: Vec<DeclarationNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MultiDeclaration<'a> {
    pub declarations: Vec<DeclarationNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub declarations: Vec<DeclarationNode<'a>>,
    pub statements: Vec<StatementNode<'a>>,
}

// WARN: in Ctiny, statements cannot contain declarations
// this is made to ensure that in a given block, all declarations are at the top
#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Assignment(AssignmentStatementNode<'a>),
    If(IfStatementNode<'a>),
    While(WhileStatementNode<'a>),
    Jump(JumpStatementNode<'a>),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStatement<'a> {
    pub identifier: IdentifierNode<'a>,
    pub expression: ExpressionNode<'a>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement<'a> {
    pub condition: ExpressionNode<'a>,
    pub if_body: Box<StatementNode<'a>>,
    pub else_body: Option<Box<StatementNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement<'a> {
    pub condition: ExpressionNode<'a>,
    pub body: Box<StatementNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum JumpStatement<'a> {
    Return(ExpressionNode<'a>),
    Break,
    Continue,
}

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Identifier(IdentifierNode<'a>),
    Literal(LiteralNode<'a>),
    BinaryExpression(BinaryExpressionNode<'a>),
    FunctionCall(FunctionCallNode<'a>),
    TypeCast(TypeCastNode<'a>),
    GetValue(GetValueNode<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct GetValueNode<'a> {
    pub identifier: IdentifierNode<'a>,
    pub index: Option<Box<ExpressionNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression<'a> {
    left: Box<ExpressionNode<'a>>,
    operator: OperatorNode<'a>,
    right: Box<ExpressionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall<'a> {
    name: IdentifierNode<'a>,
    arguments: Vec<ExpressionNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TypeCast<'a> {
    type_specifier: TypeSpecifier,
    expression: Box<ExpressionNode<'a>>,
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

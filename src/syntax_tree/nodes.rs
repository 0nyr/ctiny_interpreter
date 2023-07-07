
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

pub type AST<T> = Node<T>;



// AST nodes
#[derive(Debug, PartialEq)]
pub struct ProgramAST {
    pub translation_unit: TranslationUnit,
}


#[derive(Debug, PartialEq)]
pub struct TranslationUnit {
    pub functions: Vec<FunctionDefinition>,
}



#[derive(Debug, PartialEq)]
pub enum FunctionDefinition {
    EntryPoint(Block),
    Function {
        name: Identifier,
        return_type: TypeSpecifier,
        params: Vec<Declaration>,
        body: Block,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeSpecifier {
    Bool,
    Float,
    Char,
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub type_specifier: TypeSpecifier,
    pub identifier: Identifier,
    pub array_size: Option<i32>,  // For "[" ~ integer ~ "]" in the grammar
}

#[derive(Debug, PartialEq)]
pub struct ParameterList {
    pub parameters: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub struct MultiDeclaration {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub declarations: Vec<Declaration>,
    pub statements: Vec<Statement>,
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
    pub identifier: Identifier,
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






// Expressions
#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal), // direct value
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    FunctionCall(FunctionCall),
    TypeCast(TypeCast),
    GetValue(GetValue),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct GetValue {
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

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Negation,
    Not,
}

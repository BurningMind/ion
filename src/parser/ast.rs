use std;
use lexer::Span;
use std::hash::*;

#[derive(Debug, Clone)]
pub struct Ast {
    pub statements: std::vec::Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Ast { statements: vec![] }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Import(Box<ImportData>),
    Package(Box<PackageData>),
    FuncDecl(Box<FuncDeclData>),
    StructDecl(Box<StructDeclData>),
}

#[derive(Debug, Clone)]
pub struct ImportData {
    pub span: Span,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct PackageData {
    pub span: Span,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct FuncDeclData {
    pub span: Span,
    pub name: String,
    pub return_type: Type,
    pub parameters: std::vec::Vec<Box<FuncDeclParamData>>,
    pub statements: std::vec::Vec<BlockStatement>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum BlockStatement {
    Expression(Box<Expression>),
    VarDecl(Box<VarDeclData>),
    VarAssignment(Box<Expression>, Box<Expression>),
    If(Box<IfData>),
    While(Box<WhileData>),
    Return(Box<ReturnData>),
    ForIn(Box<ForInData>),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct ForInData {
    pub span: Span,
    pub element_name: String,
    pub collection: Expression,
    pub statements: std::vec::Vec<BlockStatement>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct ReturnData {
    pub span: Span,
    pub value: Option<Expression>,
    pub expected_type: Type,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct IfData {
    pub span: Span,
    pub condition: Expression,
    pub if_statements: std::vec::Vec<BlockStatement>,
    pub else_statements: Option<std::vec::Vec<BlockStatement>>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct WhileData {
    pub span: Span,
    pub condition: Expression,
    pub statements: std::vec::Vec<BlockStatement>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct VarDeclData {
    pub span: Span,
    pub name: String,
    pub var_type: Type,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct FuncDeclParamData {
    pub span: Span,
    pub name: String,
    pub param_type: Type,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    None,
    Reference(Box<Type>),
    MutReference(Box<Type>),
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Struct(Path),
    Func(Box<Type>, std::vec::Vec<Box<Type>>),
    String,
    Int,
    Bool,
    Char,
}

#[derive(Debug, Clone)]
pub struct StructDeclData {
    pub span: Span,
    pub name: String,
    pub fields: std::vec::Vec<Box<StructFieldData>>,
}

#[derive(Debug, Clone)]
pub struct StructFieldData {
    pub span: Span,
    pub name: String,
    pub field_type: Type,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub expr: Expression_,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression_ {
    StringLiteral(String),
    IntegerLiteral(i64),
    BoolLiteral(bool),
    CharLiteral(char),
    Variable(Path),
    StructInit(Path, std::vec::Vec<StructInitFieldData>),
    Array(std::vec::Vec<Box<Expression>>),
    Map(Map),
    FuncCall(Box<Expression>, std::vec::Vec<Box<Expression>>),
    Field(Box<Expression>, SpannedString),
    Index(Box<Expression>, Option<Box<Expression>>),
    UnaryOp(UnaryOp, Box<Expression>),
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub map: std::collections::HashMap<Box<Expression>, Box<Expression>>,
}

#[allow(derive_hash_xor_eq)]
impl Hash for Map {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        for (key, value) in &self.map {
            key.hash(state);
            value.hash(state);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulo,
    Equality,
    Inequality,
    Concatenation,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Reference,
    MutReference,
    Dereference,
    Count,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Op {
    Unary(UnaryOp),
    Binary(BinaryOp),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructInitFieldData {
    pub span: Span,
    pub name: SpannedString,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub span: Span,
    pub parts: std::vec::Vec<SpannedString>,
}

impl Path {
    pub fn concat(path1: Path, path2: Path) -> Path {
        let mut new_parts = path1.parts.clone();

        for ss in path2.parts {
            new_parts.push(ss);
        }

        Path {
            span: Span::concat(path1.span, path2.span),
            parts: new_parts,
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.parts == other.parts
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.parts.hash(state)
    }
}

#[derive(Debug, Clone)]
pub struct SpannedString {
    pub span: Span,
    pub ident: String,
}

impl PartialEq for SpannedString {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for SpannedString {}

impl Hash for SpannedString {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.ident.hash(state)
    }
}

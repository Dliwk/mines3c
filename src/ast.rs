//! Abstract Syntax Tree

pub type Ident = String;
pub type Integer = u32;

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: usize,
    pub node: T,
}

impl<T> Located<T> {
    pub fn new(location: usize, node: T) -> Self {
        Self { location, node }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, PartialEq)]
pub enum ArgumentType {
    Nothing,
    Argument,
    State,
}

#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Nothing,
    Argument,
    State,
}

#[derive(Debug, PartialEq)]
pub enum DefinitionKind {
    Function {
        name: Ident,
        arg: ArgumentType,
        returns: ReturnType,
        body: Vec<Statement>,
    },
    InlineFunction {
        name: Ident,
        body: Vec<Statement>,
    },
    Const {
        name: Ident,
        value: Integer,
    },
}

pub type Definition = Located<DefinitionKind>;

#[derive(Debug, PartialEq)]
pub enum StatementKind {
    StatementBlock {
        body: Vec<Statement>,
    },
    FunctionCall {
        name: Ident,
    },
    Loop {
        body: Vec<Statement>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Statement>,
        negative: bool,
    },
    If {
        condition: Box<Expr>,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
        negative: bool,
    },
    Return {
        expr: Option<Box<Expr>>,
    },
    InlineCode {
        body: InlineCode,
    },
}

#[derive(Debug, PartialEq)]
pub struct InlineCode {
    pub code: String,
}

pub type Statement = Located<StatementKind>;

#[derive(Debug, PartialEq)]
pub enum ComparisonOp {
    Eq,
    NotEq,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    FunctionCallResult {
        name: Ident,
    },
    VariableComparison {
        var: Variable,
        value: IntegerExpr,
        op: ComparisonOp,
    },
    InlineCode {
        body: InlineCode,
    },
}

pub type Expr = Located<ExprKind>;

#[derive(Debug, PartialEq)]
pub enum IntegerExpr {
    Literal { value: u32 },
    Const { name: Ident },
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: Ident,
}

//! Abstract Syntax Tree

type Ident = String;
type Integer = u32;

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: usize,
    pub node: T,
}

impl<T> Located<T> {
    pub fn new(location: usize, node: T) -> Self {
        Self {
            location,
            node,
        }
    }
}

pub struct Program {
    definitions: Vec<Definition>,
}

pub enum ArgumentType {
    Nothing,
    Argument,
    State,
}

pub enum ReturnType {
    Nothing,
    Argument,
    State,
}

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
}

pub type Definition = Located<DefinitionKind>;

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
        condition: Box<Condition>,
        body: Vec<Statement>,
    },
    If {
        condition: Box<Condition>,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    },
}

type Statement = Located<StatementKind>;

pub enum ComparisonOp {
    Eq,
    NotEq,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
}

pub enum ConditionKind {
    FunctionCallResult {
        name: Ident,
    },
    VariableComparison {
        var: Variable,
        value: Integer,
        op: ComparisonOp,
    },
}

pub type Condition = Located<ConditionKind>;

pub struct Variable {
    name: Ident,
}


/// Defines Excel Functions.
#[derive(Debug)]
pub enum Function {
    Abs,
    Sum,
    Product,
    Or,
    And,
    Xor,
    Not,
}

/// Defines Excel Operators.
#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Concat,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Function(Function),
}

/// Defines error types.
#[derive(Debug)]
pub enum Error {
    Div0,
    Cast,
    Formula,
    Parse,
    Value,
}

/// Defines boolean types.
#[derive(Debug)]
pub enum Boolean {
    True,
    False,
}

/// The result of an evaluation.
#[derive(Debug)]
pub enum Value {
    Number(f32),
    Text(String),
    Boolean(Boolean),
    Error(Error),
}

/// Defines each term in Expression Struct.
#[derive(Debug)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
    Reference(String),
}

/// Struct that holds a parsed string. Formula enum and Expression Struct are defined recursively.
#[derive(Debug)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

/// Defines Excel Functions.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum Error {
    Div0,
    Cast,
    Formula,
    Parse,
    Value,
}

/// Defines boolean types.
#[derive(Debug, Clone)]
pub enum Boolean {
    True,
    False,
}

/// The result of an evaluation.
#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Text(String),
    Boolean(Boolean),
    //Iterator(Vec<Value>),
    Error(Error),
}

/// Defines each term in Expression Struct.
#[derive(Debug, Clone)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
    Reference(String),
    Iterator(Vec<Formula>),
}

/// Struct that holds a parsed string. Formula enum and Expression Struct are defined recursively.
#[derive(Debug, Clone)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

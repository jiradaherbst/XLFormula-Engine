use chrono::{DateTime, FixedOffset};
/// Defines Excel Functions.
#[derive(Debug, Clone)]
pub enum Function {
    Abs,
    Sum,
    Product,
    Average,
    Or,
    And,
    Xor,
    Not,
    Negate,
    Days,
    Right,
    Left,
    Iff,
    Custom(String),
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Div0,
    Cast,
    Parse,
    Value,
    Argument,
    Reference,
}

/// Defines boolean types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Text(String),
    Boolean(Boolean),
    Iterator(Vec<Value>),
    Error(Error),
    Date(DateTime<FixedOffset>),
    Blank,
}

impl Value {
    pub fn as_num(&self) -> Result<f32, Error> {
        match self {
            Value::Number(v) => Ok(*v),
            Value::Text(v) => match v.parse::<f32>() {
                Err(_) => Err(Error::Cast),
                Ok(v) => Ok(v),
            },
            Value::Boolean(v) => match v {
                Boolean::True => Ok(1.0),
                Boolean::False => Ok(0.0),
            },
            Value::Blank => Ok(0.0),
            _ => Err(Error::Cast),
        }
    }

    pub fn as_string(&self) -> Result<String, Error> {
        match self {
            Value::Number(v) => Ok(format!("{}", v)),
            Value::Text(v) => Ok(v.clone()),
            Value::Boolean(v) => match v {
                Boolean::True => Ok("1.0".to_owned()),
                Boolean::False => Ok("0.0".to_owned()),
            },
            Value::Blank => Ok("".to_owned()),
            _ => Err(Error::Cast),
        }
    }

    pub fn as_bool(&self) -> Result<bool, Error> {
        match self {
            Value::Number(v) => Ok(*v > 0.0),
            Value::Text(v) => Ok(v.len() > 0),
            Value::Boolean(v) => match v {
                Boolean::True => Ok(true),
                Boolean::False => Ok(false),
            },
            Value::Blank => Ok(false),
            _ => Err(Error::Cast),
        }
    }
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

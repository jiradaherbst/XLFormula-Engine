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

#[derive(Debug)]
pub enum Error {
    Div0,
    Cast,
    Formula,
    Parse,
    Value,
}

#[derive(Debug)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug)]
pub enum Value {
    Number(f32),
    Text(String),
    Boolean(Boolean),
    Error(Error),
}

#[derive(Debug)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
    Reference(String),
}

#[derive(Debug)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

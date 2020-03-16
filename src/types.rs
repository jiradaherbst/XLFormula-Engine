#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Null,
}

#[derive(Debug)]
pub enum Value {
    Number(f32),
    Text(String),
    Error(String),
}

#[derive(Debug)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
}

#[derive(Debug)]
pub struct Expression {
    pub lhs: Box<Formula>,
    pub rhs: Box<Formula>,
    pub op: Operator,
}

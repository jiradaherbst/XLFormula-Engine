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
    // Reference
}

#[derive(Debug)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

//1+2 -> Expression(lhs:Value(1), rhs:Value(2), op: +)
//1+2+3 -> Expression(lhs:Value(1), rhs:Expression(lhs:2, rhs: 3, op: +), op: +)
//1+2*3 -> Expression(lhs:Value(1), rhs:Expression(lhs:2, rhs: 3, op: *), op: +)
//1*2+3 -> Expression(lhs:Expression(lhs:V(1), rhs:V(2), op: *), rhs:V(3), op: +)
//1*(2+3) -> Expression(lhs:V(1), rhs: Expression(lhs:V(2), rhs:V(3), op: +), op: *)
//SUM(1,2,3) -> Expression(params: [V(1), V(2), V(3)], fn: SUM)

//1+2+3 -> Expression(op: +, values: vec[Value(1), Value(2), Value(3)])
//1+2*3 -> Expression(op: +, values: vec[Value(1), Expression(op: *, values: [2, 3])])
//1*2+3 -> Expression(op: +, values: vec[Expression(op: *, values: [1, 2]), V(3)])
//1*(2+3) -> Expression(op: *, values: vec[V(1), Expression(op: +, values: [2, 3])])
//SUM(1,2,3) -> Expression(fn: SUM, values: vec[Value(1), Value(2), Value(3)])

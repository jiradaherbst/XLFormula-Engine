// #[derive(Debug)]
// pub enum Function {
//     Sum,
//     Product,
//     Abs
// }
#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Null, //?
          // Function(Function)
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
    // Reference
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

#[derive(Debug)]
pub struct Expression {
    //pub lhs: Box<Formula>,
    pub op: Operator,
    //pub rhs: Box<Formula>,
    pub values: Vec<Formula>,
}

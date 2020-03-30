use crate::types;

fn calculate_divide_operator(num1: f32, num2: f32) -> f32 {
    num1 / num2
}

fn is_float_int(num: f32) -> bool {
    ((num as i32) as f32) == num
}

fn calculate_power_operator(num1: f32, num2: f32) -> f32 {
    if is_float_int(num2) {
        num1.powi(num2 as i32)
    } else {
        num1.powf(num2)
    }
}

fn calculate_concat_operator(str1: &String, str2: &String) -> String {
    str1.to_owned() + str2
}

fn calculate_string_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(str1: &String, str2: &String) -> String,
) -> types::Value {
    match lhs {
        types::Value::Error(_) => lhs,
        types::Value::Number(l) => match rhs {
            types::Value::Error(_) => rhs,
            types::Value::Number(r) => types::Value::Text(f(&l.to_string(), &r.to_string())),
            types::Value::Text(r) => types::Value::Text(f(&l.to_string(), &r)),
        },
        types::Value::Text(l) => match rhs {
            types::Value::Error(_) => rhs,
            types::Value::Number(r) => types::Value::Text(f(&l, &r.to_string())),
            types::Value::Text(r) => types::Value::Text(f(&l, &r)),
        },
    }
}

fn calculate_numeric_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match lhs {
        types::Value::Error(_) => lhs,
        types::Value::Text(_) => types::Value::Error(types::Error::Cast),
        types::Value::Number(l) => match rhs {
            types::Value::Error(_) => rhs,
            types::Value::Text(_) => types::Value::Error(types::Error::Cast),
            types::Value::Number(r) => types::Value::Number(f(l, r)),
        },
    }
}

fn calculate_abs(value: types::Value) -> types::Value {
    match value {
        types::Value::Error(_) => value,
        types::Value::Text(_) => types::Value::Error(types::Error::Cast),
        types::Value::Number(l) => types::Value::Number(l.abs()),
    }
}

pub fn calculate_formula(formula: types::Formula) -> types::Value {
    match formula {
        types::Formula::Operation(mut exp) => {
            let value2 = match exp.values.pop() {
                Some(formula) => calculate_formula(formula),
                None => types::Value::Error(types::Error::Formula),
            };
            let value1 = match exp.values.pop() {
                Some(formula) => calculate_formula(formula),
                None => types::Value::Error(types::Error::Formula),
            };
            match exp.op {
                types::Operator::Plus => {
                    calculate_numeric_operator(value1, value2, |n1, n2| n1 + n2)
                }
                types::Operator::Minus => {
                    calculate_numeric_operator(value1, value2, |n1, n2| n1 - n2)
                }
                types::Operator::Multiply => {
                    calculate_numeric_operator(value1, value2, |n1, n2| n1 * n2)
                }
                types::Operator::Divide => match value2 {
                    types::Value::Number(x) if x == 0.0 => types::Value::Error(types::Error::Div0),
                    _ => calculate_numeric_operator(value1, value2, calculate_divide_operator),
                },
                types::Operator::Power => {
                    calculate_numeric_operator(value1, value2, calculate_power_operator)
                }
                types::Operator::Concat => {
                    calculate_string_operator(value1, value2, calculate_concat_operator)
                }
                types::Operator::Function(f) => match f {
                    types::Function::Abs => calculate_abs(value2),
                    types::Function::Sum => types::Value::Error(types::Error::Formula),
                }, //types::Operator::Null => types::Value::Error(types::Error::Formula),
            }
        }
        types::Formula::Value(val) => val,
    }
}

pub fn result_to_string(_value: types::Value) -> String {
    match _value {
        types::Value::Number(number) => number.to_string(),
        types::Value::Text(text) => text,
        types::Value::Error(error) => match error {
            types::Error::Div0 => String::from("#DIV/0!"),
            types::Error::Cast => String::from("#CAST!"),
            types::Error::Formula => String::from("Null Formula"),
        },
    }
}

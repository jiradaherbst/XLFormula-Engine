use crate::types;

fn calculate_plus_operator(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

fn calculate_minus_operator(num1: f32, num2: f32) -> f32 {
    num1 - num2
}

fn calculate_multiply_operator(num1: f32, num2: f32) -> f32 {
    num1 * num2
}

fn calculate_divide_operator(num1: f32, num2: f32) -> f32 {
    num1 / num2
}

fn cast_value_to_number(value: types::Value) -> Option<f32> {
    match value {
        types::Value::Number(number) => Some(number),
        types::Value::Text(_) => None,
        types::Value::Error(_) => None,
    }
}

fn option_map2<T, U, V>(a: Option<T>, b: Option<U>, f: fn(a1: T, b1: U) -> V) -> Option<V> {
    if let (Some(value_a), Some(value_b)) = (a, b) {
        Some(f(value_a, value_b))
    } else {
        None
    }
}
fn calculate_numeric_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    let l = cast_value_to_number(lhs);
    let r = cast_value_to_number(rhs);
    match option_map2(l, r, f) {
        Some(result) => types::Value::Number(result),
        None => types::Value::Error(String::from("Error")),
    }
}

pub fn calculate_formula(formula: types::Formula) -> types::Value {
    match formula {
        types::Formula::Operation(exp) => {
            let value1 = calculate_formula(*exp.lhs);
            let value2 = calculate_formula(*exp.rhs);

            match exp.op {
                types::Operator::Plus => {
                    calculate_numeric_operator(value1, value2, calculate_plus_operator)
                }
                types::Operator::Minus => {
                    calculate_numeric_operator(value1, value2, calculate_minus_operator)
                }
                types::Operator::Multiply => {
                    calculate_numeric_operator(value1, value2, calculate_multiply_operator)
                }
                types::Operator::Divide => {
                    calculate_numeric_operator(value1, value2, calculate_divide_operator)
                }
                types::Operator::Null => types::Value::Error(String::from("Error")),
            }
        }
        types::Formula::Value(val) => val,
    }
}

pub fn result_to_string(_value: types::Value) -> String {
    match _value {
        types::Value::Number(number) => number.to_string(),
        types::Value::Text(text) => text,
        types::Value::Error(error) => error, // String::from("Error: "),
    }
}

use crate::parse_formula;
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
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Number(l) => match rhs {
            types::Value::Boolean(_) => rhs,
            types::Value::Error(_) => rhs,
            types::Value::Number(r) => types::Value::Text(f(&l.to_string(), &r.to_string())),
            types::Value::Text(r) => types::Value::Text(f(&l.to_string(), &r)),
        },
        types::Value::Text(l) => match rhs {
            types::Value::Boolean(_) => rhs,
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
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Text(t) => match t.parse::<f32>() {
            Ok(nl) => match rhs {
                types::Value::Boolean(_) => rhs,
                types::Value::Error(_) => rhs,
                types::Value::Text(t) => match t.parse::<f32>() {
                    Ok(nr) => types::Value::Number(f(nl, nr)),
                    Err(_) => types::Value::Error(types::Error::Cast),
                },
                types::Value::Number(r) => types::Value::Number(f(nl, r)),
            },
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(l) => match rhs {
            types::Value::Boolean(_) => rhs,
            types::Value::Error(_) => rhs,
            types::Value::Text(t) => match t.parse::<f32>() {
                Ok(nr) => types::Value::Number(f(l, nr)),
                Err(_) => types::Value::Error(types::Error::Cast),
            },
            types::Value::Number(r) => types::Value::Number(f(l, r)),
        },
    }
}

fn calculate_comparison_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> bool,
) -> types::Value {
    match lhs {
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Text(_) => lhs,
        types::Value::Number(l) => match rhs {
            types::Value::Boolean(_) => rhs,
            types::Value::Error(_) => rhs,
            types::Value::Text(_) => rhs,
            types::Value::Number(r) => match f(l, r) {
                true => types::Value::Boolean(types::Boolean::True),
                false => types::Value::Boolean(types::Boolean::False),
            },
        },
    }
}

fn to_bool(value: types::Boolean) -> bool {
    match value {
        types::Boolean::True => true,
        types::Boolean::False => false,
    }
}

fn calculate_boolean_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    let lh = cast_value_to_boolean(lhs);
    match lh {
        types::Value::Boolean(l) => {
            let rh = cast_value_to_boolean(rhs);
            match rh {
                types::Value::Boolean(r) => match f(to_bool(l), to_bool(r)) {
                    true => types::Value::Boolean(types::Boolean::True),
                    false => types::Value::Boolean(types::Boolean::False),
                },
                types::Value::Error(_) => match l {
                    types::Boolean::True => types::Value::Boolean(types::Boolean::True),
                    types::Boolean::False => types::Value::Boolean(types::Boolean::False),
                },
                _ => unreachable!(),
            }
        }
        types::Value::Error(_) => {
            let rh = cast_value_to_boolean(rhs);
            match rh {
                types::Value::Boolean(r) => match to_bool(r) {
                    true => types::Value::Boolean(types::Boolean::True),
                    false => types::Value::Boolean(types::Boolean::False),
                },
                types::Value::Error(_) => types::Value::Error(types::Error::Cast),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn calculate_abs(value: types::Value) -> types::Value {
    match value {
        types::Value::Boolean(_) => value,
        types::Value::Error(_) => value,
        types::Value::Text(_) => value,
        types::Value::Number(l) => types::Value::Number(l.abs()),
    }
}

fn calculate_negation(value: types::Value) -> types::Value {
    match value {
        types::Value::Boolean(l) => match !(to_bool(l)) {
            true => types::Value::Boolean(types::Boolean::True),
            false => types::Value::Boolean(types::Boolean::False),
        },
        types::Value::Error(_) => value,
        types::Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => match !(to_bool(l)) {
                    true => types::Value::Boolean(types::Boolean::True),
                    false => types::Value::Boolean(types::Boolean::False),
                },
                None => types::Value::Error(types::Error::Cast),
            }
        }
        types::Value::Number(l) => match l == 0.0 {
            true => types::Value::Boolean(types::Boolean::True),
            false => types::Value::Boolean(types::Boolean::False),
        },
    }
}

fn cast_text_to_boolean(s: &String) -> Option<types::Boolean> {
    match s.eq_ignore_ascii_case("TRUE") {
        true => Some(types::Boolean::True),
        false => match s.eq_ignore_ascii_case("FALSE") {
            true => Some(types::Boolean::False),
            false => None,
        },
    }
}

fn cast_value_to_boolean(value: types::Value) -> types::Value {
    match value {
        types::Value::Boolean(_) => value,
        types::Value::Error(_) => value,
        types::Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => match to_bool(l) {
                    true => types::Value::Boolean(types::Boolean::True),
                    false => types::Value::Boolean(types::Boolean::False),
                },
                None => types::Value::Error(types::Error::Cast),
            }
        }
        types::Value::Number(l) => match l != 0.0 {
            true => types::Value::Boolean(types::Boolean::True),
            false => types::Value::Boolean(types::Boolean::False),
        },
    }
}

/// Evaluates a string that was parsed and stored in Expression Struct.
/// Takes an optional closure with the trait bound Fn(String) -> types::Value.
pub fn calculate_formula(
    formula: types::Formula,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    match formula {
        types::Formula::Operation(mut exp) => match exp.op {
            types::Operator::Plus => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_numeric_operator(value1, value2, |n1, n2| n1 + n2)
            }

            types::Operator::Minus => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_numeric_operator(value1, value2, |n1, n2| n1 - n2)
            }

            types::Operator::Multiply => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_numeric_operator(value1, value2, |n1, n2| n1 * n2)
            }
            types::Operator::Divide => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                match value2 {
                    types::Value::Number(x) if x == 0.0 => types::Value::Error(types::Error::Div0),
                    _ => calculate_numeric_operator(value1, value2, calculate_divide_operator),
                }
            }
            types::Operator::Power => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_numeric_operator(value1, value2, calculate_power_operator)
            }
            types::Operator::Concat => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_string_operator(value1, value2, calculate_concat_operator)
            }
            types::Operator::Equal => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_comparison_operator(value1, value2, |n1, n2| n1 == n2)
            }
            types::Operator::NotEqual => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_comparison_operator(value1, value2, |n1, n2| n1 != n2)
            }
            types::Operator::Greater => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_comparison_operator(value1, value2, |n1, n2| n1 > n2)
            }
            types::Operator::Less => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_comparison_operator(value1, value2, |n1, n2| n1 < n2)
            }
            types::Operator::GreaterOrEqual => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_comparison_operator(value1, value2, |n1, n2| n1 >= n2)
            }
            types::Operator::LessOrEqual => {
                let value2 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                let value1 = match exp.values.pop() {
                    Some(formula) => calculate_formula(formula, f),
                    None => types::Value::Error(types::Error::Formula),
                };
                calculate_comparison_operator(value1, value2, |n1, n2| n1 <= n2)
            }
            types::Operator::Function(func) => match func {
                types::Function::Abs => {
                    let value = match exp.values.pop() {
                        Some(formula) => calculate_formula(formula, f),
                        None => types::Value::Error(types::Error::Formula),
                    };
                    calculate_abs(value)
                }
                types::Function::Sum => {
                    let mut sum = types::Value::Number(0.00);
                    while let Some(top) = exp.values.pop() {
                        let value = calculate_formula(top, f);
                        sum = calculate_numeric_operator(sum, value, |n1, n2| n1 + n2);
                    }
                    sum
                }
                types::Function::Product => {
                    let mut product = types::Value::Number(1.00);
                    while let Some(top) = exp.values.pop() {
                        let value = calculate_formula(top, f);
                        product = calculate_numeric_operator(product, value, |n1, n2| n1 * n2);
                    }
                    product
                }
                types::Function::Or => {
                    let mut result = match exp.values.pop() {
                        Some(formula) => calculate_formula(formula, f),
                        None => types::Value::Error(types::Error::Formula),
                    };
                    result = cast_value_to_boolean(result);
                    while let Some(top) = exp.values.pop() {
                        let value = calculate_formula(top, f);
                        result = calculate_boolean_operator(result, value, |n1, n2| n1 || n2);
                    }
                    result
                }
                types::Function::And => {
                    let mut result = match exp.values.pop() {
                        Some(formula) => calculate_formula(formula, f),
                        None => types::Value::Error(types::Error::Formula),
                    };
                    result = cast_value_to_boolean(result);
                    while let Some(top) = exp.values.pop() {
                        let value = calculate_formula(top, f);
                        result = calculate_boolean_operator(result, value, |n1, n2| n1 && n2);
                    }
                    result
                }
                types::Function::Xor => {
                    let mut result = match exp.values.pop() {
                        Some(formula) => calculate_formula(formula, f),
                        None => types::Value::Error(types::Error::Formula),
                    };
                    result = cast_value_to_boolean(result);
                    while let Some(top) = exp.values.pop() {
                        let value = calculate_formula(top, f);
                        result = calculate_boolean_operator(result, value, |n1, n2| n1 ^ n2);
                    }
                    result
                }
                types::Function::Not => {
                    let value = match exp.values.pop() {
                        Some(formula) => calculate_formula(formula, f),
                        None => types::Value::Error(types::Error::Formula),
                    };
                    calculate_negation(value)
                }
            },
        },
        types::Formula::Value(val) => val,
        types::Formula::Reference(string) => match f {
            Some(f) => match f(string) {
                types::Value::Number(x) => types::Value::Number(x),
                types::Value::Text(s) => {
                    let formula = parse_formula::parse_string_to_formula(&s);
                    calculate_formula(formula, Some(f))
                }
                types::Value::Boolean(x) => types::Value::Boolean(x),
                types::Value::Error(types::Error::Value) => {
                    types::Value::Error(types::Error::Value)
                }
                _ => unreachable!(),
            },
            None => types::Value::Error(types::Error::Formula),
        },
        types::Formula::Iterator(mut vec) => {
            let mut result = match vec.pop() {
                Some(formula) => {
                    //println!("{:?}", formula);
                    calculate_formula(formula, f)
                }
                None => types::Value::Error(types::Error::Formula),
            };
            while let Some(top) = vec.pop() {
                let value = calculate_formula(top, f);
                result = calculate_numeric_operator(result, value, |n1, n2| n1 + n2);
            }
            result
        }
    }
}

/// Converts a result from Value Enum to a printable string.  
pub fn result_to_string(_value: types::Value) -> String {
    match _value {
        types::Value::Number(number) => number.to_string(),
        types::Value::Text(text) => text,
        types::Value::Error(error) => match error {
            types::Error::Div0 => String::from("#DIV/0!"),
            types::Error::Cast => String::from("#CAST!"),
            types::Error::Formula => String::from("Null Formula"),
            types::Error::Parse => String::from("#PARSE!"),
            types::Error::Value => String::from("#VALUE!"),
        },
        types::Value::Boolean(boolean) => match boolean {
            types::Boolean::True => String::from("TRUE"),
            types::Boolean::False => String::from("FALSE"),
        },
    }
}

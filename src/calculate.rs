use crate::parse_formula;
use crate::types;
use chrono::{DateTime, Duration, FixedOffset};
type NoCustomFunction<'a> = &'a fn(String, Vec<f32>) -> types::Value;

fn calculate_divide_operator(num1: f32, num2: f32) -> f32 {
    num1 / num2
}

fn is_float_int(num: f32) -> bool {
    //((num as i32) as f32) == num
    (((num as i32) as f32) - num).abs() == 0.0
}

fn calculate_power_operator(num1: f32, num2: f32) -> f32 {
    if is_float_int(num2) {
        num1.powi(num2 as i32)
    } else {
        num1.powf(num2)
    }
}

fn calculate_concat_operator(str1: &str, str2: &str) -> String {
    str1.to_owned() + str2
}

fn calculate_string_operation_rhs(
    l: &str,
    rhs: types::Value,
    f: fn(str1: &str, str2: &str) -> String,
) -> types::Value {
    match rhs {
        types::Value::Boolean(_) => rhs,
        types::Value::Error(_) => rhs,
        types::Value::Number(r) => types::Value::Text(f(&l, &r.to_string())),
        types::Value::Text(r) => types::Value::Text(f(&l, &r)),
        types::Value::Iterator(_) => types::Value::Error(types::Error::Value),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => types::Value::Text(f(&l, "")),
    }
}

fn calculate_string_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(str1: &str, str2: &str) -> String,
) -> types::Value {
    match lhs {
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Number(l) => calculate_string_operation_rhs(&l.to_string(), rhs, f),
        types::Value::Text(l) => calculate_string_operation_rhs(&l, rhs, f),
        types::Value::Iterator(_) => types::Value::Error(types::Error::Value),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => calculate_string_operation_rhs("", rhs, f),
    }
}

fn calcualte_numeric_operator_rhs_text(
    t: String,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match t.parse::<f32>() {
        Ok(nl) => match rhs {
            types::Value::Boolean(_) => rhs,
            types::Value::Error(_) => rhs,
            types::Value::Text(t) => match t.parse::<f32>() {
                Ok(nr) => types::Value::Number(f(nl, nr)),
                Err(_) => types::Value::Error(types::Error::Cast),
            },
            types::Value::Number(r) => types::Value::Number(f(nl, r)),
            types::Value::Iterator(_) => types::Value::Error(types::Error::Value),
            types::Value::Date(_) => types::Value::Error(types::Error::Value),
            types::Value::Blank => types::Value::Error(types::Error::Value),
        },
        Err(_) => types::Value::Error(types::Error::Cast),
    }
}

fn calculate_numeric_operator_rhs_number(
    l: f32,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match rhs {
        types::Value::Boolean(_) => rhs,
        types::Value::Error(_) => rhs,
        types::Value::Text(t) => match t.parse::<f32>() {
            Ok(nr) => types::Value::Number(f(l, nr)),
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(r) => types::Value::Number(f(l, r)),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => types::Value::Number(f(l, 0.0)),
    }
}

fn calculate_numeric_operator_product_rhs_number(
    l: f32,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match rhs {
        types::Value::Boolean(_) => rhs,
        types::Value::Error(_) => rhs,
        types::Value::Text(t) => match t.parse::<f32>() {
            Ok(nr) => types::Value::Number(f(l, nr)),
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(r) => types::Value::Number(f(l, r)),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_product_operator(temp, top, f);
                }
                calculate_numeric_product_operator(lhs, temp, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => match lhs {
            types::Value::Blank => types::Value::Blank,
            _ => types::Value::Number(l),
        },
    }
}

fn calculate_numeric_operator_rhs_iterator(
    mut lhs_vec: Vec<types::Value>,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match rhs {
        types::Value::Number(_) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Iterator(mut rhs_vec) => {
            let mut result_vec = Vec::new();
            loop {
                match (lhs_vec.pop(), rhs_vec.pop()) {
                    (Some(x), Some(y)) => {
                        result_vec.push(calculate_numeric_operator(x, y, f));
                    }
                    (Some(_), None) => result_vec.push(types::Value::Error(types::Error::Argument)),
                    (None, Some(_)) => result_vec.push(types::Value::Error(types::Error::Argument)),
                    (None, None) => break,
                };
            }
            types::Value::Iterator(result_vec)
        }
        _ => types::Value::Error(types::Error::Value),
    }
}

fn add_days_to_date(d: DateTime<FixedOffset>, rhs: types::Value) -> types::Value {
    match rhs {
        types::Value::Number(x) => types::Value::Date(d + Duration::days(x as i64)),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn subtract_days_from_date(d: DateTime<FixedOffset>, rhs: types::Value) -> types::Value {
    match rhs {
        types::Value::Number(x) => types::Value::Date(d - Duration::days(x as i64)),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_numeric_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Text(t) => calcualte_numeric_operator_rhs_text(t, rhs, f),
        types::Value::Number(l) => calculate_numeric_operator_rhs_number(l, lhs, rhs, f),
        types::Value::Iterator(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => calculate_numeric_operator_rhs_number(0.0, lhs, rhs, f),
    }
}

fn calculate_numeric_product_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Text(t) => calcualte_numeric_operator_rhs_text(t, rhs, f),
        types::Value::Number(l) => calculate_numeric_operator_product_rhs_number(l, lhs, rhs, f),
        types::Value::Iterator(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => calculate_numeric_operator_product_rhs_number(1.0, lhs, rhs, f),
    }
}

fn calculate_average_operator_rhs_number(
    element_count: &mut i32,
    l: f32,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match rhs {
        types::Value::Boolean(_) => rhs,
        types::Value::Error(_) => rhs,
        types::Value::Text(t) => match t.parse::<f32>() {
            Ok(nr) => types::Value::Number(f(l, nr)),
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(r) => types::Value::Number(f(l, r)),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                match temp {
                    types::Value::Blank => *element_count -= 1,
                    _ => (),
                };
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top.clone(), f);
                    match top {
                        types::Value::Blank => (),
                        _ => *element_count += 1,
                    };
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => {
            *element_count -= 1;
            types::Value::Number(f(l, 0.0))
        }
    }
}

fn calculate_average_operator_rhs_iterator(
    element_count: &mut i32,
    mut lhs_vec: Vec<types::Value>,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match rhs {
        types::Value::Number(_) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                    *element_count += 1;
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_average_operator(
    element_count: &mut i32,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    match lhs {
        types::Value::Boolean(_) => lhs,
        types::Value::Error(_) => lhs,
        types::Value::Text(t) => calcualte_numeric_operator_rhs_text(t, rhs, f),
        types::Value::Number(l) => {
            calculate_average_operator_rhs_number(element_count, l, lhs, rhs, f)
        }
        types::Value::Iterator(lhs_vec) => {
            calculate_average_operator_rhs_iterator(element_count, lhs_vec, rhs, f)
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => {
            *element_count -= 1;
            calculate_average_operator_rhs_number(element_count, 0.0, lhs, rhs, f)
        }
    }
}

fn calculate_comparison_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> bool,
) -> types::Value {
    match lhs {
        types::Value::Text(l) => match rhs {
            types::Value::Text(r) => {
                if l.eq(&r) {
                    types::Value::Boolean(types::Boolean::True)
                } else {
                    types::Value::Boolean(types::Boolean::False)
                }
            }
            types::Value::Blank => {
                if l.is_empty() {
                    types::Value::Boolean(types::Boolean::True)
                } else {
                    types::Value::Boolean(types::Boolean::False)
                }
            }
            _ => types::Value::Error(types::Error::Value),
        },
        types::Value::Number(l) => match rhs {
            types::Value::Number(r) => {
                if f(l, r) {
                    types::Value::Boolean(types::Boolean::True)
                } else {
                    types::Value::Boolean(types::Boolean::False)
                }
            }
            types::Value::Blank => {
                if f(l, 0.0) {
                    types::Value::Boolean(types::Boolean::True)
                } else {
                    types::Value::Boolean(types::Boolean::False)
                }
            }
            _ => types::Value::Error(types::Error::Value),
        },
        types::Value::Blank => match rhs {
            types::Value::Number(r) => {
                if f(0.0, r) {
                    types::Value::Boolean(types::Boolean::True)
                } else {
                    types::Value::Boolean(types::Boolean::False)
                }
            }
            types::Value::Text(r) => {
                if r.is_empty() {
                    types::Value::Boolean(types::Boolean::True)
                } else {
                    types::Value::Boolean(types::Boolean::False)
                }
            }
            types::Value::Blank => types::Value::Boolean(types::Boolean::True),
            _ => types::Value::Error(types::Error::Value),
        },
        types::Value::Boolean(_) => types::Value::Error(types::Error::Value),
        types::Value::Error(_) => types::Value::Error(types::Error::Value),
        types::Value::Iterator(_) => types::Value::Error(types::Error::Value),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
    }
}

fn to_bool(value: types::Boolean) -> bool {
    match value {
        types::Boolean::True => true,
        types::Boolean::False => false,
    }
}

fn calculate_boolean_operator_rhs_boolean(
    l: types::Boolean,
    rh: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    match rh {
        types::Value::Boolean(r) => {
            if f(to_bool(l), to_bool(r)) {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f);
                }
                let rhs = cast_value_to_boolean(temp);
                match rhs {
                    types::Value::Boolean(r) => {
                        if f(to_bool(l), to_bool(r)) {
                            types::Value::Boolean(types::Boolean::True)
                        } else {
                            types::Value::Boolean(types::Boolean::False)
                        }
                    }
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Blank => {
            if to_bool(l) {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_error(rh: types::Value) -> types::Value {
    match rh {
        types::Value::Boolean(r) => {
            if to_bool(r) {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Error(_) => types::Value::Error(types::Error::Cast),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_iterator(
    rh: types::Value,
    mut lhs_vec: Vec<types::Value>,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    match rh {
        types::Value::Boolean(r) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f);
                }
                let lhs = cast_value_to_boolean(temp);
                match lhs {
                    types::Value::Boolean(l) => {
                        if f(to_bool(l), to_bool(r)) {
                            types::Value::Boolean(types::Boolean::True)
                        } else {
                            types::Value::Boolean(types::Boolean::False)
                        }
                    }
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }

        _ => types::Value::Error(types::Error::Value),
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
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f)
        }
        types::Value::Iterator(lhs_vec) => {
            calculate_boolean_operator_rhs_iterator(cast_value_to_boolean(rhs), lhs_vec, f)
        }
        types::Value::Blank => calculate_boolean_operator_rhs_boolean(
            types::Boolean::True,
            cast_value_to_boolean(rhs),
            f,
        ),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_boolean_operator_or(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    let lh = cast_value_to_boolean(lhs);
    match lh {
        types::Value::Boolean(l) => {
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f)
        }
        types::Value::Error(_) => calculate_boolean_operator_rhs_error(cast_value_to_boolean(rhs)),
        types::Value::Iterator(lhs_vec) => {
            calculate_boolean_operator_rhs_iterator(cast_value_to_boolean(rhs), lhs_vec, f)
        }
        types::Value::Blank => calculate_boolean_operator_rhs_boolean(
            types::Boolean::False,
            cast_value_to_boolean(rhs),
            f,
        ),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_boolean_operator_xor(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    let lh = cast_value_to_boolean(lhs);
    match lh {
        types::Value::Boolean(l) => {
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f)
        }
        types::Value::Error(_) => calculate_boolean_operator_rhs_error(cast_value_to_boolean(rhs)),
        types::Value::Iterator(lhs_vec) => {
            calculate_boolean_operator_rhs_iterator(cast_value_to_boolean(rhs), lhs_vec, f)
        }
        types::Value::Blank => calculate_boolean_operator_rhs_boolean(
            types::Boolean::False,
            cast_value_to_boolean(rhs),
            f,
        ),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_abs(value: types::Value) -> types::Value {
    match value {
        types::Value::Boolean(_) => value,
        types::Value::Error(_) => value,
        types::Value::Text(_) => value,
        types::Value::Number(l) => types::Value::Number(l.abs()),
        types::Value::Iterator(_) => types::Value::Error(types::Error::Value),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => types::Value::Number(0.0),
    }
}

fn calculate_negation(value: types::Value) -> types::Value {
    match value {
        types::Value::Boolean(l) => {
            if !(to_bool(l)) {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Error(_) => value,
        types::Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => {
                    if !(to_bool(l)) {
                        types::Value::Boolean(types::Boolean::True)
                    } else {
                        types::Value::Boolean(types::Boolean::False)
                    }
                }
                None => types::Value::Error(types::Error::Cast),
            }
        }
        types::Value::Number(l) => {
            if l == 0.0 {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Iterator(_) => types::Value::Error(types::Error::Value),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => types::Value::Boolean(types::Boolean::True),
    }
}

fn calculate_negate(value: types::Value) -> types::Value {
    match value {
        types::Value::Number(l) => types::Value::Number(-l),
        types::Value::Iterator(mut value_vec) => {
            let mut result_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                result_vec.push(calculate_negate(top));
            }
            types::Value::Iterator(result_vec)
        }
        types::Value::Blank => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    }
}

fn cast_text_to_boolean(s: &str) -> Option<types::Boolean> {
    if s.eq_ignore_ascii_case("TRUE") {
        Some(types::Boolean::True)
    } else if s.eq_ignore_ascii_case("FALSE") {
        Some(types::Boolean::False)
    } else {
        None
    }
}

fn cast_value_to_boolean(value: types::Value) -> types::Value {
    match value {
        types::Value::Boolean(_) => value,
        types::Value::Error(_) => value,
        types::Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => {
                    if to_bool(l) {
                        types::Value::Boolean(types::Boolean::True)
                    } else {
                        types::Value::Boolean(types::Boolean::False)
                    }
                }
                None => types::Value::Error(types::Error::Cast),
            }
        }
        types::Value::Number(l) => {
            if l != 0.0 {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Iterator(mut value_vec) => {
            let mut boolean_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                let value = cast_value_to_boolean(top);
                boolean_vec.push(value);
            }
            types::Value::Iterator(boolean_vec)
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Cast),
        types::Value::Blank => types::Value::Blank,
    }
}

fn convert_iterator_to_result(
    result: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    match result {
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f);
                }
                match cast_value_to_boolean(temp) {
                    types::Value::Boolean(bool_result) => {
                        if to_bool(bool_result) {
                            types::Value::Boolean(types::Boolean::True)
                        } else {
                            types::Value::Boolean(types::Boolean::False)
                        }
                    }
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        _ => result,
    }
}

fn convert_iterator_to_result_or(
    result: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    match result {
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator_or(temp, top, f);
                }
                match cast_value_to_boolean(temp) {
                    types::Value::Boolean(bool_result) => {
                        if to_bool(bool_result) {
                            types::Value::Boolean(types::Boolean::True)
                        } else {
                            types::Value::Boolean(types::Boolean::False)
                        }
                    }
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        _ => result,
    }
}

fn convert_iterator_to_result_xor(
    result: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    match result {
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator_xor(temp, top, f);
                }
                match cast_value_to_boolean(temp) {
                    types::Value::Boolean(bool_result) => {
                        if to_bool(bool_result) {
                            types::Value::Boolean(types::Boolean::True)
                        } else {
                            types::Value::Boolean(types::Boolean::False)
                        }
                    }
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        _ => result,
    }
}

fn get_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> (types::Value, types::Value) {
    (
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Error(types::Error::Argument),
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Error(types::Error::Argument),
        },
    )
}

fn get_value(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    }
}

fn get_date_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> (types::Value, types::Value) {
    (
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Error(types::Error::Argument),
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Error(types::Error::Argument),
        },
    )
}

fn get_number_and_string_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> (types::Value, types::Value) {
    if exp.values.len() == 1 {
        (
            types::Value::Number(1.0),
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => types::Value::Error(types::Error::Argument),
            },
        )
    } else {
        (
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => types::Value::Error(types::Error::Argument),
            },
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => types::Value::Error(types::Error::Argument),
            },
        )
    }
}

fn get_iff_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> (types::Value, types::Value, types::Value) {
    (
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Blank,
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Blank,
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Blank,
        },
    )
}

fn calculate_iterator(
    mut vec: Vec<types::Formula>,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    let mut value_vec = Vec::new();
    while let Some(top) = vec.pop() {
        value_vec.push(calculate_formula(top, f));
    }
    types::Value::Iterator(value_vec)
}

fn calculate_reference(
    string: String,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    match f {
        Some(f) => match f(string) {
            types::Value::Number(x) => types::Value::Number(x),
            types::Value::Text(s) => calculate_formula(
                parse_formula::parse_string_to_formula(&s, None::<NoCustomFunction>),
                Some(f),
            ),
            types::Value::Boolean(x) => types::Value::Boolean(x),
            types::Value::Error(types::Error::Value) => types::Value::Error(types::Error::Value),
            types::Value::Iterator(v) => types::Value::Iterator(v),
            types::Value::Date(d) => types::Value::Date(d),
            types::Value::Blank => types::Value::Blank,
            _ => types::Value::Error(types::Error::Reference),
        },
        None => types::Value::Error(types::Error::Reference),
    }
}

fn calculate_bool(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator(result, calculate_formula(top, f), f_bool);
    }
    convert_iterator_to_result(result, f_bool)
}

fn calculate_or(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator_or(result, calculate_formula(top, f), f_bool);
    }
    convert_iterator_to_result_or(result, f_bool)
}

fn calculate_xor(
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> types::Value {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator_xor(result, calculate_formula(top, f), f_bool);
    }
    convert_iterator_to_result_xor(result, f_bool)
}

fn calculate_collective_operator(
    mut collective_value: types::Value,
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
    f_collective: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    while let Some(top) = exp.values.pop() {
        collective_value =
            calculate_numeric_operator(collective_value, calculate_formula(top, f), f_collective);
    }
    collective_value
}

fn calculate_collective_product_operator(
    mut collective_value: types::Value,
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
    f_collective: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    while let Some(top) = exp.values.pop() {
        collective_value = calculate_numeric_product_operator(
            collective_value,
            calculate_formula(top, f),
            f_collective,
        );
    }
    match collective_value {
        types::Value::Blank => types::Value::Number(0.0),
        _ => collective_value,
    }
}

fn calculate_average(
    mut collective_value: types::Value,
    mut exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
    f_collective: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    let mut element_count = 0;
    while let Some(top) = exp.values.pop() {
        element_count += 1;
        collective_value = calculate_average_operator(
            &mut element_count,
            collective_value,
            calculate_formula(top, f),
            f_collective,
        );
    }
    if element_count == 0 {
        types::Value::Error(types::Error::Div0)
    } else {
        calculate_numeric_operator(
            collective_value,
            types::Value::Number(element_count as f32),
            calculate_divide_operator,
        )
    }
}

fn calculate_days(date_pair: (types::Value, types::Value)) -> types::Value {
    let begin_of_date: DateTime<FixedOffset> =
        DateTime::parse_from_rfc3339("1900-01-01T02:00:00.000Z")
            .ok()
            .unwrap();
    let (start, end) = date_pair;
    match (start, end) {
        (types::Value::Date(start), types::Value::Date(end)) => {
            types::Value::Number((end - start).num_days() as f32)
        }
        (types::Value::Blank, types::Value::Date(end)) => {
            types::Value::Number((end - begin_of_date).num_days() as f32)
        }
        (types::Value::Date(start), types::Value::Blank) => {
            types::Value::Number((begin_of_date - start).num_days() as f32)
        }
        (types::Value::Blank, types::Value::Blank) => types::Value::Number(0.0),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_right(number_string: (types::Value, types::Value)) -> types::Value {
    let (number, string) = number_string;
    let trim_length = match number {
        types::Value::Number(x) => x as usize,
        _ => 0,
    };

    let trimmed_string = match string {
        types::Value::Text(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            &temp[(temp.len() - trim_length)..]
        }
        _ => "",
    };
    types::Value::Text(trimmed_string.to_string())
}

fn calculate_left(number_string: (types::Value, types::Value)) -> types::Value {
    let (number, string) = number_string;
    let trim_length = match number {
        types::Value::Number(x) => x as usize,
        _ => 0,
    };

    let trimmed_string = match string {
        types::Value::Text(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            &temp[..trim_length]
        }
        _ => "",
    };
    types::Value::Text(trimmed_string.to_string())
}

fn calculate_iff(iff_arguments: (types::Value, types::Value, types::Value)) -> types::Value {
    let (false_value, true_value, bool_expression) = iff_arguments;
    match bool_expression {
        types::Value::Boolean(bool_value) => {
            if to_bool(bool_value) {
                true_value
            } else {
                false_value
            }
        }
        types::Value::Number(number_value) => {
            if number_value == 0.0 {
                false_value
            } else {
                true_value
            }
        }
        types::Value::Blank => false_value,
        types::Value::Error(_) => bool_expression,
        types::Value::Text(_) => types::Value::Error(types::Error::Value),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_isblank(value: types::Value) -> types::Value {
    match value {
        types::Value::Blank => types::Value::Boolean(types::Boolean::True),
        types::Value::Text(s) => {
            if s.is_empty() {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Error(types::Error::Value) => types::Value::Boolean(types::Boolean::True),
        types::Value::Error(types::Error::Reference) => types::Value::Boolean(types::Boolean::True),
        
        _ => types::Value::Boolean(types::Boolean::False),
        
    }
}

fn calculate_function(
    func: types::Function,
    exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    match func {
        types::Function::Abs => calculate_abs(get_value(exp, f)),
        types::Function::Sum => {
            calculate_collective_operator(types::Value::Number(0.0), exp, f, |n1, n2| n1 + n2)
        }
        types::Function::Product => {
            calculate_collective_product_operator(types::Value::Blank, exp, f, |n1, n2| n1 * n2)
        }
        types::Function::Average => {
            calculate_average(types::Value::Number(0.00), exp, f, |n1, n2| n1 + n2)
        }
        types::Function::Or => calculate_or(exp, f, |n1, n2| n1 || n2),
        types::Function::And => calculate_bool(exp, f, |n1, n2| n1 && n2),
        types::Function::Xor => calculate_xor(exp, f, |n1, n2| n1 ^ n2),
        types::Function::Not => calculate_negation(get_value(exp, f)),
        types::Function::Negate => calculate_negate(get_value(exp, f)),
        types::Function::Days => calculate_days(get_date_values(exp, f)),
        types::Function::Right => calculate_right(get_number_and_string_values(exp, f)),
        types::Function::Left => calculate_left(get_number_and_string_values(exp, f)),
        types::Function::Iff => calculate_iff(get_iff_values(exp, f)),
        types::Function::IsBlank => calculate_isblank(get_value(exp, f)),
    }
}

fn calculate_operation(
    exp: types::Expression,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    match exp.op {
        types::Operator::Plus => {
            let (value2, value1) = get_values(exp, f);
            match value1 {
                types::Value::Date(d) => add_days_to_date(d, value2),
                _ => calculate_numeric_operator(value1, value2, |n1, n2| n1 + n2),
            }
        }

        types::Operator::Minus => {
            let (value2, value1) = get_values(exp, f);
            match value1 {
                types::Value::Date(d) => subtract_days_from_date(d, value2),
                _ => calculate_numeric_operator(value1, value2, |n1, n2| n1 - n2),
            }
        }

        types::Operator::Multiply => {
            let (value2, value1) = get_values(exp, f);
            calculate_numeric_operator(value1, value2, |n1, n2| n1 * n2)
        }
        types::Operator::Divide => {
            let (value2, value1) = get_values(exp, f);
            match value2 {
                types::Value::Number(x) if x == 0.0 => types::Value::Error(types::Error::Div0),
                _ => calculate_numeric_operator(value1, value2, calculate_divide_operator),
            }
        }
        types::Operator::Power => {
            let (value2, value1) = get_values(exp, f);
            calculate_numeric_operator(value1, value2, calculate_power_operator)
        }
        types::Operator::Concat => {
            let (value2, value1) = get_values(exp, f);
            calculate_string_operator(value1, value2, calculate_concat_operator)
        }
        types::Operator::Equal => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 == d2)
                }
                (types::Value::Text(l), types::Value::Text(r)) => {
                    compare_strings(l, r, |s1, s2| s1 == s2)
                }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| (n1 - n2).abs() == 0.0),
            }
        }
        types::Operator::NotEqual => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 != d2)
                }
                (types::Value::Text(l), types::Value::Text(r)) => {
                    compare_strings(l, r, |s1, s2| s1 != s2)
                }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| (n1 - n2).abs() > 0.0),
            }
        }
        types::Operator::Greater => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 > d2)
                }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 > n2),
            }
        }
        types::Operator::Less => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 < d2)
                }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 < n2),
            }
        }
        types::Operator::GreaterOrEqual => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 >= d2)
                }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 >= n2),
            }
        }
        types::Operator::LessOrEqual => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 <= d2)
                }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 <= n2),
            }
        }
        types::Operator::Function(func) => calculate_function(func, exp, f),
    }
}

fn compare_dates(
    date1: DateTime<FixedOffset>,
    date2: DateTime<FixedOffset>,
    f: fn(d1: DateTime<FixedOffset>, d2: DateTime<FixedOffset>) -> bool,
) -> types::Value {
    if f(date1, date2) {
        types::Value::Boolean(types::Boolean::True)
    } else {
        types::Value::Boolean(types::Boolean::False)
    }
}

fn compare_strings(
    string1: String,
    string2: String,
    f: fn(s1: String, s2: String) -> bool,
) -> types::Value {
    if f(string1, string2) {
        types::Value::Boolean(types::Boolean::True)
    } else {
        types::Value::Boolean(types::Boolean::False)
    }
}

/// Evaluates a string that was parsed and stored in Expression Struct.
/// Takes an optional closure with the trait bound Fn(String) -> types::Value.
pub fn calculate_formula(
    formula: types::Formula,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    match formula {
        types::Formula::Operation(exp) => calculate_operation(exp, f),
        types::Formula::Value(val) => val,
        types::Formula::Reference(string) => calculate_reference(string, f),
        types::Formula::Iterator(vec) => calculate_iterator(vec, f),
    }
}

/// Converts a result from Value Enum to a printable string.
pub fn result_to_string(_value: types::Value) -> String {
    match _value {
        types::Value::Number(number) => show_number(number),
        types::Value::Text(text) => text,
        types::Value::Error(error) => show_error(error),
        types::Value::Boolean(boolean) => show_boolean(boolean),
        types::Value::Iterator(value_vec) => show_iterator(value_vec),
        types::Value::Date(date) => date.to_string(),
        types::Value::Blank => show_blank(),
    }
}

fn show_number(number: f32) -> String {
    if number.is_infinite() {
        String::from("#DIV/0!")
    } else {
        number.to_string()
    }
}

fn show_error(error: types::Error) -> String {
    match error {
        types::Error::Div0 => String::from("#DIV/0!"),
        types::Error::Cast => String::from("#CAST!"),
        types::Error::Parse => String::from("#PARSE!"),
        types::Error::Value => String::from("#VALUE!"),
        types::Error::Argument => String::from("#ARG!"),
        types::Error::Reference => String::from("#REF!"),
    }
}

fn show_boolean(boolean: types::Boolean) -> String {
    match boolean {
        types::Boolean::True => String::from("TRUE"),
        types::Boolean::False => String::from("FALSE"),
    }
}

fn show_iterator(mut value_vec: Vec<types::Value>) -> String {
    value_vec.reverse();
    let mut result = '{'.to_string();
    while let Some(top) = value_vec.pop() {
        result = result + &result_to_string(top);
        result = result + &','.to_string();
    }
    result = result.trim_end_matches(',').to_string();
    result = result + &'}'.to_string();
    result
}

fn show_blank() -> String {
    show_number(0.0)
    //String::from("BLANK")
}

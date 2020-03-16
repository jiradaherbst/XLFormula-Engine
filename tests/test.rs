extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn evaluate_formula(s: &str) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s);
    let result = calculate::calculate_formula(formula);
    let number = calculate::result_to_string(result).parse::<f32>().unwrap();
    number
}

#[test]
fn it_evaluate_add_operator_simple_addition() {
    assert_eq!(evaluate_formula(&"=1+2"), 3.0,);
}
#[test]
fn it_evaluate_add_operator_spaces_between_operators() {
    assert_eq!(evaluate_formula(&"=1 +  2"), 3.0,);
}

#[test]
fn it_evaluate_add_operator_spaces_before_number() {
    assert_eq!(evaluate_formula(&"=  1+2"), 3.0,);
}

#[test]
fn it_evaluate_add_operator_with_large_numbers() {
    assert_eq!(evaluate_formula(&"=1234567890 + 1234567890"), 2469135780.0);
}

#[test]
fn it_evaluate_add_operator_with_negative_numbers() {
    assert_eq!(evaluate_formula(&"=-1 + -2"), -3.0);
}

#[test]
fn it_evaluate_minus_operator1() {
    assert_eq!(evaluate_formula(&"=123 - 23"), 100.0,);
}

#[test]
fn it_evaluate_minus_operator_with_negative_numbers() {
    assert_eq!(evaluate_formula(&"=-12--6"), -6.0,);
}

#[test]
fn it_evaluate_multiply_operator() {
    assert_eq!(evaluate_formula(&"=3 * 2"), 6.0,);
}

#[test]
fn it_evaluate_divide_operator() {
    assert_eq!(evaluate_formula(&"=6 / 3"), 2.0,);
}

#[test]
fn it_evaluate_negative() {
    assert_eq!(evaluate_formula(&"=-1 * -5"), 5.0,);
}

#[test]
fn it_evaluate_power_int() {
    assert_eq!(evaluate_formula(&"=2^3"), 8.0,);
}

// #[test]
// fn it_evaluate_power_float() {
//     assert_eq!(evaluate_formula(&"=4^0.5"), 2.0,);
// }

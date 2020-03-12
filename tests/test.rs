extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn evaluate_formula(op: String, s: &str) -> f32 {
    let formula = parse_formula::parse_string_to_formula(op, s);
    let result = calculate::calculate_formula(formula);
    let number = calculate::result_to_string(result).parse::<f32>().unwrap();
    number
}

#[test]
fn it_evaluate_add_operator1() {
    assert_eq!(evaluate_formula("sum".to_string(), &"1 + 2"), 3.0,);
}

#[test]
fn it_evaluate_minus_operator1() {
    assert_eq!(evaluate_formula("minus".to_string(), &"123 - 23"), 100.0,);
}

#[test]
fn it_evaluate_add_operator2() {
    assert_eq!(evaluate_formula("sum".to_string(), &"15 + 25"), 40.0,);
}

#[test]
fn it_evaluate_minus_operator2() {
    assert_eq!(evaluate_formula("minus".to_string(), &"12 - 6"), 6.0,);
}

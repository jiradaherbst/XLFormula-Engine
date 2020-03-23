extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

use assert_approx_eq::assert_approx_eq;

fn evaluate_formula_number(s: &str) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s);
    let result = calculate::calculate_formula(formula);
    calculate::result_to_string(result).parse::<f32>().unwrap()
}

fn evaluate_formula_string(s: &str) -> String {
    let formula = parse_formula::parse_string_to_formula(s);
    let result = calculate::calculate_formula(formula);
    calculate::result_to_string(result)
}

#[test]
fn it_evaluate_add_operator_simple_addition() {
    assert_eq!(evaluate_formula_number(&"=1+2"), 3.0,);
}

#[test]
fn it_evaluate_add_operator_spaces_between_operators() {
    assert_eq!(evaluate_formula_number(&"=1 +  2"), 3.0,);
}

#[test]
fn it_evaluate_add_operator_spaces_before_number() {
    assert_eq!(evaluate_formula_number(&"=  1+2"), 3.0,);
}

#[test]
fn it_evaluate_add_operator_with_large_numbers() {
    assert_eq!(
        evaluate_formula_number(&"=1234567890 + 1234567890"),
        2469135780.0
    );
}

#[test]
fn it_evaluate_add_operator_with_negative_numbers() {
    assert_eq!(evaluate_formula_number(&"=-1 + -2"), -3.0);
}

#[test]
fn it_evaluate_minus_operator1() {
    assert_eq!(evaluate_formula_number(&"=123 - 23"), 100.0,);
}

#[test]
fn it_evaluate_minus_operator_with_negative_numbers() {
    assert_eq!(evaluate_formula_number(&"=-12--6"), -6.0,);
}

#[test]
fn it_evaluate_multiply_operator() {
    assert_eq!(evaluate_formula_number(&"=3 * 2"), 6.0,);
}

#[test]
fn it_evaluate_divide_operator() {
    assert_eq!(evaluate_formula_number(&"=6 / 3"), 2.0,);
}
#[test]
fn it_evaluate_divide_operator_divsion_by_zero() {
    assert_eq!(evaluate_formula_string(&"=6 / 0"), "#DIV/0!");
}
#[test]
fn it_evaluate_negative() {
    assert_eq!(evaluate_formula_number(&"=-1 * -5"), 5.0,);
}

#[test]
fn it_evaluate_power_int() {
    assert_eq!(evaluate_formula_number(&"=2^3"), 8.0,);
}

#[test]
fn it_evaluate_float() {
    assert_eq!(evaluate_formula_number(&"=1.2+0.5"), 1.7,);
}

#[test]
fn it_evaluate_negative_float() {
    assert_approx_eq!(evaluate_formula_number(&"=-1.2+0.5"), -0.7); // left: `-0.70000005`, right: `-0.7`'
}

#[test]
fn it_evaluate_power_float() {
    assert_eq!(evaluate_formula_number(&"=4^0.5"), 2.0,);
}

#[test]
fn it_evaluate_multiple_operations() {
    assert_eq!(evaluate_formula_number(&"=1+2+3"), 6.0,);
}

#[test]
fn it_evaluate_multiple_operations2() {
    assert_eq!(evaluate_formula_number(&"=1+2-3"), 0.0,);
}
#[test]
fn it_evaluate_multiple_operations_in_right_order() {
    assert_eq!(evaluate_formula_number(&"=1+2*3"), 7.0,);
}
#[test]
fn it_evaluate_multiple_operations_in_right_order2() {
    assert_eq!(evaluate_formula_number(&"=1+3/3"), 2.0,);
}
#[test]
fn it_evaluate_multiple_operations_with_errors() {
    assert_eq!(evaluate_formula_string(&"=1+3/0"), "#DIV/0!",);
}

#[test]
fn it_evaluate_parens() {
    assert_eq!(evaluate_formula_number(&"=(1+2)"), 3.0,);
}

#[test]
fn it_evaluate_multiple_parens() {
    assert_eq!(evaluate_formula_number(&"=(1+2)+(3+4)"), 10.0,);
}

#[test]
fn it_evaluate_nested_parens() {
    assert_eq!(evaluate_formula_number(&"=(1*(2+3))*2"), 10.0,);
}

#[test]
fn it_evaluate_strings() {
    assert_eq!(evaluate_formula_string(&"=\"Hello\""), "Hello",);
}

#[test]
fn it_evaluate_strings_in_numeric_operator() {
    assert_eq!(evaluate_formula_string(&"=\"Hello\"+1"), "#CAST!",);
}

#[test]
fn it_evaluate_strings_in_numeric_operator2() {
    assert_eq!(evaluate_formula_string(&"=1 + \"Hello\""), "#CAST!",);
}

// #[test]
// fn it_evaluate_concat_operator() {
//     assert_eq!(evaluate_formula_string(&"=\"Hello \"&\"World!\""), "Hello World!",);
// }

// #[test]
// fn it_evaluate_concat_operator_with_casting() {
//     assert_eq!(evaluate_formula_string(&"=\"Hello\"&1"), "Hello1",);
// }

// #[test]
// fn it_evaluate_concat_operator_with_casting2() {
//     assert_eq!(evaluate_formula_string(&"=\"Hello\"&1.2"), "Hello1.2",);
//}
// #[test]
// fn it_support_basic_math_function() {
//     assert_eq!(evaluate_formula_number(&"=ABS(-1)"), 1.0,);
// }

// #[test]
// fn it_evaluate_wrong_parens() {
//     assert_eq!(evaluate_formula_string(&"=(2+3"), "#PARENS!",);
// }

// #[test]
// fn it_evaluate_functions() {
//     assert_eq!(evaluate_formula_number(&"=SUM(1, 2, 3, 4)"), 10.0,);
// }

// #[test]
// fn it_evaluate_references() {
//     assert_eq!(evaluate_formula_number(&"=A+B", {A: 1, B: 2}), 3.0,);
// }

// #[test]
// fn it_evaluate_references_other_formulas() {
//     assert_eq!(evaluate_formula_number(&"=A+B", {A: &"=1+B", B: 3}), 7.0);
// }

// #[test]
// fn it_evaluate_functions_and_operators_differently() {
//     assert_eq!(evaluate_formula_string(&"=1+2+""3"""}),"Error.. ");
//     assert_eq!(evaluate_formula_number(&"=SUM(1,2,""3"")"}),3.0);
// }

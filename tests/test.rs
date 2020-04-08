extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;
use calculator::types;

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

fn evaluate_formula_number_with_reference(
    s: &str,
    f: Option<fn(s: String) -> types::Value>,
) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s);
    let result = calculate::calculate_formula_with_reference(formula, f);
    calculate::result_to_string(result).parse::<f32>().unwrap()
}

/////////////////// Simple math operators with floats and integer ///////////////////
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

/////////////////// Strings ///////////////////
#[test]
fn it_evaluate_strings() {
    assert_eq!(evaluate_formula_string(&"=\"Hello!  \""), "Hello!  ",);
}

#[test]
fn it_evaluate_strings_in_numeric_operator() {
    assert_eq!(evaluate_formula_string(&"=\"Hello\"+1"), "#CAST!",);
}

#[test]
fn it_evaluate_strings_in_numeric_operator2() {
    assert_eq!(evaluate_formula_string(&"=1 + \"Hello\""), "#CAST!",);
}

#[test]
fn it_evaluate_concat_operator1() {
    assert_eq!(
        evaluate_formula_string(&"=\"Hello\" & \"World!\""),
        "HelloWorld!",
    );
}

#[test]
fn it_evaluate_concat_operator3() {
    assert_eq!(
        evaluate_formula_string(&"=\"Hello \" & \" World!\""),
        "Hello  World!",
    );
}

#[test]
fn it_evaluate_concat_operator_with_casting() {
    assert_eq!(evaluate_formula_string(&"=\"Hello\"&1"), "Hello1",);
}

#[test]
fn it_evaluate_concat_operator_with_casting2() {
    assert_eq!(evaluate_formula_string(&"=\"Hello \"&1.2"), "Hello 1.2",);
}

#[test]
fn it_evaluate_concat_operator_with_numberic() {
    assert_eq!(evaluate_formula_string(&"=1   &  2"), "12",);
}

#[test]
fn it_evaluate_strings_with_quoted_quotes1() {
    assert_eq!(
        evaluate_formula_string(&"=\"Hello 'World'\""),
        "Hello 'World'",
    );
}

#[test]
fn it_evaluate_strings_with_quoted_quotes() {
    assert_eq!(
        evaluate_formula_string(&"=\"Hello \"\"World\"\"\""),
        "Hello \"World\"",
    );
}

// ="Hello ""World""" -> Hello "World" (required)
// ='Hello "World"' (optional!)
// ='Hello \'World''' (optional!) <--- not valid in JS and also this ---> =''Hello' 'World''
// ="Hello 'World'"

#[test]
fn it_evaluate_strings_with_single_quotes() {
    assert_eq!(
        evaluate_formula_string(&"=\"Hello \"&'World'"),
        "Hello World",
    );
}

#[test]
fn it_evaluate_strings_with_quotes() {
    assert_eq!(
        evaluate_formula_string(&"='Hello \"World\"'"),
        "Hello \"World\"",
    );
}

#[test]
fn it_evaluate_strings_with_quotes2() {
    assert_eq!(
        evaluate_formula_string(&"='Hello'"), // '& 'World'
        "Hello",
    );
}

/////////////////// Constants  ///////////////////
#[test]
fn it_evaluate_constant_number() {
    assert_eq!(evaluate_formula_number(&"1"), 1.0,);
}

#[test]
fn it_evaluate_constant_number_float() {
    assert_eq!(evaluate_formula_number(&"1.2"), 1.2,);
}

#[test]
fn it_evaluate_constant_text() {
    assert_eq!(evaluate_formula_string(&"Hello World"), "Hello World",);
}

#[test]
fn it_evaluate_constant_text_with_quotes() {
    assert_eq!(evaluate_formula_string(&"Hello \"World'"), "Hello \"World'",);
}

// // 01309 -> 1309
// // '01309 -> 01309
// // '=1+2 -> =1+2

#[test]
fn it_evaluate_constant_starting_with_equal() {
    assert_eq!(evaluate_formula_string(&"'="), "=",);
    assert_eq!(evaluate_formula_string(&"'=hello"), "=hello",);
}

/////////////////// Formulas ///////////////////
#[test]
fn it_support_basic_math_function() {
    assert_eq!(evaluate_formula_number(&"=ABS(-1)"), 1.0,);
}

#[test]
fn it_support_basic_math_function_with_nested_formulas() {
    assert_eq!(evaluate_formula_number(&"=ABS(-1-4)"), 5.0,);
}

#[test]
fn it_support_basic_math_function_with_nested_functions() {
    assert_eq!(evaluate_formula_number(&"=ABS(ABS(-1))"), 1.0,);
}

#[test]
fn it_evaluate_functions_sum() {
    assert_eq!(
        evaluate_formula_number(&"=SUM(1*1, ABS(2), ABS(2+1), 4)"),
        10.0,
    );
}

#[test]
fn it_evaluate_functions_product() {
    assert_eq!(
        evaluate_formula_number(&"=PRODUCT(ABS(1),2*1, 3,4*1)"),
        24.0,
    );
}

#[test]
fn it_evaluate_operators_with_casting() {
    assert_eq!(evaluate_formula_number(&"=\"1\"+2+\"3\""), 6.0,);
}

#[test]
fn it_evaluate_functions_with_casting() {
    assert_eq!(evaluate_formula_number(&"=SUM(1,2,\"3\")"), 6.0,);
}

/////////////////////// Parse error //////////////////////////////////
#[test]
fn it_evaluate_wrong_parens1() {
    assert_eq!(evaluate_formula_string(&"=(2+3"), "#PARSE!",);
    assert_eq!(evaluate_formula_string(&"=\"Hello World"), "#PARSE!",);
    assert_eq!(evaluate_formula_string(&"=Hello World"), "#PARSE!",);
}

//////////////////////////// Boolean //////////////////////////////////
#[test]
fn it_evaluate_comparison_operators() {
    assert_eq!(evaluate_formula_string(&"=1*1=1/1"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=1^1<>1"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=1*2>1"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=1*1/1+2<1^1"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=2>=1"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=11<=3"), "FALSE",);
}

#[test]
fn it_evaluate_boolean_or() {
    assert_eq!(evaluate_formula_string(&"=OR(1>1,1<>1)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=OR(1=1,2<=4)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=OR(\"True\")"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=OR(True)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=OR(1)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=OR(\"test\")"), "#CAST!",);
    assert_eq!(
        evaluate_formula_string(&"=OR(\"false\",\"FALSE\", 1, FALSE)"),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string(&"=OR(\"True\",1,\"test\",  true) "),
        "TRUE",
    );
}

#[test]
fn it_evaluate_boolean_and() {
    assert_eq!(evaluate_formula_string(&"=AND(1>1,1=1)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=AND(1=1,2<=4)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=AND(\"true\", 0)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=AND(\"True\")"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=AND(false)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=AND(1)"), "TRUE",);
    assert_eq!(
        evaluate_formula_string(&"=AND(\"test\", \"test\")"),
        "#CAST!",
    );
    assert_eq!(
        evaluate_formula_string(&"=AND(\"test\",\"True\", 1, true) "),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string(&"=AND(\"True\",\"test\", 1, true) "),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string(&"=AND(\"True\", 1, true, \"test\")"),
        "TRUE",
    );
}

#[test]
fn it_evaluate_boolean_xor() {
    assert_eq!(evaluate_formula_string(&"=XOR(2=2,1=1)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=XOR(1=1,2>4)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=XOR(\"True\")"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=XOR(False)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=XOR(1)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=XOR(1=1,\"test\")"), "TRUE",);
    assert_eq!(
        evaluate_formula_string(&"=XOR(TRUE, TRUE, TRUE, TRUE)"),
        "FALSE"
    );
    assert_eq!(evaluate_formula_string(&"=XOR(TRUE, TRUE)"), "FALSE");
    assert_eq!(
        evaluate_formula_string(&"=XOR(false, FALSE, \"false\", false)"),
        "FALSE"
    );
    assert_eq!(evaluate_formula_string(&"=XOR(true)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=XOR(false)"), "FALSE",);
}

#[test]
fn it_evaluate_boolean_not() {
    assert_eq!(evaluate_formula_string(&"=NOT(11<=3)"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=NOT(1=1)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=NOT(True)"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=NOT(\"false\")"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=NOT(\"test\")"), "#CAST!",);
    assert_eq!(evaluate_formula_string(&"=NOT(0)"), "TRUE",);
}

//////////////////////////// References //////////////////////////////////
#[test]
fn it_evaluate_references() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(1.0),
        "B" => types::Value::Number(2.0),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A+B", Some(data_function)),
        3.0,
    );
    assert_eq!(evaluate_formula_number_with_reference(&"=1+2", None), 3.0,);
}

// #[test]
// fn it_evaluate_references() {
//     let data_function = |s: String| match s.as_str() {
//         "A" => types::Value::Number(1.0),
//         "B" => types::Value::Number(2.0),
//         _ => types::Value::Error(types::Error::Value),
//     };
//     assert_eq!(
//         evaluate_formula_number_with_reference(&"=B+A", dataFunction),
//         3.0,
//     );
// }

// #[test]
// fn it_evaluate_references_other_formulas() {
//     assert_eq!(evaluate_formula_number(&"=A+B", {A: &"=1+B", B: 3}), 7.0);
// }

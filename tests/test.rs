extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use xlformula_engine::NoCustomFunction;
use xlformula_engine::NoReference;

use chrono::format::ParseError;
use chrono::{DateTime, Duration, FixedOffset};

use assert_approx_eq::assert_approx_eq;

fn evaluate_formula_number(s: &str) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    calculate::result_to_string(result).parse::<f32>().unwrap()
}

fn evaluate_formula_string(s: &str) -> String {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    calculate::result_to_string(result)
}

fn evaluate_formula_string_with_reference(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value>,
) -> String {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result)
}

fn evaluate_formula_number_with_reference(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value>,
) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result).parse::<f32>().unwrap()
}

fn evaluate_formula_number_with_reference_no_conversion(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value>,
) -> types::Value {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, f);
    result
    //calculate::result_to_string(result).parse::<f32>().unwrap()
}

fn evaluate_formula_boolean_with_reference(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value>,
) -> String {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result) //.parse::<f32>().unwrap()
}

fn evaluate_formula_date_with_reference(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value>,
) -> String {
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result)
}

fn evaluate_formula_number_with_custom_function(
    s: &str,
    custom_function: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
    //reference: Option<&impl Fn(String) -> types::Value>,
) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s, custom_function);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    calculate::result_to_string(result).parse::<f32>().unwrap()
}

fn evaluate_formula_string_with_custom_function(
    s: &str,
    custom_function: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
    //reference: Option<&impl Fn(String) -> types::Value>,
) -> String {
    let formula = parse_formula::parse_string_to_formula(s, custom_function);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    calculate::result_to_string(result)
}

fn _evaluate_formula_number_with_custom_function_and_reference(
    s: &str,
    custom_function: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
    reference: Option<&impl Fn(String) -> types::Value>,
) -> f32 {
    let formula = parse_formula::parse_string_to_formula(s, custom_function);
    let result = calculate::calculate_formula(formula, reference);
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
    assert_eq!(evaluate_formula_number(&"=SUM(1, 2, , 3)"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=SUM( 1 , 2,,3,)"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=SUM( 1 , 2,,,3,)"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=SUM(,)"), 0.0,);
}

#[test]
fn it_evaluate_functions_avg() {
    assert_eq!(evaluate_formula_number(&"=AVERAGE(1,2)"), 1.5,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE({1,2,3})"), 2.0,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE({1,2,3},1,2,3)"), 2.0,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE(3,1,2,3)"), 2.25,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE(1,2,3,4,5,1,2,3)"), 2.625,);
    assert_eq!(
        evaluate_formula_number(&"=AVERAGE({1,2,3,4,5},1,2,3)"),
        2.625,
    );
    assert_eq!(
        evaluate_formula_number(&"=AVERAGE(AVERAGE({1,2,3,4,5}),1,2,3)"),
        2.25,
    );
    assert_eq!(evaluate_formula_number(&"=AVERAGE({100,200})"), 150.0,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE( 1 , 2,,3)"), 1.5,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE( 1 , )"), 0.5,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE(,)"), 0.0,);
    assert_eq!(evaluate_formula_number(&"=AVERAGE(1,,2,3,)",), 1.2);
    //assert_eq!(evaluate_formula_number(&"=AVERAGE({{100,200}})"), 150.0);
}

#[test]
fn it_evaluate_functions_product() {
    assert_eq!(
        evaluate_formula_number(&"=PRODUCT(ABS(1),2*1, 3,4*1)"),
        24.0,
    );
    assert_eq!(evaluate_formula_number(&"=PRODUCT( 1 , 2,,3)"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=PRODUCT(,)"), 0.0,);
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
    assert_eq!(evaluate_formula_string(&"=OR(1, )"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=OR(1,,, )"), "TRUE",);
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
    assert_eq!(evaluate_formula_string(&"=AND(1, )"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=AND(1,,,1)"), "FALSE",);
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
    assert_eq!(evaluate_formula_string(&"=XOR(1, )"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=XOR(1,, )"), "TRUE",);
    assert_eq!(evaluate_formula_string(&"=XOR(1,,1 )"), "FALSE",);
    assert_eq!(evaluate_formula_string(&"=XOR(1,,1, )"), "FALSE",);
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
        "C" => types::Value::Number(3.0),
        "fix_rate" => types::Value::Number(10.0),
        "input." => types::Value::Number(2.0),
        "D" => types::Value::Number(1.0),
        "F" => types::Value::Text("=D+1".to_string()),
        "G" => types::Value::Text("=F+1+D+1".to_string()),
        "Test" => {
            let mut vec = Vec::new();
            vec.push(types::Value::Number(100.0));
            vec.push(types::Value::Number(200.0));
            vec.push(types::Value::Number(300.0));
            types::Value::Iterator(vec)
        }

        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A+B", Some(&data_function)),
        3.0,
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=(A*(B+C))*B", Some(&data_function)),
        10.0,
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=fix_rate*input.", Some(&data_function)),
        20.0,
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=D+F", Some(&data_function)),
        3.0,
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=G+F", Some(&data_function)),
        7.0,
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(A,B,C)", Some(&data_function)),
        6.0,
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE(Test)", Some(&data_function)),
        200.0,
    );
}

#[test]
fn it_evaluate_references_other_formulas() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Text("=1+B".to_string()),
        "B" => types::Value::Number(3.0),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A+B", Some(&data_function)),
        7.0
    );
}

#[test]
fn it_evaluate_references_boolean_formulas() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Boolean(types::Boolean::True),
        "B" => types::Value::Boolean(types::Boolean::False),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND(A,B)", Some(&data_function)),
        "FALSE"
    );
}

#[test]
fn it_evaluate_references_error_value_formulas() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Boolean(types::Boolean::True),
        "B" => types::Value::Error(types::Error::Value), //types::Value::Boolean(types::Boolean::False),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND(A,B)", Some(&data_function)),
        "TRUE"
    );
}

#[test]
fn it_evaluate_references_with_dot() {
    let data_function = |s: String| match s.as_str() {
        "A.B" => types::Value::Number(1.0),
        "B.C" => types::Value::Number(2.0),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A.B+B.C", Some(&data_function)),
        3.0,
    );
}

#[test]
fn it_evaluate_iterators() {
    assert_eq!(evaluate_formula_number(&"=SUM({1,2,3})"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=PRODUCT({1,2,3})"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=SUM({1,2,3}, {5,6})"), 17.0,);
}

#[test]
fn it_evaluate_iterators_and_scalars() {
    assert_eq!(evaluate_formula_number(&"=SUM({1,2,3}, 4)"), 10.0,);
    assert_eq!(evaluate_formula_number(&"=PRODUCT({1,2,3}, 4)"), 24.0,);
}

#[test]
fn it_evaluate_multiple_iterators_and_scalars() {
    assert_eq!(evaluate_formula_number(&"=SUM({1,2,3})"), 6.0,);
    assert_eq!(evaluate_formula_number(&"=SUM({  1,2,3}, 4, {5, 6})"), 21.0,);
    assert_eq!(evaluate_formula_number(&"=SUM({1,2,3},4,{5,6})"), 21.0,);
    assert_eq!(
        evaluate_formula_number(&"=SUM({1+1,2,3-3},4,{5,6*1})"),
        19.0,
    );
    assert_eq!(
        evaluate_formula_number(&"=PRODUCT({1+1,2,3-2},4, {5,6*1})"),
        480.0,
    );
}

#[test]
fn it_evaluate_references_iterator() {
    let data_function = |s: String| match s.as_str() {
        "A.B" => types::Value::Number(1.0),
        "B.C" => types::Value::Number(2.0),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM({A.B,B.C})", Some(&data_function)),
        3.0,
    );
}

#[test]
fn it_evaluate_iterator_operators() {
    assert_eq!(evaluate_formula_string(&"={1,2,3}+{1,2,3}"), "{2,4,6}");
    assert_eq!(evaluate_formula_string(&"={3,2,1}-{1,2,3}"), "{2,0,-2}");
    assert_eq!(evaluate_formula_string(&"={1,2,3}*{1,2,3}"), "{1,4,9}");
    assert_eq!(evaluate_formula_string(&"={1,2,3}/{1,2,3}"), "{1,1,1}");
    assert_eq!(
        evaluate_formula_string(&"={1,2,3}/{0,0,0}"),
        "{#DIV/0!,#DIV/0!,#DIV/0!}"
    );
    assert_eq!(evaluate_formula_string(&"=-{1,2,3}"), "{-1,-2,-3}");
    assert_eq!(evaluate_formula_string(&"=-({1,2,3})"), "{-1,-2,-3}");
}

#[test]
fn it_evaluate_iterator_in_logic_functions() {
    assert_eq!(evaluate_formula_string(&"=AND(0,{0,0,0},0)"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=AND(0,{0,0,0})"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=AND({0,0,0},0)"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=OR(1,{1,0,0},0,1)"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=OR(0,{0,1,0})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=OR({0,0,0},1)"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=AND({0,0,0})"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=AND({1,0,0})"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=AND({1,1,1})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=OR({0,0,0})"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=OR({1,0,0})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=OR({0,1,1})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=OR({1,0,1})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=OR({1,1,1})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=XOR({1,0,1})"), "FALSE");
    assert_eq!(evaluate_formula_string(&"=XOR({0,1,0})"), "TRUE");
    assert_eq!(evaluate_formula_string(&"=XOR({0,0,0})"), "FALSE");
}

#[test]
fn it_evaluate_iterator_with_diffrent_number_of_entries() {
    assert_eq!(evaluate_formula_string(&"={0,0}+{1,2,3}"), "{1,2,#ARG!}");
    assert_eq!(evaluate_formula_string(&"={0,0}*{1,2,3}"), "{0,0,#ARG!}");
    assert_eq!(
        evaluate_formula_string(&"={1,2,3}/{0,0}"),
        "{#DIV/0!,#DIV/0!,#ARG!}"
    );
    assert_eq!(evaluate_formula_string(&"={0,0}+{1,\"Hi\"}"), "{1,#CAST!}");
}

#[test]
fn it_evaluate_date() -> Result<(), ParseError> {
    let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
    let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
    let data_function = |s: String| match s.as_str() {
        "start" => types::Value::Date(start),
        "end" => types::Value::Date(end),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=DAYS(end, start)", Some(&data_function)),
        182.00
    );
    assert_eq!(
        evaluate_formula_date_with_reference(&"=start + 1", Some(&data_function)),
        (start + Duration::days(1)).to_string()
    );
    assert_eq!(
        evaluate_formula_date_with_reference(&"=end-3", Some(&data_function)),
        (end - Duration::days(3)).to_string()
    );
    Ok(())
}

#[test]
fn it_evaluate_custom_functions_() {
    let custom_functions = |s: String, params: Vec<f32>| match s.as_str() {
        "Increase" => types::Value::Number(params[0] + 1.0),
        "SimpleSum" => types::Value::Number(params[0] + params[1]),
        "CustomSum" => types::Value::Number(params[0] + params[1] + params[2]),
        "EqualFive" => types::Value::Number(5.0),
        "CountText" => types::Value::Text(10.0.to_string()),
        "CountNumber" => types::Value::Number(20.0),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=Increase(1)", Some(&custom_functions)),
        2.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=SimpleSum(1,2)", Some(&custom_functions)),
        3.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=CustomSum(1,2,3)", Some(&custom_functions)),
        6.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=EqualFive()+1", Some(&custom_functions)),
        6.0
    );
    assert_eq!(
        evaluate_formula_string_with_custom_function(
            &"=\"P\"&CountText()",
            Some(&custom_functions)
        ),
        "P10"
    );
    assert_eq!(
        evaluate_formula_string_with_custom_function(
            &"=\"P\"&CountNumber()",
            Some(&custom_functions)
        ),
        "P20"
    );
}

#[test]
fn it_evaluate_left_and_right_functions() {
    assert_eq!(evaluate_formula_string(&"=RIGHT(\"apple\", 3)"), "ple",);
    assert_eq!(evaluate_formula_string(&"=RIGHT(\"apple\")"), "e",);

    assert_eq!(
        evaluate_formula_string(&"=\"P\"&RIGHT(\"000\"&1,3)"),
        "P001",
    );
    assert_eq!(evaluate_formula_string(&"=LEFT(\"apple\", 3)"), "app",);
    assert_eq!(evaluate_formula_string(&"=LEFT(\"apple\")"), "a",);

    assert_eq!(evaluate_formula_string(&"=\"P\"&LEFT(\"000\"&1,3)"), "P000",);
}

#[test]
fn it_evaluates_blanks() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(1.0),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(A,B)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A+B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference_no_conversion(&"=SUM(A,C)", Some(&data_function)),
        types::Value::Error(types::Error::Value)
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE(A,B)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(A,B,2,3,B)", Some(&data_function)),
        6.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE(A,B,2,3,B)", Some(&data_function)),
        2.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(A,B)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(1,B,B,B,A,2)", Some(&data_function)),
        2.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=1+B+B+B", Some(&data_function)),
        1.0
    );
}

#[test]
fn it_evaluates_blanks_only() {
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(B)", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=AVERAGE(B)", Some(&data_function)),
        "#DIV/0!"
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(B)", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=-B", Some(&data_function)),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_when_blank_in_first_position() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(1.0),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(B,A)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B+A", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE(B,A)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(B,2,3,B)", Some(&data_function)),
        5.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE(B,2,3,B)", Some(&data_function)),
        2.5
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(B,A)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(B,A,B)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(B,B,B)", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(B,B,B,A,2)", Some(&data_function)),
        2.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B+B+B", Some(&data_function)),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_in_abs_function() {
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=ABS(B)", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=ABS(-B)", Some(&data_function)),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_in_days_function() {
    let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-02-01T02:00:00.000Z")
        .ok()
        .unwrap();
    let data_function = |s: String| match s.as_str() {
        "start" => types::Value::Date(start),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=DAYS(B, B)", Some(&data_function)),
        0.00
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=DAYS(start, B)", Some(&data_function)),
        43495.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=DAYS(B, start)", Some(&data_function)),
        -43495.0
    );
}

#[test]
fn it_evaluates_blanks_with_operators() {
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=1-B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B-1", Some(&data_function)),
        -1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=1+B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B+1", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=1/B", Some(&data_function)),
        "#DIV/0!"
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B/1", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=1*B", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B*1", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=1^B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B^1", Some(&data_function)),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_with_operators_and_reference() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(1.0),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A-B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B-A", Some(&data_function)),
        -1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A+B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B+A", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=A/B", Some(&data_function)),
        "#DIV/0!"
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B/A", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A*B", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B*A", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=A^B", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=B^A", Some(&data_function)),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_in_boolean_operations() {
    let data_function = |s: String| match s.as_str() {
        "T" => types::Value::Boolean(types::Boolean::True),
        "B" => types::Value::Blank,
        "F" => types::Value::Boolean(types::Boolean::False),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND(T,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR(T,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=NOT(B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND(F,B)", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR(F,B)", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR(T,B,F,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR(F,B,T,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND(T,B,F,B)", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND(F,B,T,B)", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR(T,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR(F,B)", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR(T,B,F,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR(F,B,T,B)", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR(T,B,F,B,T)", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR(F,B,T,B,F)", Some(&data_function)),
        "TRUE"
    );
}

#[test]
fn it_evaluates_blanks_in_comparison_operators() {
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B=B", Some(&data_function)),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=1=B", Some(&data_function)),
        "FALSE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B=1", Some(&data_function)),
        "FALSE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=1>B", Some(&data_function)),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B>1", Some(&data_function)),
        "FALSE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=0=B", Some(&data_function)),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B=0", Some(&data_function)),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B='test'", Some(&data_function)),
        "FALSE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"='test'=B", Some(&data_function)),
        "FALSE",
    );
}

#[test]
fn it_evaluates_blanks_in_comparison_operators_with_references() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(-2.0),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_string_with_reference(&"=A>=B", Some(&data_function)),
        "FALSE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B>=A", Some(&data_function)),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=A<B", Some(&data_function)),
        "TRUE",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B<A", Some(&data_function)),
        "FALSE",
    );
}

#[test]
fn it_evaluates_blanks_string_operations() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(-2.0),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_string_with_reference(&"=\"Hello\"&B", Some(&data_function)),
        "Hello",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B&\"Hello\"", Some(&data_function)),
        "Hello",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=B&B", Some(&data_function)),
        "",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=RIGHT(B)", Some(&data_function)),
        "",
    );
    assert_eq!(
        evaluate_formula_string_with_reference(&"=LEFT(B)", Some(&data_function)),
        "",
    );
}

#[test]
fn it_evaluates_blank_constructors() {
    let custom_functions = |s: String, _params: Vec<f32>| match s.as_str() {
        "BLANK" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=SUM(BLANK())", Some(&custom_functions)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=SUM(BLANK(), 1)", Some(&custom_functions)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(
            &"=PRODUCT(BLANK(), 1)",
            Some(&custom_functions)
        ),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(
            &"=AVERAGE(BLANK(), 1)",
            Some(&custom_functions)
        ),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=BLANK()+1", Some(&custom_functions)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_custom_function(&"=BLANK()*1", Some(&custom_functions)),
        0.0
    );
}

#[test]
fn it_evaluates_blank_in_iterators() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(100.0),
        "Array" => types::Value::Iterator(vec![
            types::Value::Number(100.0),
            types::Value::Blank,
            types::Value::Blank,
        ]),
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE({A, B, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE({A, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE({B, A, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=AVERAGE(Array)", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM({A, B, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM({A, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM({B, A, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=SUM(Array)", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT({A, B, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT({A, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT({B, A, B})", Some(&data_function)),
        100.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=PRODUCT(Array)", Some(&data_function)),
        100.0
    );
}

#[test]
fn it_evaluates_blank_with_iterators_in_boolean_operations() {
    let data_function = |s: String| match s.as_str() {
        "T" => types::Value::Boolean(types::Boolean::True),
        "B" => types::Value::Blank,
        "F" => types::Value::Boolean(types::Boolean::False),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND({T,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR({T,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND({F,B})", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR({F,B})", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR({T,B,F,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=OR({F,B,T,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND({T,B,F,B})", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=AND({F,B,T,B})", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR({T,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR({F,B})", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR({T,B,F,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR({F,B,T,B})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR({T,B,F,B,T})", Some(&data_function)),
        "FALSE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(&"=XOR({F,B,T,B,F})", Some(&data_function)),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_boolean_with_reference(
            &"=XOR({F,B,T,B,F,{F,B,T,B,F}})",
            Some(&data_function)
        ),
        "FALSE"
    );
}

#[test]
fn it_evaluates_formulas_with_3_params() {
    assert_eq!(evaluate_formula_number(&"=IF()"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF( )"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF( ,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(, )"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF( , )"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(,,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(, ,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(,,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(,, )"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(1,,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(1,2,)"), 2.0);
    assert_eq!(evaluate_formula_number(&"=IF(1,2)"), 2.0);
    assert_eq!(evaluate_formula_number(&"=IF(0,2,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(0,2)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(0,,3)"), 3.0);
    assert_eq!(evaluate_formula_number(&"=IF(1,,3)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(0,2,3)"), 3.0);
    assert_eq!(evaluate_formula_number(&"=IF(,2,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(,2)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(,2,3)"), 3.0);
    assert_eq!(evaluate_formula_number(&"=IF(,,3)"), 3.0);
}

#[test]
fn it_evaluates_if_formulas() -> Result<(), ParseError> {
    assert_eq!(evaluate_formula_number(&"=IF(TRUE,1,0)"), 1.0);
    assert_eq!(evaluate_formula_number(&"=IF(FALSE,1,0)"), 0.0);
    assert_eq!(evaluate_formula_string(&"=IF(TRUE,\"a\",0)"), "a");
    assert_eq!(evaluate_formula_number(&"=IF(FALSE,\"a\",0)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(1=1,1,0)"), 1.0);
    assert_eq!(evaluate_formula_number(&"=IF(1=2,1,0)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(AND(TRUE,FALSE),1,0)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(TRUE,IF(FALSE,1,2),0)"), 2.0);
    assert_eq!(evaluate_formula_number(&"=IF(2,1,0)"), 1.0);
    assert_eq!(evaluate_formula_number(&"=IF(-1,1,0)"), 1.0);
    assert_eq!(evaluate_formula_number(&"=IF(0,1,0)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(TRUE,1+2+3,0)"), 6.0);
    assert_eq!(evaluate_formula_number(&"=IF(FALSE,1)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(FALSE,1,)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(TRUE,,1)"), 0.0);
    assert_eq!(evaluate_formula_number(&"=IF(TRUE, ,1)"), 0.0);
    assert_eq!(evaluate_formula_string(&"=IF(TRUE,TRUE,FALSE)"), "TRUE");
    assert_eq!(
        evaluate_formula_string(&"=IF( TRUE , TRUE , FALSE )"),
        "TRUE"
    );
    assert_eq!(evaluate_formula_string(&"=IF(TRUE , TRUE, FALSE )"), "TRUE");
    assert_eq!(evaluate_formula_number(&"=IF(1,IF(FALSE,1,2),0)"), 2.0);
    assert_eq!(
        evaluate_formula_number(&"=IF(1=0,IF(FALSE,1,2),IF(TRUE,1,2))"),
        1.0
    );
    assert_eq!(
        evaluate_formula_string(&"=IF(1/0,IF(FALSE,1,2),0)"),
        "#DIV/0!",
    );
    assert_eq!(evaluate_formula_string(&"=IF(\"text\",1,0)"), "#VALUE!",);

    let date1: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
    let date2: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
    let data_function = |s: String| match s.as_str() {
        "date1" => types::Value::Date(date1),
        "date2" => types::Value::Date(date2),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_number_with_reference(&"=IF(date1=date2,1,0)", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=IF(date1<>date2,1,0)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=IF(date1<date2,1,0)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=IF(date1>date2,1,0)", Some(&data_function)),
        0.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=IF(date1<=date2,1,0)", Some(&data_function)),
        1.0
    );
    assert_eq!(
        evaluate_formula_number_with_reference(&"=IF(date1>=date2,1,0)", Some(&data_function)),
        0.0
    );
    Ok(())
}

#[test]
fn it_evaluates_if_formulas_with_text() {
    let data_function = |s: String| match s.as_str() {
        "ReferenceKey" => types::Value::Text("100".to_string()),
        "ReferenceName" => types::Value::Text("Test".to_string()),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_string_with_reference(
            &"=IF(ReferenceKey=\"10\",\"\",ReferenceKey&\" - \")&ReferenceName",
            Some(&data_function)
        ),
        "100 - Test"
    );
}

#[test]
fn it_evaluates_isblank_function() {
    let data_function = |s: String| match s.as_str() {
        // "ReferenceKey" => types::Value::Text("100".to_string()),
        "ReferenceName" => types::Value::Text("Test".to_string()),
        _ => types::Value::Error(types::Error::Value),
    };
    assert_eq!(
        evaluate_formula_string_with_reference(
            &"=ISBLANK(ReferenceKey)",
            Some(&data_function)
        ),
        "TRUE"
    );
    assert_eq!(
        evaluate_formula_string_with_reference(
            &"=ISBLANK(ReferenceName)",
            Some(&data_function)
        ),
        "FALSE"
    );
}
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Text("=1+B".to_string()),
        "B" => types::Value::Number(3.0),
        _ => types::Value::Error(types::Error::Value),
    };
    let formula = parse_formula::parse_string_to_formula(&"=A+B");
    let result = calculate::calculate_formula(formula, Some(data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1+3/0"); // error (#DIV/0!)
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=\"Hello \" & \" World!\"");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1 + \"Hello\""); // error (#CAST!)
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"1.2");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"Hello World");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(1,2,\"3\")");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(ABS(1),2*1, 3,4*1)");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=2>=1");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=OR(1>1,1<>1)");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=AND(\"test\",\"True\", 1, true) ");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));
}

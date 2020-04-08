extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

use calculator::types;

// fn is_string_number(str1: String) -> bool {
//     match str1.parse::<f32>() {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

fn main() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Number(1.0),
        "B" => types::Value::Number(2.0),
        _ => types::Value::Error(types::Error::Value),
    };
    let x = data_function(String::from("A"));
    println!(
        "*** Result from data_function is {:?}. ***",
        calculate::result_to_string(x)
    );

    let formula = parse_formula::parse_string_to_formula(&"= fx_rate +  hello world");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );

    // let formula = parse_formula::parse_string_to_formula(&"=2>=1");
    // println!("{:?}", formula);
    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );
}

extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );

    let formula = parse_formula::parse_string_to_formula(&"=ABS(-1-4)");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );

    let formula = parse_formula::parse_string_to_formula(&"=SUM(1+2, 2, 3,4)");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );

    // let formula = parse_formula::parse_string_to_formula(&"1");
    // println!("{:?}", formula);
    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );

    // let formula = parse_formula::parse_string_to_formula(&"=\"Hello \"&'World'");
    // println!("{:?}", formula);
    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );
}

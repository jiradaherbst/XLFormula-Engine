extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"= 1773 + 1362");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );

    // let result = parse_formula::parse_string_to_formula_and_evaluate(&"= 1773 + 1362");
    // println!("{:?}", result);

    // let result = parse_formula::parse_string_to_formula_and_evaluate(&"=4^0.5");
    // println!("{:?}", result);
}

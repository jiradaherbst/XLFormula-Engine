extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));
}

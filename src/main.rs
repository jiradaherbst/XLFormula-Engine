extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(1*1, 2*1,3*1, 4*1)");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );
}

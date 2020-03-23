extern crate calculator;
//use calculator::calculate;
use calculator::parse_formula;

fn main() {
    //parse_formula::parse_string_to_formula(&"= 1773 + 1362");
    //let _formula = parse_formula::parse_string_to_formula(&"= (1+2) * (2+3)");
    //println!("{:?}", formula);
    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );

    //parse_formula::parse_string_to_formula(&"= Hello");
    let formula = parse_formula::parse_string_to_formula(&"= \"Hello\" & \"  123\"");
    println!("{:?}", formula);

    let formula = parse_formula::parse_string_to_formula(&"=\"Hello\" & \"World!\"");
    println!("{:?}", formula);

    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );

    // let formula = parse_formula::parse_string_to_formula(&"=1+2+3");
    // println!("{:?}", formula);
    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );

    // let formula = parse_formula::parse_string_to_formula(&"=1+2*3");
    // println!("{:?}", formula);
    // let result = calculate::calculate_formula(formula);
    // println!("{:?}", result);
    // println!(
    //     "Result from result_to_string is {}",
    //     calculate::result_to_string(result)
    // );
}

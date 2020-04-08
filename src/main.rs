extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

// fn is_string_number(str1: String) -> bool {
//     match str1.parse::<f32>() {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"=_input+fix.");
    println!("{:?}", formula);
    let formula = parse_formula::parse_string_to_formula(&"=\\input+fix_rate");
    println!("{:?}", formula);
    let formula = parse_formula::parse_string_to_formula(&"=HelloWorld+fix_rate");
    println!("{:?}", formula);

    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula, None);
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

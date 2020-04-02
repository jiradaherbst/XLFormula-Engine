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
    let formula = parse_formula::parse_string_to_formula(&"=XOR(2=2,1=1)");
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

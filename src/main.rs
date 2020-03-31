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
    // let parse_string = "3.0".to_string();
    // if is_string_number(parse_string) {
    //     println!("hello");
    // } else {
    //     println!("world");
    // }

    let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(1*1, 2*1,3*1, 4*1)");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );
}

extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"1773 + 1362");
    println!("{:?}", formula);
    let result = calculate::calculate_formula(formula);
    println!("{:?}", result);
    println!(
        "Result from result_to_string is {}",
        calculate::result_to_string(result)
    );
}

// extern crate pest;
// #[macro_use]
// extern crate calculator;
// extern crate pest_derive;

// //use parse_formula;
// use calculator::calculate;
// use calculator::types;
// //use calculator::parse_formula;
// use pest::Parser;

// //use std::mem;

// #[derive(Parser)]
// #[grammar = "grammar.pest"]
// pub struct GrammarParser;

// let parse_result = GrammarParser::parse(Rule::sum, "1773 + 1362")
//     .unwrap()
//     .next()
//     .unwrap();
// let parse_result = parse_string();
// println!("{:?}", parse_result);
// let _formula = build_formula(parse_result);

//assert_eq!(calculate::result_to_string(result), 3135, "should calculate plus");

// let parse_result = GrammarParser::parse(Rule::minus, "123 - 456")
//     .unwrap()
//     .next()
//     .unwrap();
// let _formula = build_formula(parse_result);
// println!("{:?}", _formula);
// let result = calculate::calculate_formula(_formula);
// println!("{:?}", result);
// println!(
//     "Resule from result_to_string is {}",
//     calculate::result_to_string(result)
// );

// let parse_result = GrammarParser::parse(Rule::minus, "12 - 12")
//     .unwrap()
//     .next()
//     .unwrap();
// let _formula = build_formula(parse_result);
// println!("{:?}", _formula);
// let result = calculate::calculate_formula(_formula);
// println!("{:?}", result);
// println!(
//     "Result from result_to_string is {}",
//     calculate::result_to_string(result)
// );

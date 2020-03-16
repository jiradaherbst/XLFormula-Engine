extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;
use std::string::String;

// // test fn
// // only the first case get matched and print out the Pairs
// fn build_formula(parse_str: pest::iterators::Pair<Rule>) {
//     match parse_str.as_rule() {
//         Rule::formula => {
//             for element in parse_str.into_inner() {
//                 println! {"{:?}", element};
//             }
//         }

//         Rule::number => {
//             let x = parse_str.as_str().parse::<f32>().unwrap();
//             println!("{}", x);
//         }
//         Rule::operator => {
//             let op = parse_str.as_rule();
//             match op {
//                 Rule::add => println!("message from Rule::operator -> operator is add"),
//                 Rule::subtract => println!("message from Rule::operator -> operator is subtract"),
//                 _ => println!("message from Rule::operator"),
//             }
//         }
//         Rule::add => {
//             println!("message from Rule::add -> operator is add");
//         }
//         Rule::subtract => {
//             println!("message from Rule::subtract -> operator is subtract");
//         }
//     }
// }

// //test fn
// pub fn parse_string_to_formula(s: &str) {
//     let parse_result = GrammarParser::parse(Rule::formula, s)
//         .unwrap()
//         .next()
//         .unwrap();
//     build_formula(parse_result);
// }

fn build_formula(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
    match parse_str.as_rule() {
        Rule::formula => {
            let mut pairs = parse_str.into_inner();
            let num1 = pairs.next().unwrap();
            let op = pairs.next().unwrap();
            let num2 = pairs.next().unwrap();
            match op.as_rule() {
                Rule::add => {
                    let operation = types::Expression {
                        lhs: Box::new(build_formula(num1)),
                        rhs: Box::new(build_formula(num2)),
                        op: types::Operator::Plus,
                    };
                    types::Formula::Operation(operation)
                }
                Rule::subtract => {
                    let operation = types::Expression {
                        lhs: Box::new(build_formula(num1)),
                        rhs: Box::new(build_formula(num2)),
                        op: types::Operator::Minus,
                    };
                    types::Formula::Operation(operation)
                }
                _ => {
                    let value = types::Value::Error(String::from("Null Formula"));
                    types::Formula::Value(value)
                }
            }
        }
        Rule::number => {
            let x = parse_str.as_str().parse::<f32>().unwrap();
            let value = types::Value::Number(x);
            types::Formula::Value(value)
        }
        Rule::operator => {
            let value = types::Value::Error(String::from("Null Formula"));
            types::Formula::Value(value)
        }
        Rule::add => {
            let value = types::Value::Error(String::from("Null Formula"));
            types::Formula::Value(value)
        }
        Rule::subtract => {
            let value = types::Value::Error(String::from("Null Formula"));
            types::Formula::Value(value)
        }
    }
}

pub fn parse_string_to_formula(s: &str) -> types::Formula {
    let parse_result = GrammarParser::parse(Rule::formula, s)
        .unwrap()
        .next()
        .unwrap();
    let _formula = build_formula(parse_result);
    _formula
}

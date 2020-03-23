extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;
//use std::string::String;

use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;
use pest::prec_climber::PrecClimber;

pub fn parse_string_to_formula(s: &str) -> types::Formula {
    let parse_result = GrammarParser::parse(Rule::formula, s)
        .unwrap()
        .next()
        .unwrap();
    //println!("{:?}", parse_result);
    build_formula_with_climber(parse_result.into_inner())
}

fn build_formula_with_climber(expression: pest::iterators::Pairs<Rule>) -> types::Formula {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    climber.climb(
        expression,
        |pair: pest::iterators::Pair<Rule>| match pair.as_rule() {
            Rule::number => {
                let x = pair.as_str().parse::<f32>().unwrap();
                let value = types::Value::Number(x);
                types::Formula::Value(value)
            }
            Rule::string => {
                let string = pair.as_str().parse::<String>().unwrap();
                let value = types::Value::Text(string);
                types::Formula::Value(value)
            }

            Rule::expr => build_formula_with_climber(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: types::Formula, op: pest::iterators::Pair<Rule>, rhs: types::Formula| match op
            .as_rule()
        {
            Rule::add => {
                let operation = types::Expression {
                    op: types::Operator::Plus,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::subtract => {
                let operation = types::Expression {
                    op: types::Operator::Minus,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::multiply => {
                let operation = types::Expression {
                    op: types::Operator::Multiply,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::divide => {
                let operation = types::Expression {
                    op: types::Operator::Divide,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::power => {
                let operation = types::Expression {
                    op: types::Operator::Power,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            _ => unreachable!(),
        },
    )
}

// fn build_formula(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
//     match parse_str.as_rule() {
//         Rule::expr => build_formula_internal(parse_str),
//         Rule::number => {
//             let x = parse_str.as_str().parse::<f32>().unwrap();
//             let value = types::Value::Number(x);
//             types::Formula::Value(value)
//         }
//         _ => {
//             let value = types::Value::Error(String::from("Null Formula1"));
//             types::Formula::Value(value)
//         }
//     }
// }

// fn build_formula_internal(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
//     let mut pairs = parse_str.into_inner();
//     let number1 = pairs.next().unwrap();
//     let operator = pairs.next().unwrap();
//     let number2 = pairs.next().unwrap();

//     let operator = match operator.as_rule() {
//         Rule::add => types::Operator::Plus,
//         Rule::subtract => types::Operator::Minus,
//         Rule::multiply => types::Operator::Multiply,
//         Rule::divide => types::Operator::Divide,
//         Rule::power => types::Operator::Power,
//         _ => types::Operator::Null,
//     };

//     match operator {
//         types::Operator::Null => {
//             let value = types::Value::Error(String::from("Null Formula2"));
//             types::Formula::Value(value)
//         }
//         _ => {
//             let operation = types::Expression {
//                 //lhs: Box::new(build_formula(number1)),
//                 //rhs: Box::new(build_formula(number2)),
//                 op: operator,
//                 values: vec![build_formula(number1), build_formula(number2)],
//             };
//             types::Formula::Operation(operation)
//         }
//     }
// }

extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;
use std::string::String;

use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;
use pest::prec_climber::PrecClimber;

pub fn parse_string_to_formula(s: &str) -> types::Formula {
    let parse_result = GrammarParser::parse(Rule::formula, s)
        .unwrap()
        .next()
        .unwrap();
    println!("{:?}", parse_result);
    build_formula(parse_result)
}

fn build_formula(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
    match parse_str.as_rule() {
        Rule::expr => build_formula_internal(parse_str),
        Rule::number => {
            let x = parse_str.as_str().parse::<f32>().unwrap();
            let value = types::Value::Number(x);
            types::Formula::Value(value)
        }
        _ => {
            let value = types::Value::Error(String::from("Null Formula1"));
            types::Formula::Value(value)
        }
    }
}

fn build_formula_internal(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
    let mut pairs = parse_str.into_inner();
    let number1 = pairs.next().unwrap();
    let operator = pairs.next().unwrap();
    let number2 = pairs.next().unwrap();

    let operator = match operator.as_rule() {
        Rule::add => types::Operator::Plus,
        Rule::subtract => types::Operator::Minus,
        Rule::multiply => types::Operator::Multiply,
        Rule::divide => types::Operator::Divide,
        Rule::power => types::Operator::Power,
        _ => types::Operator::Null,
    };

    match operator {
        types::Operator::Null => {
            let value = types::Value::Error(String::from("Null Formula2"));
            types::Formula::Value(value)
        }
        _ => {
            let operation = types::Expression {
                //lhs: Box::new(build_formula(number1)),
                //rhs: Box::new(build_formula(number2)),
                op: operator,
                values: vec![build_formula(number1), build_formula(number2)],
            };
            types::Formula::Operation(operation)
        }
    }
}

// pub fn parse_string_to_formula_and_evaluate(s: &str) -> types::Value {
//     let parse_result = GrammarParser::parse(Rule::formula, s)
//         .unwrap()
//         .next()
//         .unwrap();
//     let result = evaluate_formula(parse_result.into_inner());
//     if result.is_infinite() {
//         types::Value::Error(String::from("#DIV/0!"))
//     } else {
//         types::Value::Number(result)
//     }
// }

// fn is_float_int(num: f32) -> bool {
//     ((num as i32) as f32) == num
// }

// // fn eval_primary(pair: pest::iterators::Pair<Rule>) -> f32 {
// //     match pair.as_rule() {
// //         Rule::number => pair.as_str().parse::<f32>().unwrap(),
// //         Rule::expr => evaluate_formula(pair.into_inner()),
// //         _ => unreachable!(),
// //     }
// // }

// fn evaluate_formula(expression: pest::iterators::Pairs<Rule>) -> f32 {
//     let climber = PrecClimber::new(vec![
//         Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
//         Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
//         Operator::new(Rule::power, Assoc::Right),
//     ]);
//     climber.climb(
//         expression,
//         |pair: pest::iterators::Pair<Rule>| match pair.as_rule() {
//             Rule::number => pair.as_str().parse::<f32>().unwrap(),
//             Rule::expr => evaluate_formula(pair.into_inner()),
//             _ => unreachable!(),
//         },
//         |lhs: f32, op: pest::iterators::Pair<Rule>, rhs: f32| match op.as_rule() {
//             Rule::add => lhs + rhs,
//             Rule::subtract => lhs - rhs,
//             Rule::multiply => lhs * rhs,
//             Rule::divide => lhs / rhs,
//             Rule::power => {
//                 if is_float_int(rhs) {
//                     lhs.powi(rhs as i32)
//                 } else {
//                     lhs.powf(rhs)
//                 }
//             }
//             _ => unreachable!(),
//         },
//     )
// }

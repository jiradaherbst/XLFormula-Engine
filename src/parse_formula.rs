extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;
use std::string::String;

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
            let value = types::Value::Error(String::from("Null Formula"));
            types::Formula::Value(value)
        }
        _ => {
            let operation = types::Expression {
                lhs: Box::new(build_formula(number1)),
                rhs: Box::new(build_formula(number2)),
                op: operator,
            };
            types::Formula::Operation(operation)
        }
    }
}

fn build_formula(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
    match parse_str.as_rule() {
        Rule::formula => build_formula_internal(parse_str),
        Rule::number => {
            let x = parse_str.as_str().parse::<f32>().unwrap();
            let value = types::Value::Number(x);
            types::Formula::Value(value)
        }
        _ => {
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
    build_formula(parse_result)
}

extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;
use std::string::String;

fn build_formula(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
    match parse_str.as_rule() {
        Rule::formula => {
            let mut pairs = parse_str.into_inner();
            let num1 = pairs.next().unwrap();
            let op = pairs.next().unwrap();
            let num2 = pairs.next().unwrap();
            let oper = op.as_str().parse::<String>().unwrap();
            match oper.as_str() {
                " + " => {
                    let operation = types::Expression {
                        lhs: Box::new(build_formula(num1)),
                        rhs: Box::new(build_formula(num2)),
                        op: types::Operator::Plus,
                    };
                    types::Formula::Operation(operation)
                }
                " - " => {
                    let operation = types::Expression {
                        lhs: Box::new(build_formula(num1)),
                        rhs: Box::new(build_formula(num2)),
                        op: types::Operator::Minus,
                    };
                    types::Formula::Operation(operation)
                }
                _ => {
                    let operation = types::Expression {
                        lhs: Box::new(build_formula(num1)),
                        rhs: Box::new(build_formula(num2)),
                        op: types::Operator::Null,
                    };
                    types::Formula::Operation(operation)
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

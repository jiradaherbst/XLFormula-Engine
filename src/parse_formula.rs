extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;
use std::string::String;

fn build_formula(parse_str: pest::iterators::Pair<Rule>) -> types::Formula {
    match parse_str.as_rule() {
        Rule::sum => {
            let mut pairs = parse_str.into_inner();
            let num1 = pairs.next().unwrap();
            let num2 = pairs.next().unwrap();
            let _operation = types::Expression {
                lhs: Box::new(build_formula(num1)),
                rhs: Box::new(build_formula(num2)),
                op: types::Operator::Plus,
            };
            types::Formula::Operation(_operation)
        }
        Rule::minus => {
            let mut pairs = parse_str.into_inner();
            let num1 = pairs.next().unwrap();
            let num2 = pairs.next().unwrap();
            let _operation = types::Expression {
                lhs: Box::new(build_formula(num1)),
                rhs: Box::new(build_formula(num2)),
                op: types::Operator::Minus,
            };
            types::Formula::Operation(_operation)
        }
        Rule::number => {
            let _x = parse_str.as_str().parse::<f32>().unwrap();
            let _value = types::Value::Number(_x);
            types::Formula::Value(_value)
        }
    }
}

pub fn parse_string_to_formula(op: String, s: &str) -> types::Formula {
    match op.as_str() {
        "sum" => {
            let parse_result = GrammarParser::parse(Rule::sum, s).unwrap().next().unwrap();
            let _formula = build_formula(parse_result);
            _formula
        }
        "minus" => {
            let parse_result = GrammarParser::parse(Rule::minus, s)
                .unwrap()
                .next()
                .unwrap();
            let _formula = build_formula(parse_result);
            _formula
        }
        _ => {
            let error = types::Value::Error(String::from("Error"));
            types::Formula::Value(error)
        }
    }

    // let parse_result = GrammarParser::parse(Rule::sum, s).unwrap().next().unwrap();
    // let _formula = build_formula(parse_result);
    // _formula
}

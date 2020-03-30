extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;

use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;
use pest::prec_climber::PrecClimber;

pub fn parse_string_to_formula(s: &str) -> types::Formula {
    let parse_result = GrammarParser::parse(Rule::formula, s)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    println!("{:?}", parse_result);
    match parse_result.as_rule() {
        Rule::expr => build_formula_with_climber(parse_result.into_inner()),
        Rule::string_constant => {
            let string = parse_result
                .into_inner()
                .as_str()
                .parse::<String>()
                .unwrap();
            let value = types::Value::Text(string.trim_start_matches('\'').to_string());
            types::Formula::Value(value)
        }
        _ => unreachable!(),
    }
}

fn build_formula_with_climber(expression: pest::iterators::Pairs<Rule>) -> types::Formula {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::concat, Assoc::Left),
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
            Rule::string_double_quote => {
                let string = pair.into_inner().as_str().parse::<String>().unwrap();
                let value = types::Value::Text(string.replace("\"\"", "\""));
                types::Formula::Value(value)
            }

            Rule::string_single_quote => {
                let string = pair.into_inner().as_str().parse::<String>().unwrap();
                let value = types::Value::Text(string);
                types::Formula::Value(value)
            }
            Rule::abs => {
                let operation = types::Expression {
                    // op: types::Operator::Null,
                    // func: types::Function::Abs,
                    op: types::Operator::Function(types::Function::Abs),
                    values: vec![build_formula_with_climber(pair.into_inner())],
                };
                types::Formula::Operation(operation)
            }

            Rule::sum => {
                let mut vec = Vec::new();
                for term in pair.into_inner() {
                    vec.push(build_formula_with_climber(term.into_inner()));
                }
                let operation = types::Expression {
                    // op: types::Operator::Null,
                    // func: types::Function::Sum,
                    op: types::Operator::Function(types::Function::Sum),
                    values: vec,
                };
                types::Formula::Operation(operation)
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
                    //func: types::Function::Null,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::subtract => {
                let operation = types::Expression {
                    op: types::Operator::Minus,
                    //func: types::Function::Null,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::multiply => {
                let operation = types::Expression {
                    op: types::Operator::Multiply,
                    //func: types::Function::Null,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::divide => {
                let operation = types::Expression {
                    op: types::Operator::Divide,
                    //func: types::Function::Null,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::power => {
                let operation = types::Expression {
                    op: types::Operator::Power,
                    //func: types::Function::Null,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::concat => {
                let operation = types::Expression {
                    op: types::Operator::Concat,
                    //func: types::Function::Null,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            _ => unreachable!(),
        },
    )
}

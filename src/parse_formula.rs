extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;

use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;
use pest::prec_climber::PrecClimber;

/// Use this function to catch a parse error.
fn parse_string(s: &str) -> Option<pest::iterators::Pair<Rule>> {
    let parse_result = GrammarParser::parse(Rule::formula, s);
    //println!("{:?}", parse_result);
    match parse_result {
        Ok(mut result) => {
            let parse_result = result.next().unwrap();
            Some(parse_result)
        }
        Err(_) => None,
    }
}

fn parse_string_constant(parse_result: pest::iterators::Pair<Rule>) -> types::Formula {
    let string = parse_result
        .into_inner()
        .as_str()
        .parse::<String>()
        .unwrap();
    types::Formula::Value(types::Value::Text(
        string.trim_start_matches('\'').to_string(),
    ))
}

/// Parses a string and stores it in Formula Enum.
pub fn parse_string_to_formula(s: &str) -> types::Formula {
    match parse_string(&s) {
        Some(parse_result) => match parse_result.as_rule() {
            Rule::expr => build_formula_with_climber(parse_result.into_inner()),
            Rule::string_constant => parse_string_constant(parse_result),
            _ => unreachable!(),
        },
        None => types::Formula::Value(types::Value::Error(types::Error::Parse)),
    }
}

fn build_formula_number(pair: pest::iterators::Pair<Rule>) -> types::Formula {
    let x = pair.as_str().parse::<f32>().unwrap();
    let value = types::Value::Number(x);
    types::Formula::Value(value)
}

fn build_formula_string_double_quote(pair: pest::iterators::Pair<Rule>) -> types::Formula {
    let string = pair.into_inner().as_str().parse::<String>().unwrap();
    let value = types::Value::Text(string.replace("\"\"", "\""));
    types::Formula::Value(value)
}

fn build_formula_string_single_quote(pair: pest::iterators::Pair<Rule>) -> types::Formula {
    let string = pair.into_inner().as_str().parse::<String>().unwrap();
    let value = types::Value::Text(string);
    types::Formula::Value(value)
}

fn build_formula_boolean(boolean_value: bool) -> types::Formula {
    match boolean_value {
        true => types::Formula::Value(types::Value::Boolean(types::Boolean::True)),
        false => types::Formula::Value(types::Value::Boolean(types::Boolean::False)),
    }
}

fn build_formula_unary_operator(
    unary_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
) -> types::Formula {
    let operation = match unary_operation {
        Rule::abs => types::Expression {
            op: types::Operator::Function(types::Function::Abs),
            values: vec![build_formula_with_climber(pair.into_inner())],
        },
        Rule::not => types::Expression {
            op: types::Operator::Function(types::Function::Not),
            values: vec![build_formula_with_climber(pair.into_inner())],
        },
        Rule::negate => types::Expression {
            op: types::Operator::Function(types::Function::Negate),
            values: vec![build_formula_with_climber(pair.into_inner())],
        },
        _ => unreachable!(),
    };
    types::Formula::Operation(operation)
}

fn build_formula_reference(pair: pest::iterators::Pair<Rule>) -> types::Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    types::Formula::Reference(string)
}

fn build_formula_iterator(pair: pest::iterators::Pair<Rule>) -> types::Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        vec.push(build_formula_with_climber(term.into_inner()));
    }
    types::Formula::Iterator(vec)
}

fn build_formula_collective_operator(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
) -> types::Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        vec.push(build_formula_with_climber(term.into_inner()));
    }
    let operation = match collective_operation {
        Rule::sum => types::Expression {
            op: types::Operator::Function(types::Function::Sum),
            values: vec,
        },
        Rule::product => types::Expression {
            op: types::Operator::Function(types::Function::Product),
            values: vec,
        },
        Rule::average => types::Expression {
            op: types::Operator::Function(types::Function::Average),
            values: vec,
        },
        Rule::or => types::Expression {
            op: types::Operator::Function(types::Function::Or),
            values: vec,
        },
        Rule::and => types::Expression {
            op: types::Operator::Function(types::Function::And),
            values: vec,
        },
        Rule::xor => types::Expression {
            op: types::Operator::Function(types::Function::Xor),
            values: vec,
        },
        _ => unreachable!(),
    };
    types::Formula::Operation(operation)
}

fn build_formula_binary_operator(
    binary_operator: Rule,
    lhs: types::Formula,
    rhs: types::Formula,
) -> types::Formula {
    let operation = match binary_operator {
        Rule::add => types::Expression {
            op: types::Operator::Plus,
            values: vec![lhs, rhs],
        },
        Rule::subtract => types::Expression {
            op: types::Operator::Minus,
            values: vec![lhs, rhs],
        },
        Rule::multiply => types::Expression {
            op: types::Operator::Multiply,
            values: vec![lhs, rhs],
        },
        Rule::divide => types::Expression {
            op: types::Operator::Divide,
            values: vec![lhs, rhs],
        },
        Rule::power => types::Expression {
            op: types::Operator::Power,
            values: vec![lhs, rhs],
        },
        Rule::concat => types::Expression {
            op: types::Operator::Concat,
            values: vec![lhs, rhs],
        },
        Rule::equal => types::Expression {
            op: types::Operator::Equal,
            values: vec![lhs, rhs],
        },
        Rule::not_equal => types::Expression {
            op: types::Operator::NotEqual,
            values: vec![lhs, rhs],
        },
        Rule::greater => types::Expression {
            op: types::Operator::Greater,
            values: vec![lhs, rhs],
        },
        Rule::less => types::Expression {
            op: types::Operator::Less,
            values: vec![lhs, rhs],
        },
        Rule::greater_or_equal => types::Expression {
            op: types::Operator::GreaterOrEqual,
            values: vec![lhs, rhs],
        },
        Rule::less_or_equal => types::Expression {
            op: types::Operator::LessOrEqual,
            values: vec![lhs, rhs],
        },
        _ => unreachable!(0),
    };

    types::Formula::Operation(operation)
}

/// Builds Formula Enum using a `pest-PrecClimber`.
fn build_formula_with_climber(expression: pest::iterators::Pairs<Rule>) -> types::Formula {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::concat, Assoc::Left),
        Operator::new(Rule::equal, Assoc::Left) | Operator::new(Rule::not_equal, Assoc::Left),
        Operator::new(Rule::greater, Assoc::Left)
            | Operator::new(Rule::less, Assoc::Left)
            | Operator::new(Rule::greater_or_equal, Assoc::Left)
            | Operator::new(Rule::less_or_equal, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    climber.climb(
        expression,
        |pair: pest::iterators::Pair<Rule>| match pair.as_rule() {
            Rule::number => build_formula_number(pair),
            Rule::string_double_quote => build_formula_string_double_quote(pair),
            Rule::string_single_quote => build_formula_string_single_quote(pair),
            Rule::t => build_formula_boolean(true),
            Rule::f => build_formula_boolean(false),
            Rule::abs => build_formula_unary_operator(Rule::abs, pair),
            Rule::sum => build_formula_collective_operator(Rule::sum, pair),
            Rule::product => build_formula_collective_operator(Rule::product, pair),
            Rule::average => build_formula_collective_operator(Rule::average, pair),
            Rule::or => build_formula_collective_operator(Rule::or, pair),
            Rule::and => build_formula_collective_operator(Rule::and, pair),
            Rule::xor => build_formula_collective_operator(Rule::xor, pair),
            Rule::not => build_formula_unary_operator(Rule::not, pair),
            Rule::reference => build_formula_reference(pair),
            Rule::iterator => build_formula_iterator(pair),
            Rule::negate => build_formula_unary_operator(Rule::negate, pair),
            Rule::expr => build_formula_with_climber(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: types::Formula, op: pest::iterators::Pair<Rule>, rhs: types::Formula| match op
            .as_rule()
        {
            Rule::add => build_formula_binary_operator(Rule::add, lhs, rhs),
            Rule::subtract => build_formula_binary_operator(Rule::subtract, lhs, rhs),
            Rule::multiply => build_formula_binary_operator(Rule::multiply, lhs, rhs),
            Rule::divide => build_formula_binary_operator(Rule::divide, lhs, rhs),
            Rule::power => build_formula_binary_operator(Rule::power, lhs, rhs),
            Rule::concat => build_formula_binary_operator(Rule::concat, lhs, rhs),
            Rule::equal => build_formula_binary_operator(Rule::equal, lhs, rhs),
            Rule::not_equal => build_formula_binary_operator(Rule::not_equal, lhs, rhs),
            Rule::greater => build_formula_binary_operator(Rule::greater, lhs, rhs),
            Rule::less => build_formula_binary_operator(Rule::less, lhs, rhs),
            Rule::greater_or_equal => {
                build_formula_binary_operator(Rule::greater_or_equal, lhs, rhs)
            }
            Rule::less_or_equal => build_formula_binary_operator(Rule::less_or_equal, lhs, rhs),
            _ => unreachable!(),
        },
    )
}

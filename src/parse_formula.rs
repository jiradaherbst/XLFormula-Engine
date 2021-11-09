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
    //println!("{:#?}", parse_result);
    match parse_result {
        Ok(mut result) => {
            let parse_result = result.next().unwrap();
            Some(parse_result)
        }
        Err(_) => None,
    }
    // GrammarParser::parse(Rule::formula, s)
    //     .expect("unsuccessful parse")
    //     .next()
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
pub fn parse_string_to_formula(
    s: &str,
    f: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
) -> types::Formula {
    match parse_string(&s) {
        Some(parse_result) => match parse_result.as_rule() {
            Rule::expr => build_formula_with_climber(parse_result.into_inner(), f),
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

fn build_formula_blank() -> types::Formula {
    types::Formula::Value(types::Value::Blank)
}

fn build_formula_boolean(boolean_value: bool) -> types::Formula {
    if boolean_value {
        types::Formula::Value(types::Value::Boolean(types::Boolean::True))
    } else {
        types::Formula::Value(types::Value::Boolean(types::Boolean::False))
    }
}

fn build_formula_unary_operator(
    unary_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
) -> types::Formula {
    let op_type = match unary_operation {
        Rule::abs => types::Operator::Function(types::Function::Abs),
        Rule::not => types::Operator::Function(types::Function::Not),
        Rule::negate => types::Operator::Function(types::Function::Negate),
        _ => unreachable!(),
    };
    let operation = types::Expression {
        op: op_type,
        values: vec![build_formula_with_climber(pair.into_inner(), f)],
    };
    types::Formula::Operation(operation)
}

fn build_formula_reference(pair: pest::iterators::Pair<Rule>) -> types::Formula {
    let string = pair.as_str().parse::<String>().unwrap();
    types::Formula::Reference(string)
}

fn build_formula_iterator(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
) -> types::Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        vec.push(build_formula_with_climber(term.into_inner(), f));
    }
    types::Formula::Iterator(vec)
}

fn build_formula_collective_operator(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
) -> types::Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        vec.push(build_formula_with_climber(term.into_inner(), f));
    }
    let op_type = match collective_operation {
        Rule::sum => types::Operator::Function(types::Function::Sum),
        Rule::product => types::Operator::Function(types::Function::Product),
        Rule::average => types::Operator::Function(types::Function::Average),
        Rule::or => types::Operator::Function(types::Function::Or),
        Rule::and => types::Operator::Function(types::Function::And),
        Rule::xor => types::Operator::Function(types::Function::Xor),
        Rule::days => types::Operator::Function(types::Function::Days),
        Rule::right => types::Operator::Function(types::Function::Right),
        Rule::left => types::Operator::Function(types::Function::Left),
        Rule::iff => types::Operator::Function(types::Function::Iff),
        _ => unreachable!(),
    };
    let operation = types::Expression {
        op: op_type,
        values: vec,
    };
    types::Formula::Operation(operation)
}

fn build_formula_custom_function(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
) -> types::Formula {
    let mut vec = Vec::new();
    for field in pair.clone().into_inner() {
        match field.as_rule() {
            Rule::expr =>
            // vec.push(field.into_inner().as_str().parse::<f32>().unwrap()),
            // if custom formula is undefined, then don't have anything to push into vec
            {
                let x = field.into_inner().as_str();
                let y = x.parse::<f32>();
                match y {
                    Ok(_) => vec.push(y.unwrap()),
                    Err(_) => (),
                }
            }
            _ => (),
        }
    }
    let mut ref_string = String::new();
    for field in pair.clone().into_inner() {
        ref_string = match field.as_rule() {
            Rule::reference => field.as_str().parse::<String>().unwrap(),
            _ => ref_string,
        }
    }
    match f {
        Some(f) => match f(ref_string, vec) {
            types::Value::Number(x) => types::Formula::Value(types::Value::Number(x)),
            types::Value::Text(s) => types::Formula::Value(types::Value::Text(s)),
            types::Value::Boolean(x) => types::Formula::Value(types::Value::Boolean(x)),
            types::Value::Error(types::Error::Value) => {
                types::Formula::Value(types::Value::Error(types::Error::Value))
            }
            types::Value::Iterator(v) => types::Formula::Value(types::Value::Iterator(v)),
            types::Value::Date(d) => types::Formula::Value(types::Value::Date(d)),
            types::Value::Blank => types::Formula::Value(types::Value::Blank),
            _ => types::Formula::Value(types::Value::Error(types::Error::Value)), //unreachable!(),
        },
        None => types::Formula::Value(types::Value::Error(types::Error::Formula)),
    }
}

fn build_formula_binary_operator(
    binary_operator: Rule,
    lhs: types::Formula,
    rhs: types::Formula,
) -> types::Formula {
    let op_type = match binary_operator {
        Rule::add => types::Operator::Plus,
        Rule::subtract => types::Operator::Minus,
        Rule::multiply => types::Operator::Multiply,
        Rule::divide => types::Operator::Divide,
        Rule::power => types::Operator::Power,
        Rule::concat => types::Operator::Concat,
        Rule::equal => types::Operator::Equal,
        Rule::not_equal => types::Operator::NotEqual,
        Rule::greater => types::Operator::Greater,
        Rule::less => types::Operator::Less,
        Rule::greater_or_equal => types::Operator::GreaterOrEqual,
        Rule::less_or_equal => types::Operator::LessOrEqual,
        _ => unreachable!(0),
    };
    let operation = types::Expression {
        op: op_type,
        values: vec![lhs, rhs],
    };
    types::Formula::Operation(operation)
}

/// Builds Formula Enum using a `pest-PrecClimber`.
fn build_formula_with_climber(
    expression: pest::iterators::Pairs<Rule>,
    f: Option<&impl Fn(String, Vec<f32>) -> types::Value>,
) -> types::Formula {
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
            Rule::abs => build_formula_unary_operator(Rule::abs, pair, f),
            Rule::sum => build_formula_collective_operator(Rule::sum, pair, f),
            Rule::product => build_formula_collective_operator(Rule::product, pair, f),
            Rule::average => build_formula_collective_operator(Rule::average, pair, f),
            Rule::or => build_formula_collective_operator(Rule::or, pair, f),
            Rule::and => build_formula_collective_operator(Rule::and, pair, f),
            Rule::xor => build_formula_collective_operator(Rule::xor, pair, f),
            Rule::not => build_formula_unary_operator(Rule::not, pair, f),
            Rule::reference => build_formula_reference(pair),
            Rule::iterator => build_formula_iterator(pair, f),
            Rule::negate => build_formula_unary_operator(Rule::negate, pair, f),
            Rule::expr => build_formula_with_climber(pair.into_inner(), f),
            Rule::days => build_formula_collective_operator(Rule::days, pair, f),
            Rule::right => build_formula_collective_operator(Rule::right, pair, f),
            Rule::left => build_formula_collective_operator(Rule::left, pair, f),
            Rule::custom_function => build_formula_custom_function(pair, f),
            Rule::iff => build_formula_collective_operator(Rule::iff, pair, f),
            Rule::blank => build_formula_blank(),
            Rule::params3 => build_formula_with_climber(pair.into_inner(), f),
            //Rule::param => build_formula_with_climber(pair.into_inner(), f),
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

//!# XLFormula Engine
//!XLFormula Engine is a Rust crate for parsing and evaluating Excel formulas. It currently works with f32 types.
//!
//!## Features
//!It supports:
//!
//!* Any numbers, negative and positive, as float or integer
//!* Arithmetic operations +, -, /, *, ^
//!* Logical operations AND(), OR(), NOT(), XOR()
//!* Comparison operations =, >, >=, <, <=, <>
//!* String operation & (concatenation)
//!* Build-in variables TRUE, FALSE
//!* Excel functions ABS(), SUM(), PRODUCT(), AVERAGE(), RIGHT(), LEFT(), IF()
//!* Operations on lists of values (one dimensional range)
//!* Add or subtract dates and excel funtion DAYS()
//!* Custom functions with number arguments
//!* Handle blank/null values in calculation
//!* Handle empty/missing parameters of function calls as blank values
//!
//!## Installation
//!
//!Add the corresponding entry to your Cargo.toml dependency list:
//!```toml
//![dependencies]
//!xlformula_engine = "0.1.16"
//!```
//!and add this to your crate root:
//!```rust
//!extern crate xlformula_engine;
//!```
//!
//!## Examples
//!
//!Here are simple examples of parsing an Excel formula string and evaluating to a result:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=1+2", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=1+3/0", None::<NoCustomFunction>); // error (#DIV/0!)
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!The last string is evaluated to #DIV/0!.
//!
//!Concatenating strings:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=\"Hello \" & \" World!\"", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=1 + \"Hello\"", None::<NoCustomFunction>); // error (#CAST!)
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!Concatenating number and string results in a #CAST! error.
//!
//!Constants ( i.e. a string without '=' ):
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let formula = parse_formula::parse_string_to_formula(&"1.2", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"Hello World", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Excel functions:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=SUM(1,2,\"3\")", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(ABS(1),2*1, 3,4*1)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\", 3)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=LEFT(\"apple\", 3)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=LEFT(\"apple\")", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=IF(TRUE,1,0)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Logical expressions:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let formula = parse_formula::parse_string_to_formula(&"=2>=1", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=OR(1>1,1<>1)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=AND(\"test\",\"True\", 1, true) ", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!References:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::types;
//!use xlformula_engine::NoCustomFunction;
//!
//!let data_function = |s: String| match s.as_str() {
//!"A" => types::Value::Text("=1+B".to_string()),
//!"B" => types::Value::Number(3.0),
//!_ => types::Value::Error(types::Error::Value),
//!};
//!let formula = parse_formula::parse_string_to_formula(&"=A+B", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!List:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let formula = parse_formula::parse_string_to_formula(&"={1,2,3}+{1,2,3}", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));    
//!
//!let formula = parse_formula::parse_string_to_formula(&"=XOR({0,0,0})", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=AVERAGE({1,2,3},1,2,3)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Date:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::types;
//!use chrono::format::ParseError;
//!use chrono::{DateTime, FixedOffset};
//!use xlformula_engine::NoCustomFunction;
//!
//!fn main() -> Result<(), ParseError> {
//!let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
//!let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
//!let data_function = |s: String| match s.as_str() {
//!"start" => types::Value::Date(start),
//!"end" => types::Value::Date(end),
//!_ => types::Value::Error(types::Error::Value),
//!};
//!
//!let formula = parse_formula::parse_string_to_formula(&"=DAYS(end, start)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=start+1", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula = parse_formula::parse_string_to_formula(&"=end-3", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//! Ok(())
//!}
//!```
//!
//!Custom Function:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::types;
//!use xlformula_engine::NoReference;
//!
//!let custom_functions = |s: String, params: Vec<f32>| match s.as_str() {
//!"Increase" => types::Value::Number(params[0] + 1.0),
//!"SimpleSum" => types::Value::Number(params[0] + params[1]),
//!"EqualFive" => types::Value::Number(5.0),
//!_ => types::Value::Error(types::Error::Value),
//!};
//!
//!let formula =
//!parse_formula::parse_string_to_formula(&"=Increase(1)+1", Some(&custom_functions));
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula =
//!parse_formula::parse_string_to_formula(&"=EqualFive()+1", Some(&custom_functions));
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula =
//!parse_formula::parse_string_to_formula(&"=SimpleSum(1,2)", Some(&custom_functions));
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```
//!
//!Handle blank in calculation:
//!```rust
//!extern crate xlformula_engine;
//!use xlformula_engine::calculate;
//!use xlformula_engine::parse_formula;
//!use xlformula_engine::types;
//!use chrono::format::ParseError;
//!use chrono::{DateTime, FixedOffset};
//!use xlformula_engine::NoReference;
//!use xlformula_engine::NoCustomFunction;
//!
//!let data_function = |s: String| match s.as_str() {
//!"B" => types::Value::Blank,
//!_ => types::Value::Error(types::Error::Value),
//!};
//!
//!let custom_functions = |s: String, params: Vec<f32>| match s.as_str() {
//!"BLANK" => types::Value::Blank,
//!_ => types::Value::Error(types::Error::Value),
//!};
//!
//!let formula = parse_formula::parse_string_to_formula(&"=SUM(B, 1)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula =
//!parse_formula::parse_string_to_formula(&"=SUM(BLANK(), 1)", Some(&custom_functions));
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!//takes list as input
//!let formula = parse_formula::parse_string_to_formula(&"=SUM({B, 1})", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, Some(&data_function));
//!println!("Result is {}", calculate::result_to_string(result));
//!
//!let formula =
//!parse_formula::parse_string_to_formula(&"=SUM(1, 2, , 3)", None::<NoCustomFunction>);
//!let result = calculate::calculate_formula(formula, None::<NoReference>);
//!println!("Result is {}", calculate::result_to_string(result));
//!```

#[macro_use]
extern crate pest_derive;

/// Evaluates a formula.
pub mod calculate;

/// The Structs and Enums for the calculation.
pub mod types;

/// Parses a string using `pest` and `pest::prec_climber`.
pub mod parse_formula;

pub type NoReference<'a> = &'a fn(String) -> types::Value;
pub type NoCustomFunction<'a> = &'a fn(String, Vec<f32>) -> types::Value;

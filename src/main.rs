extern crate xlformula_engine;
use chrono::format::ParseError;
use chrono::{DateTime, FixedOffset};
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use xlformula_engine::NoCustomFunction;
use xlformula_engine::NoReference;

fn main() -> Result<(), ParseError> {
    let formula = parse_formula::parse_string_to_formula(&"=1+2", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Text("=1+B".to_string()),
        "B" => types::Value::Number(3.0),
        "C" => types::Value::Text("=1+A".to_string()),
        _ => types::Value::Error(types::Error::Value),
    };
    let formula = parse_formula::parse_string_to_formula(&"=A+B", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));
    let formula = parse_formula::parse_string_to_formula(&"=SUM(A,B,C)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1+2", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1+3/0", None::<NoCustomFunction>); // error (#DIV/0!)
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(
        &"=\"Hello \" & \" World!\"",
        None::<NoCustomFunction>,
    );
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=1 + \"Hello\"", None::<NoCustomFunction>); // error (#CAST!)
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"1.2", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"Hello World", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=SUM(1,2,\"3\")", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(
        &"=PRODUCT(ABS(1),2*1, 3,4*1)",
        None::<NoCustomFunction>,
    );
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=2>=1", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=OR(1>1,1<>1)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(
        &"=AND(\"test\",\"True\", 1, true) ",
        None::<NoCustomFunction>,
    );
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(
        &"=SUM({1,2,3}, 4, {5,6,7})",
        None::<NoCustomFunction>,
    );
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(
        &"=AVERAGE({1,2,3},1,2,3)",
        None::<NoCustomFunction>,
    );
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=XOR({0,0,0})", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"={1,2,3}+{1,2,3}", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"={0,0}+{1,2,3}", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result)); // error (#ARG!)

    let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
    let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
    let data_function = |s: String| match s.as_str() {
        "start" => types::Value::Date(start),
        "end" => types::Value::Date(end),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula =
        parse_formula::parse_string_to_formula(&"=DAYS(end, start)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=start+1", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=end-3", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let custom_functions = |s: String, params: Vec<f32>| match s.as_str() {
        "Increase" => types::Value::Number(params[0] + 1.0),
        "SimpleSum" => types::Value::Number(params[0] + params[1]),
        "EqualFive" => types::Value::Number(5.0),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula =
        parse_formula::parse_string_to_formula(&"=Increase(1)+1", Some(&custom_functions));
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=EqualFive()+1", Some(&custom_functions));
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=SimpleSum(1,2)", Some(&custom_functions));
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let custom_function = |s: String, _params: Vec<f32>| match s.as_str() {
        "EqualFive" => types::Value::Number(5.0),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=EqualFive()", Some(&custom_function));
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    ///////////// RIGHT function
    let formula =
        parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\", 3)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\")", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(
        &"=\"P\"&RIGHT(\"000\"&1,3)",
        None::<NoCustomFunction>,
    );
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));
    ///////////// LEFT function
    let formula =
        parse_formula::parse_string_to_formula(&"=LEFT(\"apple\", 3)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=LEFT(\"apple\")", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    ///////////// Handle blank in calculation
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };

    let custom_functions = |s: String, params: Vec<f32>| match s.as_str() {
        "Increase" => types::Value::Number(params[0] + 1.0),
        "BLANK" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=SUM(B, 1)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=SUM(BLANK(), 1)", Some(&custom_functions));
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let data_function = |s: String| match s.as_str() {
        "T" => types::Value::Boolean(types::Boolean::True),
        "B" => types::Value::Blank,
        "F" => types::Value::Boolean(types::Boolean::False),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=OR({F,B})", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=SUM(1, 2, , 3)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    ///////////// IF function
    let formula =
        parse_formula::parse_string_to_formula(&"=IF(TRUE,1,0)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let data_function = |s: String| match s.as_str() {
        "ReferenceKey" => types::Value::Text("100".to_string()),
        "ReferenceName" => types::Value::Text("Test".to_string()),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(
        &"=IF(ReferenceKey=\"10\",\"\",ReferenceKey&\" - \")&ReferenceName",
        None::<NoCustomFunction>,
    );

    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    Ok(())
}

extern crate xlformula_engine;
use chrono::format::ParseError;
use chrono::{DateTime, FixedOffset};
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use xlformula_engine::{NoCustomFunction, NoReference};

fn main() -> Result<(), ParseError> {
    let formula = parse_formula::parse_string_to_formula(&"=1");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_number(result).unwrap());

    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Text("TEXT".to_string()),
        "B" => types::Value::Number(3.2),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=A");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=B");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_number(result).unwrap());

    let custom_functions = |s: String, params: Vec<types::Value>| match s.as_str() {
        "ONE" => types::Value::Number(params[0].as_num().unwrap() + 1.0),
        "TWO" => types::Value::Number(params[0].as_num().unwrap() + params[1].as_num().unwrap()),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=ONE(1)");
    // println!("=ONE(1) > {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=TWO(1.2,2.1)");
    // println!("=TWO(1.2, 2.1) > {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=ONE(B)");
    // println!("=ONE(B) > {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=A+B");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));
    let formula = parse_formula::parse_string_to_formula(&"=SUM(A,B,C)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1+3/0"); // error (#DIV/0!)
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=\"Hello \" & \" World!\"");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=1 + \"Hello\""); // error (#CAST!)
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"1.2");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"Hello World");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(1,2,\"3\")");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(ABS(1),2*1, 3,4*1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=2>=1");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=OR(1>1,1<>1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=AND(\"test\",\"True\", 1, true) ");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM({1,2,3}, 4, {5,6,7})");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=AVERAGE({1,2,3},1,2,3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=XOR({0,0,0})");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"={1,2,3}+{1,2,3}");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"={0,0}+{1,2,3}");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result)); // error (#ARG!)

    let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
    let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
    let data_function = |s: String| match s.as_str() {
        "start" => types::Value::Date(start),
        "end" => types::Value::Date(end),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=DAYS(end, start)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=start+1");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=end-3");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let custom_functions = |s: String, params: Vec<types::Value>| match s.as_str() {
        "Increase" => types::Value::Number(params[0].as_num().unwrap() + 1.0),
        "SimpleSum" => {
            types::Value::Number(params[0].as_num().unwrap() + params[1].as_num().unwrap())
        }
        "EqualFive" => types::Value::Number(5.0),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=Increase(1)+1");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=EqualFive()+1");
    // println!("Formula[=EqualFive()+1] = {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SimpleSum(1,2)");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let custom_function = |s: String, _params: Vec<types::Value>| match s.as_str() {
        "EqualFive" => types::Value::Number(5.0),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=EqualFive()");
    let result = calculate::calculate_formula(formula, Some(&custom_function), None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    ///////////// RIGHT function
    let formula = parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\", 3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\")");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=\"P\"&RIGHT(\"000\"&1,3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));
    ///////////// LEFT function
    let formula = parse_formula::parse_string_to_formula(&"=LEFT(\"apple\", 3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=LEFT(\"apple\")");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    ///////////// Handle blank in calculation
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };

    let custom_functions = |s: String, params: Vec<types::Value>| match s.as_str() {
        "Increase" => types::Value::Number(params[0].as_num().unwrap() + 1.0),
        "BLANK" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=SUM(B, 1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(BLANK(), 1)");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    let data_function = |s: String| match s.as_str() {
        "T" => types::Value::Boolean(true),
        "B" => types::Value::Blank,
        "F" => types::Value::Boolean(false),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=OR({F,B})");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(1, 2, , 3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    ///////////// IF function
    let formula = parse_formula::parse_string_to_formula(&"=IF(TRUE,1,0)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let data_function = |s: String| match s.as_str() {
        "ReferenceKey" => types::Value::Text("100".to_string()),
        "ReferenceName" => types::Value::Text("Test".to_string()),
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(
        &"=IF(ReferenceKey=\"10\",\"\",ReferenceKey&\" - \")&ReferenceName",
    );

    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    Ok(())
}

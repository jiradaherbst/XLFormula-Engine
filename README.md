# XLFormula Engine
XLFormula Engine is a Rust crate for parsing and evaluating excel formulas. It currently works with f32 types. 

## Features
It supports:

* Any numbers, negative and positive as float or integer;
* Arithmetic operations +, -, /, *, ^;
* Logical operations AND(), OR(), NOT(), XOR();
* Comparison operations =, >, >=, <, <=, <>;
* String operations & (concatenation);
* Build-in variables TRUE, FALSE;
* Excel functions ABS(), SUM(), PRODUCT().

## Installation

Add the corresponding entry to your Cargo.toml dependency list:
```rust
[dependencies]
calculator = "0.1.0"
```
and add this to your crate root:
```rust
extern crate calculator;
```

## Examples

Here is a simple example to parse a string as an excel formula and evaluate a result:
```rust
extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;

fn main() {
    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    let result = calculate::calculate_formula(formula, None);
    println!("Result is {}", calculate::result_to_string(result));
}
```

Another example is for a formula with references:
```rust
extern crate calculator;
use calculator::calculate;
use calculator::parse_formula;
use calculator::types;

fn main() {
    let data_function = |s: String| match s.as_str() {
        "A" => types::Value::Text("=1+B".to_string()),
        "B" => types::Value::Number(3.0),
        _ => types::Value::Error(types::Error::Value),
    };
    let formula = parse_formula::parse_string_to_formula(&"=A+B");
    let result = calculate::calculate_formula(formula, Some(data_function));
    println!("Result is {}", calculate::result_to_string(result));
}
```

## License
The MIT License (see the [LICENSE](https://github.com/jiradaherbst/XLFormula-Engine/blob/master/LICENSE) file for the full text). 

## Contact
Please feel free to contact us at jirada.herbst@data2impact.com.
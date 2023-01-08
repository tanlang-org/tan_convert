use std::fs;

use clap::{Arg, Command};
use serde_json::{json, Map, Value};
use tan::{
    api::eval_string,
    eval::{env::Env, prelude::setup_prelude},
    expr::Expr,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// #TODO implement!
// fn json_to_expr(json: Value) -> Expr {
//     todo!()
// }

/// Convers a symbolic Expr to a JSON Value.
fn expr_to_json<E>(expr: E) -> Value
where
    E: AsRef<Expr>,
{
    let expr = expr.as_ref();

    // #TODO support multi-line strings
    // #TODO Null
    // #TODO somehow encode annotations.

    match expr {
        Expr::Array(exprs) => {
            let mut arr = Vec::new();
            for x in exprs {
                arr.push(expr_to_json(x));
            }
            Value::Array(arr)
        }
        Expr::Dict(dict) => {
            let mut obj = Map::new();
            for (k, v) in dict {
                obj.insert(k.to_string(), expr_to_json(v));
            }
            Value::Object(obj)
        }
        Expr::String(s) => Value::String(s.clone()),
        Expr::Symbol(s) => Value::String(s.clone()),
        Expr::KeySymbol(s) => Value::String(s.clone()),
        Expr::Int(n) => json!(n),
        Expr::Float(n) => json!(n),
        Expr::Bool(b) => Value::Bool(*b),
        _ => Value::String("Unknown".to_string()), // #TODO remove!
    }
}

fn main() -> anyhow::Result<()> {
    let cmd = Command::new("tan_convert")
        .bin_name("tan-convert")
        .author("George Moschovitis, gmosx@reizu.org")
        .version(VERSION)
        .about("A CLI for converting Tan text and binary files from/to other formats")
        .arg(
            Arg::new("INPUT")
                .help("The path of the input file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("The path of the output file")
                .required(true)
                .index(2),
        );

    let matches = cmd.get_matches();

    let input_path: &String = matches
        .get_one("INPUT")
        .expect("missing path to the input file");

    let output_path: &String = matches
        .get_one("OUTPUT")
        .expect("missing path to the output file");

    let input = fs::read_to_string(input_path).expect("cannot read input");

    let mut env = setup_prelude(Env::default());

    let value = eval_string(&input, &mut env)?;

    let json = expr_to_json(&value);

    let json = serde_json::to_string_pretty(&json)?;

    fs::write(output_path, json)?;

    Ok(())
}

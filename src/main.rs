use std::{collections::HashMap, fs};

use anyhow::anyhow;
use clap::{Arg, Command};
use serde_json::{json, Map, Value};
use tan::{
    api::eval_string,
    eval::{env::Env, prelude::setup_prelude},
    expr::Expr,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Converts a JSON Value to a symbolic Expr.
fn json_to_expr(json: Value) -> Expr {
    match json {
        Value::Array(vals) => {
            let mut arr = Vec::new();
            for v in vals {
                arr.push(json_to_expr(v));
            }
            Expr::Array(arr)
        }
        Value::Object(obj) => {
            let mut dict = HashMap::new();
            for (k, v) in obj {
                dict.insert(k.clone(), json_to_expr(v));
            }
            Expr::Dict(dict)
        }
        Value::String(s) => Expr::String(s),
        Value::Number(n) => Expr::Float(n.as_f64().unwrap()), // #TODO handle Int, Float, remove unwrap!
        Value::Bool(b) => Expr::Bool(b),
        Value::Null => Expr::One, // #TODO is Unit the correct mapping?
    }
}

/// Converts a symbolic Expr to a JSON Value.
fn expr_to_json(expr: impl AsRef<Expr>) -> Value {
    let expr = expr.as_ref();

    // #TODO support multi-line strings
    // #TODO support Null
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
        _ => Value::String("Unsupported".to_string()), // #TODO remove!
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

    let output = if input_path.ends_with(".tan") && output_path.ends_with(".json") {
        let mut env = setup_prelude(Env::default());
        let value = eval_string(&input, &mut env)?;
        let json = expr_to_json(&value);
        serde_json::to_string_pretty(&json)?
    } else if input_path.ends_with(".json") && output_path.ends_with(".tan") {
        let json = serde_json::from_str(&input)?;
        let expr = json_to_expr(json);
        expr.to_string()
    } else {
        return Err(anyhow!("unsupported conversion"));
    };

    fs::write(output_path, output)?;

    Ok(())
}

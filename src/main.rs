use clap::{Arg, Command};
use tan::{
    api::eval_string,
    eval::{env::Env, prelude::setup_prelude},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

    let input = std::fs::read_to_string(input_path).expect("cannot read input");

    let mut env = setup_prelude(Env::default());

    let value = eval_string(&input, &mut env);

    dbg!(&value);
    dbg!(&output_path);

    Ok(())
}

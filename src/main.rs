use clap::{Arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()> {
    let cmd = Command::new("tan_convert")
        .bin_name("tan-convert")
        .author("George Moschovitis, gmosx@reizu.org")
        .version(VERSION)
        .about("A CLI for converting Tan text and binary files from/to other formats")
        .arg(
            Arg::new("FROM")
                .help("The path of the file to convert from")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("TO")
                .help("The path of the file to convert to")
                .required(true)
                .index(2),
        );

    let matches = cmd.get_matches();

    dbg!(&matches);

    Ok(())
}

use analyzer::analyzer::VideoAnalyzer;
use clap::{App, Arg};
use std::error::Error;

mod analyzer;
mod lexer;
mod neural_net;
mod parser;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // argument parsing
    let matches = App::new("Knowledge Component Extraction")
        .version("0.1.0")
        .about("Analyzes programming videos and extracts knowledge components")
        .arg(Arg::with_name("url")
                .short("url")
                .long("url")
                .takes_value(true)
                .help("URL as input"))
        .arg(Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Path as input"))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File as input"))
        .get_matches();


    if let Some(url) = matches.value_of("url") {
        let mut analyzer = VideoAnalyzer::new(url);
        analyzer.run()?;
        analyzer.save()?;
    }

    if let Some(path) = matches.value_of("path") {
        println!("Value for input: {}", path);
    } 

    if let Some(file) = matches.value_of("file") {
        println!("Value for input: {}", file);
    }

    Ok(())
}
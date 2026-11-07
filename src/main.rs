use std::{fs, path::PathBuf, process::ExitCode};

use alteraparser::prelude::{AstStrWriter, AstVisualizer};
use clap::{Parser as ClapParser, ValueEnum};
use thobol::parser::Parser;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum AstFormat {
    Xml,
    Dot,
}

#[derive(Debug, ClapParser)]
#[command(name = "thobol")]
#[command(about = "Thobol language processor")]
struct Cli {
    #[arg(help = "Path to a thobol source file")]
    source_path: PathBuf,

    #[arg(long, help = "Parse source and print the AST")]
    parse: bool,

    #[arg(
        long,
        value_enum,
        default_value_t = AstFormat::Xml,
        requires = "parse",
        help = "AST output format"
    )]
    ast_format: AstFormat,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let source = match fs::read_to_string(&cli.source_path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!(
                "Could not read source file '{}': {}",
                cli.source_path.display(),
                err
            );
            return ExitCode::FAILURE;
        }
    };

    if cli.parse {
        return run_parse_only(&source, cli.ast_format);
    }

    eprintln!("No processing option selected. Use --parse to parse and print AST.");
    ExitCode::FAILURE
}

fn run_parse_only(source: &str, ast_format: AstFormat) -> ExitCode {
    let parser = Parser::new();
    let ast = match parser.parse(source) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parse error: {:?}", err);
            return ExitCode::FAILURE;
        }
    };

    let output = match ast_format {
        AstFormat::Xml => AstStrWriter::new(2).write_ast_to_str(&ast),
        AstFormat::Dot => AstVisualizer::new().to_dot_string(&ast),
    };

    println!("{output}");
    ExitCode::SUCCESS
}

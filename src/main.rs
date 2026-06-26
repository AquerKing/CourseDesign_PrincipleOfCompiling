use std::fs::File;

use cdpc::fc::{self, calc::calculate_first_follow_sets, gmr::symbol::SymbolTable};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    grammar: String,

    #[arg(default_value = "output.txt")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.grammar.is_empty() {
        eprintln!("Error: Grammar file path is required.");
        std::process::exit(1);
    }

    let mut symbol_manager = fc::gmr::symbol::SymbolTable::<char>::new();

    let grammar_content = std::fs::read_to_string(args.grammar.to_string()).unwrap();
    let ge_strs = fc::sp::get_ge_from_str(&grammar_content.to_string());
    let ges = fc::gmr::parse_generative_expressions(&ge_strs, &mut symbol_manager);

    let context = fc::gmr::GrammarContext {
        symbol_manager: symbol_manager,
        expressions: ges,
    };

    let (first_sets, follow_sets) = calculate_first_follow_sets(&context);

    let contents = format!(
        "First Sets:\n{}\n\nFollow Sets:\n{}",
        {
            first_sets
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        },
        {
            follow_sets
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        }
    );
    std::fs::write(args.output.unwrap_or_else(|| "output.txt".into()), contents).unwrap();
}

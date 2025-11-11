use clap::{Parser, CommandFactory};
use emoji_sentiment_analyzer::{get_sentiment_label, parse_text};
use std::fs;
use std::process;


#[derive(Parser)]
#[command(version = "0.1.0", about = "Emoji sentiment analyzer", long_about = None)]
enum Cli {

    Analyze {
        #[arg(short, long)]
        file: Option<String>,
    },

    Credits,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Інструкція: використовуйте --help для деталей.");
        Cli::command().print_help().unwrap();
        process::exit(0);
    }

    let cli = Cli::parse();

    match cli {
        Cli::Analyze { file } => {
            let input = match file {

                Some(path) => fs::read_to_string(&path).unwrap_or_else(|e| {
                    eprintln!("error reading file {}", e);
                    process::exit(1);
                }),

                None => "Happy 😊😊 Sad 😞 Neutral 😐🤔 Angry 😡😡😡 Excited 🎉👍".to_string(),
            };
            match parse_text(&input) {
                Ok(score) => {
                    let label = get_sentiment_label(score);
                    println!("Вхідний текст:\n{}", input);
                    println!("Рахунок настрою: {}", score);
                    println!("Загальний настрій: {}", label);
                }

                Err(e) => {
                    eprintln!("Помилка: {}", e);
                    process::exit(1);
                }
            }
        }

        Cli::Credits => {
            println!("Created by Bryk Mykhailo");
            println!("Email: m.bryk@ukma.edu.ua");
        }
    }

}



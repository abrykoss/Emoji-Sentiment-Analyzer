use clap::Parser;
use emoji_sentiment_analyzer::{get_sentiment_label, parse_text};
use std::fs;
use std::process;


#[derive(Parser)]
#[command(version = "0.1.0", about = "Emoji sentiment analyzer")]
struct Cli {

    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();


    let input = match cli.file {
        Some(path) => {

            match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("error reading file {}: {}", path, e);
                    process::exit(1);
                }
            }
        }
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
            eprintln!("Parsing error: {}", e);
            process::exit(1);
        }
    }

}



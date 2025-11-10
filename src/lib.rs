use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct EmojiParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing failed: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
}

pub fn parse_text(input: &str) -> Result<f32, ParseError> {
    let parse = EmojiParser::parse(Rule::text, input)?;

    let mut total = 0.0;
    let mut count = 0;

    for pair in parse.flatten() {

        if pair.as_rule() == Rule::pair {
            let mut emojis = Vec::new();
            for child in pair.into_inner() {

                if child.as_rule() == Rule::emoji {
                    emojis.push(child.as_str());
                }

            }
            if !emojis.is_empty() {

                let base_sentiment = classify_emoji(emojis[0]);
                let intensity = emojis.len() as f32;
                total += base_sentiment * intensity;
                count += 1;
            }

        }
    }

    let score = if count > 0 { total / count as f32 } else { 0.0 };
    Ok(score)
}

fn classify_emoji(emoji: &str) -> f32 {
    match emoji {

        "😊" | "😀" | "😄" | "😂" | "😍" | "🥰" | "👍" | "🎉" => 1.0,
        "😞" | "😢" | "😠" | "😡" | "😭" | "👎" | "💔" | "😤" => -1.0,
        "😐" | "🤔" | "😶" | "🤷" | "📄" | "🔍" => 0.0,
        _ => 0.0,
    }

}





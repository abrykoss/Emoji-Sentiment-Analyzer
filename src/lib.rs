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
    let mut lines = input.lines();

    let mut total_score = 0.0;
    let mut line_count = 0;
    while let Some(line) = lines.next() {

        let parse = EmojiParser::parse(Rule::text, line)?;
        let mut line_total = 0.0;
        let mut pair_count = 0;

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
                    
                    line_total += base_sentiment * intensity;
                    pair_count += 1;
                }

            }
        }

        if pair_count > 0 {
            total_score += line_total / pair_count as f32;
            line_count += 1;
        }
    }
    let score = if line_count > 0 { total_score / line_count as f32 } else { 0.0 };
    
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


pub fn get_sentiment_label(score: f32) -> String {
    if score > 0.5 {
        "strong positive".to_string()

    } 
    else if score > 0.0 {

        "positive".to_string()
    } 
    else if score < -0.5 {

        "strong negative".to_string()
    } 
    else if score < 0.0 {
        "negative".to_string()
    } 
    else {
        "neutral".to_string()
    }
}



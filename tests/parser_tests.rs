#[cfg(test)]
mod tests {
    use anyhow::Result;
    use emoji_sentiment_analyzer::{get_sentiment_label, parse_text};
    use pest::Parser;
    use emoji_sentiment_analyzer::EmojiParser;
    use emoji_sentiment_analyzer::Rule;

#[test]
fn test_rule_word() -> Result<()> {
    let parse = EmojiParser::parse(Rule::word, "Happy")?;
    assert_eq!(parse.as_str(), "Happy");
    Ok(())
}

#[test]
fn test_rule_emoji() -> Result<()> {
    let parse = EmojiParser::parse(Rule::emoji, "😊")?;
    assert_eq!(parse.as_str(), "😊");
    Ok(())
}

#[test]
fn test_rule_pair() -> Result<()> {
    let input = "Sad 😞😞";
    let parse = EmojiParser::parse(Rule::pair, input)?;
    assert_eq!(parse.as_str(), input);
    Ok(())
}

#[test]
fn test_rule_sentence() -> Result<()> {
    let input = "Happy 😊😊.";
    let parse = EmojiParser::parse(Rule::sentence, input)?;
    assert_eq!(parse.as_str(), input);
    Ok(())
}

#[test]
fn test_rule_hashtag() -> Result<()> {
    let input = "#Mood 😊🤔";
    let parse = EmojiParser::parse(Rule::hashtag, input)?;
    assert_eq!(parse.as_str(), input);
    Ok(())
}

    #[test]
    fn test_grammar_rule_complex() -> Result<()> {
        let input = r#"
Happy 😊😊😊
Sad 😞😞
Neutral 😐🤔
Angry 😡😡😡😡
Excited 🎉👍
        "#;
        let score = parse_text(input)?;
        assert_eq!(score, -0.2);
        Ok(())
    }

    // 0
    #[test]
    fn test_grammar_rule_no_emojis() -> Result<()> {
        let input = "Just some text without emojis.";

        let score = parse_text(input)?;
        assert_eq!(score, 0.0);

        assert_eq!(get_sentiment_label(score), "neutral");

        Ok(())
    }

#[test]
fn test_rule_modifier() -> Result<()> {
    let parse = EmojiParser::parse(Rule::modifier, "AND")?;
    assert_eq!(parse.as_str(), "AND");
    
    let parse2 = EmojiParser::parse(Rule::modifier, "OR")?;
    assert_eq!(parse2.as_str(), "OR");
    
    let parse3 = EmojiParser::parse(Rule::modifier, "*")?;
    assert_eq!(parse3.as_str(), "*");
    
    Ok(())
}

#[test]
fn test_rule_text() -> Result<()> {
    let input = "Happy 😊😊. Sad 😞.";
    let parse = EmojiParser::parse(Rule::text, input)?;
    assert_eq!(parse.as_str(), input);
    Ok(())
}

#[test]
fn test_rule_text_empty() -> Result<()> {
    let input = "";
    let parse = EmojiParser::parse(Rule::text, input)?;
    assert_eq!(parse.as_str(), input);
    Ok(())
}

#[test]
fn test_sentence_with_modifier() -> Result<()> {
    let input = "Happy 😊 AND Sad 😞.";
    let parse = EmojiParser::parse(Rule::sentence, input)?;
    assert_eq!(parse.as_str(), input);
    Ok(())
}

}
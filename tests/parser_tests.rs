#[cfg(test)]
mod tests {
    use anyhow::Result;
    use emoji_sentiment_analyzer::{get_sentiment_label, parse_text};

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
}
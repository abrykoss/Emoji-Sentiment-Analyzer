#[cfg(test)]
mod tests {
    use anyhow::Result;
    use emoji_sentiment_analyzer::parse_text;

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
}
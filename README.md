A Rust parser that analyzes text containing emojis to determine overall sentiment with intensity consideration. It parses sequences of words followed by one or more emojis, classifies them into positive, negative, or neutral categories, and computes a weighted sentiment score (accounting for multiple emojis for emphasis). Useful for enhanced analysis of social media messages or chat logs where emoji repetition indicates stronger emotions.

## Description

The parser processes input text (potentially multi-line) to identify and extract structured patterns involving words and associated emojis. It uses a Parsing Expression Grammar (PEG) implemented via the `pest` crate to break down the text into meaningful units.

### Parsing Process
1. **Input**: Arbitrary text, such as social media posts or chat messages, containing words interspersed with emojis (e.g., "This is amazing 😊😊 but disappointing 😞").
2. **Grammar Application**: The grammar scans the text to match sequences where words are followed by one or more emojis, handling variations like multiple emojis per word to capture emotional intensity.
3. **Sentiment Classification**: Extracted emojis are classified based on predefined categories (positive, negative, neutral), with aggregation that weights the sentiment by the number of repeating emojis.
4. **Results and Usage**: The parsing yields a numerical sentiment score for the entire text or per section. These results are used to provide a nuanced sentiment analysis, such as detecting overall mood in user-generated content, enabling applications like automated feedback tools or emotion-based content filtering. For example, a high positive score with intensity could trigger positive reinforcements in chatbots.
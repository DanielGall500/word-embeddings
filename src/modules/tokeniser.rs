pub mod tokeniser {
    pub fn tokenise(sentence: &str) -> Vec<String> {
        let sentence_lower = sentence.to_lowercase();
        let punct_tokens = [' ', '.', ',', '?', '!'];
        let tokens: Vec<String> = sentence_lower.split(&punct_tokens[..]).map(|s| s.to_string()).collect();
        tokens
    }
}
pub mod metrics{
    use std::collections::HashSet;
    use crate::modules::tokeniser::tokeniser;

    pub fn jaccard_similarity(x: &str, y: &str) -> f32 {
        let tokens_x = tokeniser::tokenise(x);
        let tokens_y = tokeniser::tokenise(y);
        let mut token_set_x = HashSet::new();
        let mut token_set_y = HashSet::new();
        
        // improve this code
        for token in &tokens_x {
            token_set_x.insert(token);
        }
        for token in &tokens_y {
            token_set_y.insert(token);
        }

        let intersection_len: f32 = token_set_x.intersection(&token_set_y).count() as f32;
        let union_len: f32 = token_set_x.union(&token_set_y).count() as f32;
        let jaccard_sim: f32 = intersection_len / union_len;
        jaccard_sim
    }
}
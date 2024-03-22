mod modules;
use crate::modules::metrics::metrics::jaccard_similarity;

fn main() {
    println!("Hello, world!");
    let example_a = String::from("This is the first sentence.");
    let example_b = String::from("This is the second sentence.");
    println!("{}", jaccard_similarity(&example_a, &example_b));
}
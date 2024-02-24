
use semanticsimilarity_rs::cosine_similarity;

fn main() {

    let vec1: [f64; 3] = [1.0, 2.0, 3.0];
    let vec2: [f64; 3] = [4.0, 5.0, 6.0];

    let similarity = cosine_similarity(&vec1, &vec2);

    println!("Cosine similarity between vec1 and vec2: {}", similarity);
}
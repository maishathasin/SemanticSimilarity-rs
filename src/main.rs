use std::time::Instant;

use ndarray::{Array1, ArrayView1};
use rayon::prelude::*;


fn main() {

    let start = Instant::now();

    let vec1: [f64; 3] = [1.0, 2.0, 3.0];
    let vec2: [f64; 3] = [4.0, 5.0, 6.0];

    let similarity = cosine_similarity(&vec1, &vec2);

    println!("Cosine similarity between vec1 and vec2: {}", similarity);
    let elapsed = start.elapsed();
    println!("Cosine similarity between vec1 and vec2: {}", similarity);
    println!("Time taken: {:.2?}", elapsed);
}



// hugging face embeddings 
// openai embeddings 
// ollama embeddings 



fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let magnitude1: f64 = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    dot_product / (magnitude1 * magnitude2)
}


fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter().zip(vec2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}


fn manhattan_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter().zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}


fn pearson_correlation(vec1: &[f64], vec2: &[f64]) -> f64 {
    let mean1 = vec1.iter().sum::<f64>() / vec1.len() as f64;
    let mean2 = vec2.iter().sum::<f64>() / vec2.len() as f64;

    let numerator: f64 = vec1.iter().zip(vec2.iter())
        .map(|(a, b)| (a - mean1) * (b - mean2))
        .sum();

    let denom1: f64 = vec1.iter()
        .map(|a| (a - mean1).powi(2))
        .sum::<f64>()
        .sqrt();

    let denom2: f64 = vec2.iter()
        .map(|b| (b - mean2).powi(2))
        .sum::<f64>()
        .sqrt();

    numerator / (denom1 * denom2)
}


fn angular_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    let cosine_sim = cosine_similarity(vec1, vec2);
    cosine_sim.acos() / std::f64::consts::PI
}

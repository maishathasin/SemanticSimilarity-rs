use rayon::prelude::*;

pub fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product: f64 = vec1.par_iter().zip(vec2.par_iter()).map(|(a, b)| a * b).sum();
    let magnitude1: f64 = vec1.par_iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.par_iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    dot_product / (magnitude1 * magnitude2)
}


pub fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.par_iter().zip(vec2.par_iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}


pub fn manhattan_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.par_iter().zip(vec2.par_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}


pub fn pearson_correlation(vec1: &[f64], vec2: &[f64]) -> f64 {
    let mean1 = vec1.par_iter().sum::<f64>() / vec1.len() as f64;
    let mean2 = vec2.par_iter().sum::<f64>() / vec2.len() as f64;

    let numerator: f64 = vec1.par_iter().zip(vec2.par_iter())
        .map(|(a, b)| (a - mean1) * (b - mean2))
        .sum();

    let denom1: f64 = vec1.par_iter()
        .map(|a| (a - mean1).powi(2))
        .sum::<f64>()
        .sqrt();

    let denom2: f64 = vec2.par_iter()
        .map(|b| (b - mean2).powi(2))
        .sum::<f64>()
        .sqrt();

    numerator / (denom1 * denom2)
}


pub fn angular_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    let cosine_sim = crate::cosine_similarity(vec1, vec2);
    cosine_sim.acos() / std::f64::consts::PI
}


pub fn chebyshev_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter())
      .map(|(x, y)| (x - y).abs())
      .fold(0.0, f64::max)
}



pub fn dot_product_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.par_iter().zip(vec2.par_iter()).map(|(a, b)| a * b).sum()
}


pub fn minkowski_distance(vec1: &[f64], vec2: &[f64], p: i32) -> f64 {
    vec1.par_iter().zip(vec2.par_iter())
        .map(|(a, b)| (a - b).abs().powi(p))
        .sum::<f64>()
        .powf(1.0 / p as f64)
}
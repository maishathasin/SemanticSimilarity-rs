# Rusty Semantic Similarity 

![Crates.io Version](https://img.shields.io/crates/v/semanticsimilarity_rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/semanticsimilarity_rs)
![Crates.io License](https://img.shields.io/crates/l/semanticsimilarity_rs)

A small library designed to compute similarity/dissimilarity metrics between embeddings using vector  distance. 

Current distance measures implemented:
- [x] Cosine (handles both normalized and non-normalized vectors)
- [x] Euclidean
- [x] Manhattan
- [x] Chebyshev
- [x] Angular 
- [ ] Jaccard Index
- [ ] Levenshtein 
- [x] Minkowski
- [x] Dot product 



## Features
- Parallel Computation: Utilizes rayon for parallel processing. 
- Bring your own embedding: Use any embedding model to generate embeddings and compute the similarity/distance scores. 



## Installation 

### Add semanticsimilarity_rs to your Cargo.toml file 

```
[dependencies]
semanticsimilarity_rs = "0.1.0" 
```
### Or use cargo add  

```
cargo add semanticsimilarity_rs
```


## Usage 
```
use similarity_metrics::{cosine_distance, euclidean_distance, manhattan_distance, chebyshev_distance};

fn main() {

    let vec1: [f64; 3] = [1.0, 2.0, 3.0];
    let vec2: [f64; 3] = [4.0, 5.0, 6.0];

    let similarity = cosine_similarity(&vec1, &vec2, false);

    println!("Cosine similarity between vec1 and vec2: {}", similarity);
}
```



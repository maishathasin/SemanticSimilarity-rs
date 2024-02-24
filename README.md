# SemanticSimilarity

A small library designed to compute similarity/dissimilarity metrics between embeddings using distance. 

Current distance measures implemented:
- Cosine
- Euclidean
- Manhattan
- Chebyshev
- Angular 



## Features
- Parallel Computation: Utilizes rayon for parallel processing. 
- Bring your own embedding: Use any embedding model to generate embeddings and compute the similarity/distance scores 



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
use similarity_metrics::{cosine_distance, euclidean_distance, manhattan_distance, chebyshev_distance, hamming_distance, compute_score};

fn main() {

    let vec1: [f64; 3] = [1.0, 2.0, 3.0];
    let vec2: [f64; 3] = [4.0, 5.0, 6.0];

    let similarity = cosine_similarity(&vec1, &vec2);

    println!("Cosine similarity between vec1 and vec2: {}", similarity);
}
```




//! Algebra functions related to hashmaps. Hashmap key is assumed to be a vector of u8s, similar to str or string, while the value is assumed to be u64. Returns an f64.

use std::collections::HashMap;

/// Hashmap dot product of hashmap counts
pub fn value_dot(hm1: &HashMap<Vec<u8>, u64>, hm2: &HashMap<Vec<u8>, u64>) -> f64 {
    let dot: u64 = hm1.iter().map(|(key, m1)| {
        match hm2.get(key) {
            Some(m2) => m1*m2,
            None => 0
        }
    }).sum();
    dot as f64
}

/// Computes the hashmap norm (sqrt of dot) of hashmap counts
pub fn value_norm(hm1: &HashMap<Vec<u8>, u64>) -> f64 {
    value_dot(&hm1,&hm1).sqrt() as f64
}

/// Covariance of two hashmap counts
pub fn value_covariance(hm1: &HashMap<Vec<u8>, u64>, hm2: &HashMap<Vec<u8>, u64>, num: f64) -> f64 {
    let s1: u64 = hm1.values().sum();
    let s2: u64 = hm2.values().sum();
    value_dot(&hm1,&hm2) - ((s1*s1 + s2*s2) as f64)/(num)
}

/// Cosine similarity of hashmaps
pub fn value_cosine(hm1: &HashMap<Vec<u8>, u64>, hm2: &HashMap<Vec<u8>, u64>, num: Vec<u8>) -> f64 {
    let norms = value_norm(&hm1)*value_norm(&hm2);
    if norms > 0.0 {
        return value_dot(&hm1,&hm2)/(value_norm(&hm1)*value_norm(&hm2))
    }
    return 0.0;
}

/// Jaccard similarity of two hashmaps
pub fn jaccard(hm1: &HashMap<Vec<u8>, u64>, hm2: &HashMap<Vec<u8>, u64>, num: Vec<u8>) -> f64 {
    let inner = value_dot(&hm1,&hm2);
    inner/(value_dot(&hm1,&hm1)+value_dot(&hm2,&hm2)-inner)
}

/// Pearson correlation of two hashmaps
pub fn pearson(hm1: &HashMap<Vec<u8>, u64>, hm2: &HashMap<Vec<u8>, u64>, num: f64) -> f64 {
    value_covariance(&hm1,&hm2,num)/(value_covariance(&hm1,&hm1,num).sqrt()*value_covariance(&hm2,&hm2,num).sqrt())
}

// fn small_window(s: &str) -> HashSet<String> {
//     let chars: Vec<_> = s.chars().collect();
//     chars.windows(2).map(|w| w.iter().cloned().collect()).collect()
// }

// fn jaccard_distance(s1: &str, s2: &str) -> f64 {
//     let s1_small_window = small_window(s1);
//     let s2_small_window = small_window(s2);
//     let inter = s1_small_window.intersection(&s2_small_window).count();
//     let union = s1_small_window.union(&s2_small_window).count();
//     (inter as f64) / (union as f64)
// }

// // fn main() {
// //     println!("{}", jaccard_distance("Pear", "Peach"));
// // }

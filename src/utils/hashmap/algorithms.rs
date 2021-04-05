// // Copyright 2021 Christopher Sugai

// //! Module with some basic similarities between vectors

// use std::collections::HashMap;
// use super::algebra::*;

// /// Computes the cosine similarity between two sparse vectors.
// /// It returns 0.0 if one of the vectors has null norm.
// #[allow(unused_variables)]
// pub fn value_cosine(a: &HashMap<Vec<u8>, u64>, b: &HashMap<Vec<u8>, u64>, n: Vec<u8>) -> f64 {
//     let norms = value_norm(&a)*value_norm(&b);
//     if norms > 0.0 {
//         return value_dot(a,b)/(value_norm(a)*value_norm(b))
//     }
//     return 0.0;
// }

// /// Computes the extended Jaccard similarity between two sparse vectors.
// #[allow(unused_variables)]
// pub fn jaccard(a: &HashMap<Vec<u8>, f64>, b: &HashMap<Vec<u8>, f64>, n: Vec<u8>) -> f64 {
//     let inner = dot(a,b);
//     inner/(dot(a,a)+dot(b,b)-inner)
// }

// /// Computes the Pearson correlation index between two sparse vectors.
// /// It treats the vectors as discrete uniform random variables.
// pub fn pearson(a: &HashMap<Vec<u8>, f64>, b: &HashMap<Vec<u8>, f64>, n: Vec<u8>) -> f64 {
//     covariance(a,b,n)/(covariance(a,a,n).sqrt()*covariance(b,b,n).sqrt())
// }

// use std::collections::HashSet;

// fn shingles(s: &str) -> HashSet<String> {
//     let chars: Vec<_> = s.chars().collect();
//     chars.windows(2).map(|w| w.iter().cloned().collect()).collect()
// }

// fn jaccard_distance(s1: &str, s2: &str) -> f64 {
//     let s1_shingles = shingles(s1);
//     let s2_shingles = shingles(s2);
//     let inter = s1_shingles.intersection(&s2_shingles).count();
//     let union = s1_shingles.union(&s2_shingles).count();
//     (inter as f64) / (union as f64)
// }

// // fn main() {
// //     println!("{}", jaccard_distance("Pear", "Peach"));
// // }
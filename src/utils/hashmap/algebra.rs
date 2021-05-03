// Copyright (c) 2021 Kana LLC

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


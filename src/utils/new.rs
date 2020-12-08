// Copyright 2020 Christopher Sugai

use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

const DNA_NUCLEOTIDES: [u8; 4] = [b'A', b'C', b'G', b'T'];
const RNA_NUCLEOTIDES: [u8; 4] = [b'A', b'C', b'G', b'U'];



pub fn dna(length: u8, mut rng: ThreadRng) -> String {
    let bases = ['A', 'C', 'T', 'G'];
    // let nucl: String = 

};

// In this random generator, we assume 50/50 chance of U vs T. Refer to base_percents function to specify percentages of bases.
pub fn rna(length: u8, mut rng: ThreadRng) -> String {
    let bases = ['A', 'C', 'T', 'G'];
};

pub fn aa(length: u8, mut rng: ThreadRng) -> String {

};

fn new_seq(n_bases: u8) -> String {
    let mut rng = rand::thread_rng();
    let bases = ['A', 'C', 'T', 'G'];
    let seq = String::new();
    for n_bases in 1u8..=n_bases{
        seq.push(bases.choose(&mut rng).unwrap())
    }
seq
}
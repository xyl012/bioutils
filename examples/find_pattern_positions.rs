// Copyright (c) 2021 Kana LLC

//! This example finds positions of an arbitrary sequence or CG sequences.

use bioutils::utils::get::item::all_positions;
use bioutils::utils::get::item::cg_positions;

fn main(){
    let dna = b"ACTGCGACG".to_vec();
    let dna2 = b"ACTGCGACG".to_vec();
    let target: u8 = 65; // b"A"
    let matching = all_positions(&dna, |x| x == &&target);
    let cg = cg_positions(&dna2);
    println!("The sequence ACTGCGACG is {:?} in ascii numbers, we're trying to find target patterns", dna);
    println!("target: {:?}", target);
    println!("matching pattern position: {:?}", matching);
    println!("cpg position: {:?}", cg);
}
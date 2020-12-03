// Copyright 2020 Christopher Sugai

//! Functions to replace non-ACTG characters with pseudorandom bases.
//! # Examples
//!
//!
//! 
//! 
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

// convert U to T
pub fn replace_u_with_t(nucl: String) -> String {
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            'U' => 'T',
            'u' => 't',
            _ => ch,
        })
        .collect();
    nucl
}

// fill gaps {.,-} with pseudorandom nucleotides ACTG
pub fn replace_gap(nucl: String, mut rng: ThreadRng) -> String {
    let bases = ['A', 'C', 'T', 'G'];
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            '.' => *bases.choose(&mut rng).unwrap(),
            '-' => *bases.choose(&mut rng).unwrap(),
            _ => ch,
        })
        .collect();
    nucl
}

// fill gaps {.,-} with pseudorandom nucleotides actg
pub fn replace_gap_lowercase(nucl: String, mut rng: ThreadRng) -> String {
    let bases = ['a', 'c', 't', 'g'];
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            '.' => *bases.choose(&mut rng).unwrap(),
            '-' => *bases.choose(&mut rng).unwrap(),
            _ => ch,
        })
        .collect();
    nucl
}

// fill N with pseudorandom nucleotides ACTG and n with actg
pub fn replace_n(nucl: String, mut rng: ThreadRng) -> String {
    let bases = ['A', 'C', 'T', 'G'];
    let lowercase_bases = ['a', 'c', 't', 'g'];
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            'N' => *bases.choose(&mut rng).unwrap(),
            'n' => *lowercase_bases.choose(&mut rng).unwrap(),
            _ => ch,
        })
        .collect();
    nucl
}

// pseudorandom nucleotide replacements within IUPAC specifications, e.g. R: either A or G. Does not fill N's or gaps. Refer to other functions for these.
pub fn replace_iupac(nucl: String, mut rng: ThreadRng) -> String {
    let r_bases = ['A', 'G'];
    let r_bases_lowercase = ['a', 'g'];
    let y_bases = ['C', 'T'];
    let y_bases_lowercase = ['c', 't'];
    let s_bases = ['C', 'G'];
    let s_bases_lowercase = ['c', 'g'];
    let w_bases = ['A', 'T'];
    let w_bases_lowercase = ['a', 't'];
    let k_bases = ['T', 'G'];
    let k_bases_lowercase = ['t', 'g'];
    let m_bases = ['A', 'C'];
    let m_bases_lowercase = ['a', 'c'];
    let b_bases = ['C', 'T', 'G'];
    let b_bases_lowercase = ['c', 't', 'g'];
    let d_bases = ['A', 'T', 'G'];
    let d_bases_lowercase = ['a', 't', 'g'];
    let h_bases = ['A', 'C', 'T'];
    let h_bases_lowercase = ['a', 'c', 't'];
    let v_bases = ['A', 'C', 'G'];
    let v_bases_lowercase = ['a', 'c', 'g'];
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            'R' => *r_bases.choose(&mut rng).unwrap(),
            'r' => *r_bases_lowercase.choose(&mut rng).unwrap(),
            'Y' => *y_bases.choose(&mut rng).unwrap(),
            'y' => *y_bases_lowercase.choose(&mut rng).unwrap(),
            'S' => *s_bases.choose(&mut rng).unwrap(),
            's' => *s_bases_lowercase.choose(&mut rng).unwrap(),
            'W' => *w_bases.choose(&mut rng).unwrap(),
            'w' => *w_bases_lowercase.choose(&mut rng).unwrap(),
            'K' => *k_bases.choose(&mut rng).unwrap(),
            'k' => *k_bases_lowercase.choose(&mut rng).unwrap(),
            'M' => *m_bases.choose(&mut rng).unwrap(),
            'm' => *m_bases_lowercase.choose(&mut rng).unwrap(),
            'B' => *b_bases.choose(&mut rng).unwrap(),
            'b' => *b_bases_lowercase.choose(&mut rng).unwrap(),
            'D' => *d_bases.choose(&mut rng).unwrap(),
            'd' => *d_bases_lowercase.choose(&mut rng).unwrap(),
            'H' => *h_bases.choose(&mut rng).unwrap(),
            'h' => *h_bases_lowercase.choose(&mut rng).unwrap(),
            'V' => *v_bases.choose(&mut rng).unwrap(),
            'v' => *v_bases_lowercase.choose(&mut rng).unwrap(),
            _ => ch,
        })
        .collect();
    nucl
}

/// fill all other than ACGT with pseudorandom nucleotides ACTG. Should be used last after other functions or for cleanup of unknown characters.
pub fn replace_all(nucl: String, mut rng: ThreadRng) -> String {
    let bases = ['A', 'C', 'T', 'G'];
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            'A' => ch,
            'a' => ch,
            'C' => ch,
            'c' => ch,
            'T' => ch,
            't' => ch,
            'G' => ch,
            'g' => ch,
            'U' => ch,
            'u' => ch,
            _ => *bases.choose(&mut rng).unwrap(),
        })
        .collect();
    nucl
}

// fill all other than ACGT with pseudorandom nucleotides actg. Should be used after other functions for cleanup of unknown characters.
pub fn replace_all_lowercase(nucl: String, mut rng: ThreadRng) -> String {
    let bases = ['a', 'c', 't', 'g'];
    let nucl: String = nucl
        .chars()
        .map(|ch| match ch {
            'A' => ch,
            'a' => ch,
            'C' => ch,
            'c' => ch,
            'T' => ch,
            't' => ch,
            'G' => ch,
            'g' => ch,
            'U' => ch,
            'u' => ch,
            _ => *bases.choose(&mut rng).unwrap(),
        })
        .collect();
    nucl
}

// #[test]
// fn test_rand_replace() {
//     let mut rng = rand::thread_rng();
//     let test = replace_all("ACTGuyUNn.-@<^>".to_string());
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

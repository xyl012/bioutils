// Copyright 2021 Christopher Sugai

use std::io::{Write};
use seq_io::fastq::Record;
use std::collections::HashSet;
use std::iter::FromIterator;

// Takes a reader and a fastq field ("seq", "head", or "qual") type and returns a hashset of all reads' specified field
pub fn hashset_fastq(
    mut reader: seq_io::fastq::Reader<flate2::read::GzDecoder<std::fs::File>>,
    field: &str,
) -> std::collections::HashSet<Vec<u8>> {
    let mut hashset = HashSet::new();
    while let Some(record) = reader.next() {
        //if cannot read the record skip it
        let result = match record {
            Ok(record) => record,
            Err(_record) => continue
        };
        if field == "seq" {
            hashset.insert(result.seq().to_owned());
        } else if field == "head" {
            hashset.insert(result.head().to_owned());
        } else if field == "qual" {
            hashset.insert(result.qual().to_owned());
        }
        else {
            println!(
                "Specified fastq field is not a seq_io record field. Specify seq, head, or qual"
            )
        }
    }
    hashset
}

// Make struct of vector of readers, as atac has 3 reads
// Takes a vector of readers and creates a single hashset of common headers. Used to get which reads are paired.
// TODO: If head is not ascii will give error
pub fn fastq_head_inner_join(hashset_vector: Vec<seq_io::fastq::Reader<flate2::read::GzDecoder<std::fs::File>>>)
 -> std::collections::HashSet<Vec<u8>> 
{
    let mut hashset = HashSet::new();
    let mut inner_join_hashset = HashSet::new();
    for mut vector_hashset in hashset_vector {
        while let Some(record) = vector_hashset.next() {
            // if cannot read the record skip it
            let record = match record {
                Ok(record) => record,
                Err(_record) => continue
            };
            hashset.insert(record.head().to_owned());
        }
        let inner_join_hashset: HashSet<std::vec::Vec<u8>> = inner_join_hashset.intersection(&hashset).cloned().collect();
    }
    inner_join_hashset
}

// pub fn fastq_head_inner_join(
//     mut reader1: seq_io::fastq::Reader<flate2::read::GzDecoder<std::fs::File>>,
//     mut reader2: seq_io::fastq::Reader<flate2::read::GzDecoder<std::fs::File>>) -> std::collections::HashSet<Vec<u8>> {
//     let mut hashset1 = HashSet::new();
//     let mut hashset2 = HashSet::new();
//     while let Some(record) = reader1.next() {
//         // if cannot read the record skip it
//         let record = match record {
//             Ok(record) => record,
//             Err(_record) => continue
//         };
//         hashset1.insert(record.head().to_owned());
//     }
//     while let Some(record) = reader2.next() {
//         // if cannot read the record skip it
//         let record = match record {
//             Ok(record) => record,
//             Err(_record) => continue
//         };
//         hashset2.insert(record.head().to_owned());
//     }
//     let hashsetx: HashSet<std::vec::Vec<u8>> = hashset1.intersection(&hashset2).cloned().collect();
//     hashsetx
// }

        // match field {
        //     "seq" => hashset.insert(result.seq().to_owned()),
        //     "head" => hashset.insert(result.seq().to_owned()),
        //     "qual" => hashset.insert(result.seq().to_owned()),
        //     Err(_field) => println!("Specified fastq field is not a seq_io record field. Specify seq, head, or qual")
        // };

// Takes a reader and a fastq field ("seq", "head", or "qual") type and returns a hashset of all reads' specified field
// String version

// pub fn hashset_fastq(
//     mut reader: seq_io::fastq::Reader<std::fs::File>,
//     field: &str,
// ) -> HashSet<std::string::String> {
//     let mut hashset = HashSet::new();
//     while let Some(record) = reader.next() {
//         //if cannot read the record skip it
//         let result = match record {
//             Ok(record) => record,
//             Err(_record) => continue
//         };
//         if field == "seq" {
//             let seq = str::from_utf8(&result.seq()).unwrap().to_string();
//             hashset.insert(seq);
//         } else if field == "head" {
//             let head = str::from_utf8(&result.head()).unwrap().to_string();
//             hashset.insert(head);
//         } else if field == "qual" {
//             let qual = str::from_utf8(&result.qual()).unwrap().to_string();
//             hashset.insert(qual);
//         }
//         else {
//             println!(
//                 "Specified fastq field is not a seq_io record field. Specify seq, head, or qual"
//             )
//         }
//     }
//     hashset
// }

//         // } else if field == "id" {
//         //     let id = str::from_utf8(&result.()).unwrap().to_string();
//         //     hashset.insert(qual);
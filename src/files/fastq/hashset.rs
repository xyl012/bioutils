// Copyright 2021 Christopher Sugai

use std::io::{Write};
use seq_io::fastq::Record;
use std::collections::HashSet;

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
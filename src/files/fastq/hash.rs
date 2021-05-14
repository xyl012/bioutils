
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Cursor, Read};
use std::iter::FromIterator;
use seq_io::fastq::{Reader,Record};
use std::fs::File;
use std::io::BufWriter;
use crate::files::fastq::util::*;

// Takes a reader and a fastq field ("seq", "head", or "qual") type and returns a hashset of all reads' specified field
pub fn hashset_fastq<T>(mut reader: seq_io::fastq::Reader<T>, field: &str, format: &str) -> std::collections::HashSet<Vec<u8>> where T: std::io::Read {
    let mut hashset = HashSet::new();
    while let Some(record) = reader.next() {
        //if cannot read the record skip it
        let result = match record {
            Ok(record) => record,
            Err(record) => continue
        };
        if field == "seq" {
            hashset.insert(result.seq().to_owned());
        } else if field == "head" {
            if format == "illumina" {
                hashset.insert(remove_illumina_read_number(&mut result.head().to_owned()).to_vec());
            } else if format == "sra" {
            hashset.insert(trim_sra(&mut result.head().to_owned()).to_vec());
            }
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

/// Find paired reads in two fastqs. Takes two seq io readers and a specification of format (illumina or sra) and creates a single hashset of common headers.
pub fn find_paired_fastq_reads<T>(reader1: seq_io::fastq::Reader<T>, reader2: seq_io::fastq::Reader<T>, field: &str, format: &str)
-> HashSet<Vec<u8>> where
    T: std::io::Read
{
    let hs1 = hashset_fastq(reader1, &field, &format);
    let hs2 = hashset_fastq(reader2, &field, &format);
    hs1.intersection(&hs2).cloned().collect::<HashSet<Vec<u8>>>()
}

// Takes a reader and a fastq field ("seq", "head", or "qual") type and returns a hashmap with count of each occurrence
pub fn hashmap_count_fastq<T>(mut reader: seq_io::fastq::Reader<T>, field: &str, format: &str) -> HashMap<Vec<u8>, u64> where T: std::io::Read {
    let mut hashmap:HashMap<Vec<u8>, u64> = HashMap::new();
    while let Some(record) = reader.next() {
        //if cannot read the record skip it
        let result = match record {
            Ok(record) => record,
            Err(record) => continue
        };
        if field == "seq" {
            let seq = result.seq().to_owned();
            *hashmap.entry(seq).or_insert(1u64) += 1u64;
        } else if field == "head" {
            if format == "illumina" {
                let head = remove_illumina_read_number(&mut result.head().to_owned()).to_vec();
                *hashmap.entry(head).or_insert(1u64) += 1u64;
            } else if format == "sra" {
                let head = trim_sra(&mut result.head().to_owned()).to_vec();
                *hashmap.entry(head).or_insert(1u64) += 1u64;
            }
        } else if field == "qual" {
            let qual = result.qual().to_owned();
            *hashmap.entry(qual).or_insert(1u64) += 1u64;
        }
        else {
            println!(
                "Specified fastq field is not a seq_io record field. Specify seq, head, or qual"
            )
        }
    }
    hashmap
}

// pub fn write_paired_fastq_reads(){}
// let reader = Reader::from_path("seqs.fastq").unwrap();
// let mut writer = BufWriter::new(File::create("filtered.fastq").unwrap());


// // Make vector of seq_io fastq readers, as atac has 3 reads
// /// Find paired reads in a fastq. Takes a vector of readers and creates a single hashset of common headers.
// pub fn find_paired_fastq_reads<T>(reader_vector: Vec<seq_io::fastq::Reader<T>>)
// -> HashSet<Vec<u8>> where
//     T: std::io::Read
// {
//     let mut inner_join_hashset = HashSet::new();
//     for mut vector_hashset in reader_vector {
//         let mut hashset = HashSet::new();
//         while let Some(record) = vector_hashset.next() {
//             // if cannot read the record skip it
//             let record = match record {
//                 Ok(record) => record,
//                 Err(record) => continue
//             };
//             let name = record.head().to_owned();
//             hashset.insert(name);
//         }
//         let inner_join_hashset = inner_join_hashset.intersection(&hashset).cloned().collect::<HashSet<Vec<u8>>>();
//     }
//     inner_join_hashset
// }




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

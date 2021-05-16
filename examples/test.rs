
// use std::path::Path;
// use std::borrow::Borrow;
// use std::fs::File;

// use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
// use bioutils::utils::check::value::CheckU8;
// use suffix_array::SuffixArray;
// use noodles::fastq;
// use std::{
//     io::{BufReader},
// };

// fn main()-> std::io::Result<()>{
//     let references_directory = std::path::Path::new("./data/references/");
//     let samples_directory = std::path::Path::new("./data/samples/");
//     let fastq_gz = "40.fastq.gz";
//     let reference_name = "GRCh38.primary_assembly.genome.fa.gz";
//     let fastq_file = &samples_directory.join("40.fastq.gz");

//     let mut reader =  File::open(fastq_file)
//     .map(flate2::read::GzDecoder::new)
//     .map(BufReader::new)
//     .map(fastq::Reader::new)?;

//     let mut record = fastq::Record::default();
//     let mut n = 0;
//     let mut n_homopolymers = 0;

//     loop {
//         match reader.read_record(&mut record) {
//             Ok(0) => break,
//             Ok(_) => {n += 1; if record.sequence().is_percent_homopolymer(&90).unwrap() {n_homopolymers +=1; println!("Read header: {:?}", record.read_name()); } else {println!("Read header: {:?}", record.quality_scores()); continue}}, // Count homopolymers
//             Err(e) => return Err(e),
//         }
//     }
    
//     println!("Reads read: {}", n);
//     println!("Number of homopolymers: {}", n_homopolymers);
//     Ok(())
// }

// use bioutils::utils::check::value::Check;
use bioutils::utils::check::value::CheckU8;
use bioutils::utils::get::item::GetItemU8;
fn main(){
    let test = b"ACTG";
    test.cut_to_length(&6);
    test.is_homopolymer();
}
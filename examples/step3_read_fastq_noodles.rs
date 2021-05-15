
use std::borrow::Borrow;
use std::fs::File;

use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
use bioutils::utils::check::value::CheckU8;
use suffix_array::SuffixArray;
use noodles::fastq;
use std::{
    // env,
    // fs::File,
    io::{BufReader},
};

fn main()-> std::io::Result<()>{

    // Running everything from Step 1

    // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references/");
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz = "SRR1700869.fastq.gz";
    let reference_name = "GRCh38.primary_assembly.genome.fa.gz";
    let fastq_file = &samples_directory.join("SRR1700869.fastq.gz");
    
    // // Step 1 Complete!
    // // Running everything from Step 2

    // // Now we start downloading our files with a lot of printing as we go.
    // // This may fail or give an error if the connection is interrupted. Please try again and be patient if so.
    // println!("Downloading reference with: ");
    // println!("download_grch38_primary_assembly_genome_fa_gz()");
    // println!("...Please Wait...");
    // println!("...If over 10 minutes, there may be a connection issue...");

    // download_grch38_primary_assembly_genome_fa_gz(&references_directory);

    // println!("Reference functions take a path, we make this with std::path::path::new()");
    // println!("Downloading fastq with: ");
    // println!("bioutils::files::http::curl()");
    // println!("...Please Wait...");
    // println!("...If over 10 minutes, there may be a connection issue...");

    // // Example Illumina format
    // bioutils::files::http::curl(fastq_ftp, fastq_gz, &samples_directory);

    // println!("We now have the reference in our references directory");
    // println!("Let's check which are available");

    // Get our reference paths
    let ref_paths = std::fs::read_dir(&references_directory).unwrap();
    // Print our reference paths
    for ref_path in ref_paths {
        println!("Reference: {}", ref_path.unwrap().path().display())
    }
    println!("We now have the sample data in our samples directory");
    println!("Let's check which are available");
    let smpl_paths = std::fs::read_dir(&samples_directory).unwrap();

    // Step 2 Complete!
    // Running everything from Step 3
    for smpl_path in smpl_paths {
        let mut reader =  File::open(smpl_path.unwrap().path())
        .map(flate2::read::GzDecoder::new)
        .map(BufReader::new)
        .map(fastq::Reader::new)?;
    
        let mut record = fastq::Record::default();
        let mut n = 0;
        let mut n_homopolymers = 0;

        loop {
            match reader.read_record(&mut record) {
                Ok(0) => break,
                Ok(_) => n += 1,
                Err(e) => return Err(e),
            }
            match reader.read_record(&mut record) {
                Ok(0) => break,
                Ok(_) => if record.sequence().is_homopolymer() {} else {continue}, //
                Err(e) => return Err(e),
            }
            println!("Reads read: {}", n);
        }
        println!("Reads read: {}", n);

    }
    Ok(())
}

// fn ih(seq:&[u8])-> bool {
//     seq.into_iter().all(|x| x==&b'A')
// }

// pub trait TestU8<T, C> {
//     fn ih(&self) -> bool;
// }

// impl<T,C> TestU8<T,C> for T where
//     C: Borrow<u8>,
//     T: IntoIterator<Item = C>,
//     C: PartialEq,
//     T: ExactSizeIterator,
// {
// impl<'a, T, C> dyn TestU8<T, C> where
// C: Borrow<u8>,
// T: IntoIterator<Item = C>,
// {
    // type Item = &'a [u8];

// impl<T,C> IntoIterator for T {
//     type Item = i8;
//     type IntoIter = PixelIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         PixelIntoIterator {
//             pixel: self,
//             index: 0,
//         }
//     }
// }
//     fn ih(&self)-> bool {
//         self.into_iter().all(|x| x==b'A')
//     }
// }

// impl<T, const N: usize> Iterator for IntoIter<T, N> {
//     type Item = T;
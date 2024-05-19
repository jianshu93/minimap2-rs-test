use minimap2::Aligner;
use needletail::{parse_fastx_file, Sequence, FastxReader};
use std::path::Path;
use num_cpus;
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ref_path = "/Users/jianshuzhao/Github/minimap2-rs-test/test_data/SAR11_silva_16S.fasta";
    let num_threads = num_cpus::get();
    let aligner = Aligner::builder()
    // we need overlap alignment for compute alignment ratio and identity
    .ava_pb()
    .with_index_threads(num_threads)
    .with_cigar()
    .with_index(ref_path, None)
    .expect("Unable to build index");

    // Test in string byte mode
    let seq: Vec<u8> = b"TGAGAGTTTGATCATGGCTCAGAACGTACGCTGGCGGCACGCCTAACACATGCAAGTCGAACGAAGTAGCAATACTTAGTGGCAAACGGGTGAGTAATATGTGGGAATCTACCCTTCGGTCTGGAATAACATGAGGAAACTTATGCTAATACCGGATAATCCTTTACAGGGAAAGCTTTATGCTCCGATTGATGAGTCCACACTTGATTAGTTAGTTGGCGAGGTAATGGCTCACCAAGACAATGATCAATAGCTGATTTGAGAGGATGATCAGCCACATTGGGACTGAGACACGGCCCAAACTCCTACGGGAGGCAGCAGTGGGGAATCTTGCACAATGGGGGAAACCCTGATGCAGCGATGCCGCGTGAGTGAAGAAGGCCCTTGGGTTGTAAAGCTCTTTCGTCGGGGAAGAAAATGACTGTACCCGAATAAGAAGGTCCGGCTAACTTCGTGCCAGCAGCCGCGGTAATACGAAGGGACCTAGCGTAGTTCGGAATTACTGGGCTTAAAGAGCACGTAGGTGGTTAAAAAAGTTGGTGGTGAAATCCCAGAGCTTAACTCTGGAACTGCCATCAAAACTTTTTAGCTAGAGTATGATAGAGGAAAGTAGAA".to_vec();
    let alignment = aligner
        .map(&seq, false, false, None, None)
        .expect("Unable to align");
    println!("{:?}", alignment);

    // Testing map with parallelization from files
    let query_path = "/Users/jianshuzhao/Github/minimap2-rs-test/test_data/test_16S_SAR11.fa";
    let mut reader = parse_fastx_file(&query_path).expect("valid path/file");
    let mut sequences = Vec::new();
    while let Some(result) = reader.next() {
        let record = result?;
        sequences.push(record.seq().to_vec());
    }
    let results: Vec<_> = sequences.par_iter().map(|seq| {
        aligner.map(seq, false, false, None, None).unwrap()  // Consider handling this `unwrap` more safely in production
    }).collect();


    // Print results of the parallelized alignments
    //for (i, result) in results.iter().enumerate() {
    //    println!("Parallel sequence alignment {} : {:?}", i + 1, result);
    //}

    Ok(())
}

use minimap2::Aligner;
use needletail::{parse_fastx_file, Sequence, FastxReader};
use std::path::Path;
use num_cpus;
use rayon::prelude::*;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ref_path = "/Users/jianshuzhao/Github/minimap2-rs-test/test_data/SAR11_silva_16S.fasta";
    let num_threads = num_cpus::get();
    let aligner = Aligner::builder()
    // we need overlap alignment for compute alignment ratio and identity. Query alignment ratio may be larger than 1 (only a little bit)
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
    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();

    // Producer thread: reads sequences and sends them to the channel
    let producer = thread::spawn(move || {
        let mut reader = parse_fastx_file(&query_path).expect("valid path/file");
        while let Some(result) = reader.next() {
            let record = result.expect("Error reading record");
            sender.send(record.seq().to_vec()).expect("Error sending sequence");
        }
    });

    // Consumer threads: receive sequences and perform alignment
    let consumers: Vec<_> = (0..num_threads).map(|_| {
        let receiver = receiver.clone();
        let aligner = aligner.clone(); // Clone aligner for each thread if possible, or recreate if needed
        thread::spawn(move || {
            receiver.iter().filter_map(|seq: Vec<u8>| { // Modify type here
                aligner.map(&seq, false, false, None, None).ok()
            }).collect::<Vec<_>>()
        })
    }).collect();

    // Wait for the producer to finish reading
    producer.join().expect("Producer thread panicked");

    // Collect results from consumer threads
    let mut results = Vec::new();
    for consumer in consumers {
        let mut res = consumer.join().expect("Consumer thread panicked");
        results.append(&mut res);
    }

    // Print results of the parallelized alignments
    //for (i, result) in results.iter().enumerate() {
    //    println!("Parallel sequence alignment {} : {:?}", i + 1, result);
    //}

    Ok(())
}

## Testing minimap2-rs for parallelization
This is just to test whether minimap2-rs crate works for real-world data. I will use it in Nonpareil crate for estimation metagenomic coverage and diversity.

Minimap2 only works for highly similar sequences like >85% or even 90% sequence identity. For bacterial genomes we can expect ID to be as low as 30%. We can use other faster methods such as kmer-based MinHash to speed up the comparisons for distantly related sequences. 

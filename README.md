## Testing minimap2-rs for parallelization
This is just to test whether minimap2-rs (Rust wrapper around the C implementation) crate works for real-world data via implementing some parallelization. I will use it in Nonpareilx crate for estimation of metagenomic coverage and diversity. 

The seed-chain-extend heuristic in various software packages such as minimap2 is theoretically guaranteed (Shaw and  Yu, 2023). The derivation is based on syncmer but not minimzer although in practice minimzer and (open)syncmer make no big difference in terms of accuracy for highly similar sequences (see Heng Li's blog here: https://lh3.github.io/2022/10/21/random-open-syncmers) and a recent paper using parameterized syncmer (Figure 6b) (Dutta et.al., 2022). It is clear that syncmer can improve mapping accuracy but only marginally at the expense of additional computation for various identity levels. For bacterial genomes we can expect ID to be as low as 50%. We can use other faster methods such as kmer-based MinHash to speed up the comparisons for distantly related sequences. 


## Benchmark
We use vsearch --allpairs_global to compare the results from minimap2 overlap alignment (semi-global). Both query alignment ratio and identity can be benchmarked. 
## References
```bash
@article{li2018minimap2,
  title={Minimap2: pairwise alignment for nucleotide sequences},
  author={Li, Heng},
  journal={Bioinformatics},
  volume={34},
  number={18},
  pages={3094--3100},
  year={2018},
  publisher={Oxford University Press}
}
@article{dutta2022parameterized,
  title={Parameterized syncmer schemes improve long-read mapping},
  author={Dutta, Abhinav and Pellow, David and Shamir, Ron},
  journal={PLOS Computational Biology},
  volume={18},
  number={10},
  pages={e1010638},
  year={2022},
  publisher={Public Library of Science San Francisco, CA USA}
}
@article{shaw2023proving,
  title={Proving sequence aligners can guarantee accuracy in almost O (m log n) time through an average-case analysis of the seed-chain-extend heuristic},
  author={Shaw, Jim and Yu, Yun William},
  journal={Genome Research},
  volume={33},
  number={7},
  pages={1175--1187},
  year={2023},
  publisher={Cold Spring Harbor Lab}
}
```

use counter::Counter;

pub fn ba1b(dna: &str, k: usize) -> Vec<&str> {
    let kmer_counts = dna.as_bytes().windows(k).collect::<Counter<_>>();
    let max_frequency = match kmer_counts.most_common().get(0) {
        Some(pair) => pair.1,
        None => return vec![],
    };
    let mut most_common_kmers: Vec<&str> = Vec::new();
    for (kmer, frequency) in kmer_counts.most_common_ordered() {
        if frequency < max_frequency {
            break;
        } else {
            most_common_kmers.push(std::str::from_utf8(kmer).unwrap())
        }
    }
    most_common_kmers
}

// rosalind ba1a
pub fn ba1a(dna: &[u8], pattern: &[u8]) -> i32 {
    let k = pattern.len();
    dna.windows(k).filter(|&kmer| kmer == pattern).count() as i32
}

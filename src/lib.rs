pub mod rosalind {
    pub fn ba1a(dna: &[u8], pattern: &[u8]) -> i32 {
        let k = pattern.len();
        dna.windows(k).filter(|&kmer| kmer == pattern).count() as i32
    }
}

#[cfg(test)]
mod rosalind_test {
    use super::rosalind::*;

    #[test]
    fn test_ba1a_1() {
        let dna = b"GCGCG";
        let pattern = b"GCG";
        assert_eq!(ba1a(dna, pattern), 2);
    }
}

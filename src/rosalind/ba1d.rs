pub fn ba1d(dna: &str, pattern: &str) -> Vec<i32> {
    let pat_bytes = pattern.as_bytes();
    let pat_length = pat_bytes.len();
    dna.as_bytes()
        .windows(pat_length)
        .enumerate()
        .filter(|(_, window)| *window == pat_bytes)
        .map(|(i, _)| i as i32)
        .collect::<Vec<i32>>()
}

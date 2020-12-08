use std::collections::{HashMap, HashSet};

fn update<'a>(common_words: &mut HashSet<&'a [u8]>, word_frequency: &mut HashMap<&'a [u8], i32>, word: &'a [u8], t: i32) {
    if !common_words.contains(word) {
        let count = *(word_frequency.get(word).unwrap_or(&0)) + 1;
        word_frequency.insert(&word, count);
        if count == t {
            common_words.insert(word);
        }
    }
}

pub fn ba1e(dna: &str, kmer_length: usize, window: usize, t: i32) -> Vec<String> {
    let dna_bytes = dna.as_bytes();
    if dna_bytes.len() < window {
        return vec![];
    }
    let bytes_end = dna_bytes.len();
    let mut word_frequency: HashMap<&[u8], i32> = HashMap::new();
    let mut common_words: HashSet<&[u8]> = HashSet::new();

    // process the first window
    for word in dna_bytes[..window].windows(kmer_length) {
        update(&mut common_words, &mut word_frequency, word, t);
    }

    let mut window_start = 1 as usize;
    while (window_start + window) < bytes_end {
        // this is the word moving out of the window
        let word = &dna_bytes[(window_start - 1)..(window_start + kmer_length - 1)];
        // if the word we are dropping is not a common word, we need to reduce its frequency by 1
        let count = *(word_frequency.get(word).unwrap_or(&0)) - 1;
        word_frequency.insert(word, count);

        let word = &dna_bytes[(window_start + window - kmer_length)..(window_start + window)];
        update(&mut common_words, &mut word_frequency, word, t);

        window_start += 1;
    }
    let mut result = common_words
        .iter()
        .map(|&word| std::str::from_utf8(word).unwrap().to_string())
        .collect::<Vec<String>>();
    result.sort();
    result
}

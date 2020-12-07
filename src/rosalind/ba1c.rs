use std::collections::HashMap;

pub fn ba1c(dna: &str) -> String {
    let mut complements: HashMap<u8, u8> = HashMap::new();
    complements.insert(b'A', b'T');
    complements.insert(b'T', b'A');
    complements.insert(b'G', b'C');
    complements.insert(b'C', b'G');
    let comp_bases = dna
        .as_bytes()
        .iter()
        .map(|base| match complements.get(base) {
            Some(complement) => *complement,
            None => b'N',
        })
        .rev()
        .collect::<Vec<u8>>();
    String::from_utf8(comp_bases).unwrap()
}

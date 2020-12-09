use std::cmp::Ordering;

pub fn ba1f(dna: &str) -> Vec<usize> {
    let mut skew = 0;
    let mut min_skew = (dna.len() + 1) as i32;
    let mut smallest_skews: Vec<usize> = Vec::new();
    for (i, b) in dna.bytes().enumerate() {
        let old_skew = skew;
        if b == b'C' {
            skew -= 1;
        } else if b == b'G' {
            skew += 1;
        }
        match old_skew.cmp(&min_skew) {
            Ordering::Less => {
                min_skew = old_skew;
                // if we found a new minimum skew, toss the previous list of
                // smallest skews because it is now invalid
                smallest_skews = Vec::new();
                smallest_skews.push(i);
            },
            Ordering::Equal => smallest_skews.push(i),
            _ => {}
        }
    }

    match skew.cmp(&min_skew) {
        Ordering::Less => vec![dna.len()],
        _ => {
            if skew == min_skew {
                smallest_skews.push(dna.len());
            }
            smallest_skews
        }
    }
}

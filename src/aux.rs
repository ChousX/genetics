use rand::{thread_rng, Rng};

#[allow(dead_code)]
pub fn segment_dna(max: usize, len: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    let mut segments = Vec::new();
    let mut len = len;

    while len > 0 {
        let segment = {let segment = rng.gen_range(segments[segments.len()]..max);
            if segment >= len {
                len
            } else {
                segment
            }
        };
        len -= segment;
        segments.push(segment);
    }
    segments
}


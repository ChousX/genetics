use super::nucleotide::{Nucleotide, NucleotideSegment, A, C, DNA, G, T};
use std::{
    convert::From,
    fmt::{self, write},
};

// Preconditions: Chromosome.len() % 4 == 0
#[derive(Debug, PartialEq, Clone)]
pub struct Chromosome(Vec<u8>);
impl Chromosome {}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut concat = String::new();
        for s in self.0.iter() {
            concat = format!("{}{}", concat, s);
        }
        write!(f, "|{}|", concat)
    }
}

impl From<DNA> for Chromosome {
    fn from(item: DNA) -> Chromosome {
        if item.len() % 4 != 0 {
            panic!("Tryed to converted a DNA strand with a len not divisable by 4!")
        }
        let mut output: Vec<u8> = Vec::with_capacity(item.len() / 4);
        for i in (0..item.len()).step_by(4) {
            output.push(NucleotideSegment([item[i], item[i + 1], item[i + 2], item[i + 3]]).into());
        }
        Chromosome(output)
    }
}

impl From<Chromosome> for DNA {
    fn from(item: Chromosome) -> DNA {
        let mut output: DNA = Vec::with_capacity((item.0).len() * 4);
        for d in (item.0).into_iter() {
            let nucleotide_segment: NucleotideSegment = d.into();
            let raw_segment = nucleotide_segment.0;

            output.push(raw_segment[0]);
            output.push(raw_segment[1]);
            output.push(raw_segment[2]);
            output.push(raw_segment[3]);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dna_to_chromosome() {
        //                      |   0      |    85     |    27     |
        let sample_a: DNA = vec![A, A, A, A, G, G, G, G, A, G, C, T];
        let expected_output = Chromosome(vec![0, 85, 27]);
        let result: Chromosome = sample_a.clone().into();
        assert_eq!(expected_output, result);
    }

    #[test]
    fn chromosome_to_dna() {
        let sample_a = Chromosome(vec![0, 85, 27]);
        //                             |   0      |    85     |    27     |
        let expected_output: DNA = vec![A, A, A, A, G, G, G, G, A, G, C, T];
        let invers_output: DNA = vec![T, C, G, A, G, G, G, G, A, A, A, A];
        let result: DNA = sample_a.clone().into();
        assert_eq!(expected_output, result);
        assert_ne!(invers_output, result);
    }

    #[test]
    fn convertion() {
        //                      |   0      |    85     |    27     |
        let sample_a: DNA = vec![A, A, A, A, G, G, G, G, A, G, C, T];
        let sample_b = Chromosome(vec![0, 85, 27]);
        let result: DNA = sample_b.clone().into();
        assert_eq!(sample_a, result);
        let result: Chromosome = result.into();
        assert_eq!(sample_b, result);
    }
}

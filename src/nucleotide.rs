use std::{cmp::PartialEq, convert::From, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nucleotide {
    Adenine = 0,
    Guanine = 1,
    Cytosine = 2,
    Thymine = 3,
}

pub use Nucleotide::{Adenine as A, Cytosine as C, Guanine as G, Thymine as T};

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                A => {
                    "A"
                }
                G => {
                    "G"
                }
                C => {
                    "C"
                }
                T => {
                    "T"
                }
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NucleotideSegment(pub [Nucleotide; 4]);
pub type DNA = Vec<Nucleotide>;

impl From<NucleotideSegment> for u8 {
    fn from(item: NucleotideSegment) -> u8 {
        let mut output = 0u8;
        let n0 = (item.0)[0] as u8;
        let n1 = (item.0)[1] as u8;
        let n2 = (item.0)[2] as u8;
        let n3 = (item.0)[3] as u8;
        output = output | n0;
        output = output << 2;
        output = output | n1;
        output = output << 2;
        output = output | n2;
        output = output << 2;
        output = output | n3;
        output
    }
}

impl From<u8> for NucleotideSegment {
    fn from(item: u8) -> NucleotideSegment {
        //mask for the last 2 bits
        const MASK: u8 = 0b00000011;
        let mut item = item;
        let mut output = [A; 4];
        //run through the bits looking at the last two every time and converting them in to
        //Nucleotides
        for i in 0..4 {
            let n = match item & MASK {
                0 => A,
                1 => G,
                2 => C,
                3 => T,
                _ => unreachable!(),
            };
            //placing the n in the highest remaining index in the array
            output[3 - i] = n;
            item = item >> 2;
        }
        NucleotideSegment(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_A: NucleotideSegment = NucleotideSegment([A, A, A, A]); //0
    const SAMPLE_B: NucleotideSegment = NucleotideSegment([G, G, G, G]); //85
    const SAMPLE_C: NucleotideSegment = NucleotideSegment([T, T, T, T]); //225
    const SAMPLE_D: NucleotideSegment = NucleotideSegment([T, A, A, A]); //192
    const SAMPLE_E: NucleotideSegment = NucleotideSegment([T, C, G, A]); //228

    #[test]
    fn u8_from_nucleotide_segment() {
        assert_eq!(0u8, SAMPLE_A.into());
        assert_eq!(85u8, SAMPLE_B.into());
        assert_eq!(255u8, SAMPLE_C.into());
        assert_eq!(192u8, SAMPLE_D.into());
    }

    #[test]
    fn nucleotide_segment_from_u8() {
        assert_eq!(SAMPLE_A, 0u8.into());
        assert_eq!(SAMPLE_B, 85u8.into());
        assert_eq!(SAMPLE_C, 255u8.into());
        assert_eq!(SAMPLE_D, 192u8.into());
    }
    #[test]
    fn convertion() {
        //repeated convertion
        let result: u8 = SAMPLE_D.into();
        assert_eq!(192u8, result);
        let result: NucleotideSegment = result.into();
        assert_eq!(SAMPLE_D, result);

        //a bit more on a harder case
        let result: u8 = SAMPLE_E.into();
        assert_eq!(228, result);
        let result: NucleotideSegment = result.into();
        assert_eq!(SAMPLE_E, result);
        let result: u8 = result.into();
        assert_eq!(228, result);
    }
}

use std::{cmp::PartialEq, fmt};

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

impl Nucleotide{
    pub fn to_u8(item: &[Nucleotide]) -> u8{
       //converting Nucleotides to n = 0..4
       let mut output = 0u8;
       let n0 = item[0] as u8;
       let n1 = item[1] as u8;
       let n2 = item[2] as u8;
       let n3 = item[3] as u8;
       //pushing bits in to a single u8
       output = output | n0;
       output = output << 2;
       output = output | n1;
       output = output << 2;
       output = output | n2;
       output = output << 2;
       output = output | n3;
       output
    }

    pub fn to_nucleotides(item: u8) -> [Nucleotide;4]{
        //mask for the last 2 bits
        const MASK: u8 = 0b00000011;
        let mut item = item;
        let mut output = [A; 4];
        //run through the bits looking at the last two every time and converting them in 
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
        output
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
}

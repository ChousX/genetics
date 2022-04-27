use crate::{genetic::data::GeneticData, dna::DNA, nucleotide::Nucleotide};

use std::convert::From;
use std::fmt;
// Preconditions: Chromosome.len() % 4 == 0
#[derive(Debug, PartialEq, Clone)]
pub struct Chromosome(Vec<u8>);

impl Chromosome{
    pub fn get_dna(&self) -> DNA{
        let mut output = Vec::with_capacity(self.0.len());
        for u in self.0.iter(){
            let n = Nucleotide::to_nucleotides(*u);
            output.extend_from_slice(&n);
        }
        output
    }

    pub fn new(input: DNA) -> Self{
        if input.len() % 4 != 0 {
            panic!("the dna provide is not a facter of 4");
        }

        let mut output = Vec::with_capacity(input.len() / 4);
        for i in (0..input.len()).step_by(4){
            output.push(Nucleotide::to_u8(&input[i..(i + 4)]));
        }
        Self(output)
    }
}

impl Genetic for Chromosome{
    fn get_dna(&self) -> DNA{
        let mut output = Vec::with_capacity(self.0.len());
        for u in self.0.iter(){
            let n = Nucleotide::to_nucleotides(*u);
            output.extend_from_slice(&n);
        }
        output
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn convertion() {
    }
}

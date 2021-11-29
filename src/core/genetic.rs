use super::chromosome::Chromosome;
use super::nucleotide::{Nucleotide, A, C, DNA, G, T};
use rand::{thread_rng, Rng};
use std::{
    convert::From,
    fmt::{self, write},
};

#[derive(Debug, Clone)]
pub enum GeneticType {
    Chromosome(Chromosome),
    Dna(DNA),
}

impl GeneticType {
    fn to_dna(self) -> DNA {
        match self {
            Self::Chromosome(c) => {
                let d: DNA = c.into();
                d
            }
            Self::Dna(d) => d,
        }
    }

    fn to_chromosome(self) -> Chromosome {
        match self {
            Self::Chromosome(c) => c,
            Self::Dna(d) => {
                let c: Chromosome = d.into();
                c
            }
        }
    }
}


impl From<GeneticType> for Chromosome {
    fn from(item: GeneticType) -> Chromosome {
        item.to_chromosome()
    }
}

impl From<Chromosome> for GeneticType {
    fn from(item: Chromosome) -> GeneticType {
        GeneticType::Chromosome(item)
    }
}

impl From<DNA> for GeneticType {
    fn from(item: DNA) -> GeneticType {
        GeneticType::Dna(item)
    }
}

impl From<GeneticType> for DNA {
    fn from(item: GeneticType) -> DNA {
        item.to_dna()
    }
}

pub trait Genetic {
    fn compress(&self) -> GeneticType;
    fn generait(input: GeneticType) -> Self;
}

//make sure they are the same len
pub fn prim_crossover(
    one: GeneticType,
    two: GeneticType,
    min_segment: usize,
    max_segment: usize,
) -> DNA {
    let one: DNA = one.into();
    let two: DNA = two.into();

    if one.len() != two.len() {
        panic!("crossover requires = len of both items");
    }
    if min_segment <= 0 {
        panic!("min_segment violation <= 0");
    }
    let len = one.len();
    if max_segment > len {
        panic!("max_segment violation > len");
    }

    let mut rng = thread_rng();

    let mut output = Vec::with_capacity(len);
    let mut i = 0;
    while i < len {
        let mut segment = rng.gen_range(min_segment..max_segment);
        if i + segment >= len {
            segment = len - i;
        }
        let slice = if rng.gen_bool(1.0) {
            &one[i..(i + segment)]
        } else {
            &two[i..(i + segment)]
        };
        output.append(&mut slice.to_vec());
        i += segment;
    }
    output
}

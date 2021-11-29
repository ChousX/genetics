mod mutation;
use super::{genetic::*};

pub struct Genome(Vec<GeneticType>);

pub trait Organism {
    fn get_genome(&self) -> Genome;
    fn spawn_from(input: Genome) -> Self;
    fn crossover(one: &Self, two: &Self) -> Genome{
        let one = one.get_genome().0;
        let two = two.get_genome().0;
        let len = one.len();

        let mut output: Vec<GeneticType> = Vec::with_capacity(len);
        for i in 0..len {
            let dna = prim_crossover(one[i].clone(), two[i].clone(), 1, 4);
            output.push(dna.into())
        }

        Genome(output)
    }
}



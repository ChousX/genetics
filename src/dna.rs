use super::{nucleotide::*, genetic::*};
use rand::prelude::*;

pub type DNA = Vec<Nucleotide>;

pub type Chromosome = Vec<u8>;

pub fn crossover<T>(one: &T, two: &T, min_fragment: Option<usize>, max_fragment: Option<usize>, rng: Option<&mut ThreadRng>) -> T where T: Genetic{
    let one_dna = one.get_dna();
    let two_dna = two.get_dna();
    let min_fragment = min_fragment.unwrap_or(1);
    let max_fragment = max_fragment.unwrap_or(4);
    let mut rng = rng.unwrap_or(&mut thread_rng()).to_owned();

    let mut output_dna = Vec::with_capacity(one_dna.len());
    let mut i = 0;
    while 
        i < one_dna.len()
    {
        let fragment = rng.gen_range(min_fragment..max_fragment);
        let f = if fragment + i < one_dna.len() { i + fragment } else { one_dna.len() };
        output_dna.extend_from_slice(
            if rng.gen_bool(0.5) {
                &one_dna[i..f]
            } else {
                &two_dna[i..f]
            });
        i += fragment;
    }
    T::from_dna(&output_dna)
}
#[cfg(test)]
mod tests{
    use super::*;

    #[derive(Debug)]
    struct Dog{
        size: f32,
    }

    impl Dog{
        fn new(input: f32) -> Self{
            Self{
                size: input
            }
        }
    }

    impl Genetic for Dog{
        fn get_dna(&self) -> DNA {
            self.size.get_dna()
        }

        fn from_dna(input: &[Nucleotide]) -> Self {
            Self{
                size: f32::from_dna(input)
            }
        }

        fn size(&self) -> usize {
            16
        }
    }

    #[test]
    fn crossover_base(){
        let dog0 = Dog::new(10.0);
        let dog1 = Dog::new(10.0);

        let dog2 = crossover(&dog0, &dog1, None, None, None);

        assert_eq!(dog0.size, dog2.size);
    }
}

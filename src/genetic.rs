use super::{dna::{DNA, Chromosome}, nucleotide::Nucleotide};

pub trait Genetic: Sized{
    fn get_dna(&self) -> DNA;
    fn from_dna(input: &[Nucleotide]) -> Self;
    fn size(&self) -> usize;
}


impl Genetic for Chromosome{
    fn get_dna(&self) -> DNA {
        let mut output = Vec::with_capacity(self.len() * 4);
        for u in self.iter(){
            let n = Nucleotide::to_nucleotides(*u);
            output.extend_from_slice(&n);
        }
        output
    }

    fn from_dna(input: &[Nucleotide]) -> Self {
        if input.len() % 4 != 0 {
            panic!("the dna provide is not a facter of 4");
        }

        let mut output = Vec::with_capacity(input.len() / 4);
        for i in (0..input.len()).step_by(4){
            output.push(Nucleotide::to_u8(&input[i..(i + 4)]));
        }
        output
    }

    fn size(&self) -> usize {
        self.len() * 4
    }
}




impl Genetic for String{
    fn get_dna(&self) -> DNA {
        //this is an easy way
        self.clone().as_bytes().to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self {
        String::from_utf8(Chromosome::from_dna(input)).unwrap()
    }

    fn size(&self) -> usize {
        self.len() * 4
    }
}

impl Genetic for u8{
    fn get_dna(&self) -> DNA {
        Nucleotide::to_nucleotides(*self).to_vec()
    }

    fn from_dna(input: &[Nucleotide]) -> Self {
        Nucleotide::to_u8(input)
    }

    fn size(&self) -> usize {
        4
    }

}

impl Genetic for u16{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let bytes = Chromosome::from_dna(input);
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    fn size(&self) -> usize {
        8
    }

}

impl Genetic for u32{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let input = Chromosome::from_dna(input);
        let bytes = [input[0], input[1], input[2], input[3]];
        Self::from_le_bytes(bytes)
    }

    fn size(&self) -> usize {
        16
    }

}

impl Genetic for u64{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let input = Chromosome::from_dna(input);
        let bytes = [input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]];
        Self::from_le_bytes(bytes)
    }

    fn size(&self) -> usize {
        32 
    }
}





impl Genetic for i8{
    fn get_dna(&self) -> DNA {
        Nucleotide::to_nucleotides(self.to_le_bytes()[0]).to_vec()
    }

    fn from_dna(input: &[Nucleotide]) -> Self {
        let bytes = Nucleotide::to_u8(input);
        Self::from_le_bytes([bytes])
    }

    fn size(&self) -> usize {
        4
    }

}

impl Genetic for i16{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }


    fn from_dna(input: &[Nucleotide]) -> Self{
        let bytes = Chromosome::from_dna(input);
        Self::from_le_bytes([bytes[0], bytes[1]])
    }

    fn size(&self) -> usize {
        8
    }

}

impl Genetic for i32{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let input = Chromosome::from_dna(input);
        let bytes = [input[0], input[1], input[2], input[3]];
        Self::from_le_bytes(bytes)
    }

    fn size(&self) -> usize {
        16
    }

}

impl Genetic for i64{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let input = Chromosome::from_dna(input);
        let bytes = [input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]];
        Self::from_le_bytes(bytes)
    }

    fn size(&self) -> usize {
        32 
    }
}impl Genetic for f32{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let input = Chromosome::from_dna(input);
        let bytes = [input[0], input[1], input[2], input[3]];
        Self::from_le_bytes(bytes)
    }

    fn size(&self) -> usize {
        16
    }
}

impl Genetic for f64{
    fn get_dna(&self) -> DNA {
        let bytes = self.to_le_bytes();
        bytes.to_vec().get_dna()
    }

    fn from_dna(input: &[Nucleotide]) -> Self{
        let input = Chromosome::from_dna(input);
        let bytes = [input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]];
        Self::from_le_bytes(bytes)
    }

    fn size(&self) -> usize {
        32
    }
}

//lots of tests are needed... uhg
#[cfg(test)]
mod tests{
    use super::*;
    use crate::nucleotide::{A, G, C, T};

    const D4_0: [Nucleotide; 4] = [A, A, A, A];
    const D8_0: [Nucleotide; 8] = [A, A, A, A, A, A, A, A];
    const D16_0: [Nucleotide; 16] = [A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A];
    const D32_0: [Nucleotide; 32] = [A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A,A, A, A, A, A, A, A, A];

    const D4_85: [Nucleotide; 4] = [G, G, G, G];
    const D4_27: [Nucleotide; 4] = [A, G, C, T];

    const D12_0_85_27: [Nucleotide; 12] = [A, A, A, A, G, G, G, G, A, G, C, T];
    const D16_00: [Nucleotide; 16] = [A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A];
    const D32_00: [Nucleotide; 32] = [A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A, A,A, A, A, A, A, A, A, A];

    const C1_0: [u8;1] = [0];
    const C2_0: [u8;2] = [0, 0];
    const C4_0: [u8;4] = [0, 0, 0, 0];
    const C8_0: [u8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
    
    const C3_0_85_27: [u8; 3] = [0, 85, 27];

    #[test]
    fn nucleotides_to_u8(){
        assert_eq!(0, Nucleotide::to_u8(&D4_0));
        assert_ne!(u8::MAX, Nucleotide::to_u8(&D4_0));
        
        assert_eq!(D4_0.to_vec(), 0u8.get_dna());
        assert_ne!(D4_0.to_vec(), 1u8.get_dna());
    }

    #[test]
    fn chromosome_compretion_0(){
        assert_eq!(0, Chromosome::from_dna(&D4_0.to_vec())[0]);
        assert_eq!(vec![0,0], Chromosome::from_dna(&D8_0.to_vec()));
        assert_eq!(vec![0,0, 0, 0], Chromosome::from_dna(&D16_0.to_vec()));
        assert_eq!(vec![0, 0,0, 0, 0, 0, 0, 0], Chromosome::from_dna(&D32_0.to_vec()));

        
    }

    #[test]
    fn chromosome_decompretion_0(){
        assert_eq!(D4_0.to_vec(), C1_0.to_vec().get_dna());
        assert_eq!(D8_0.to_vec(), C2_0.to_vec().get_dna());
        assert_eq!(D16_0.to_vec(),C4_0.to_vec().get_dna());
        assert_eq!(D32_0.to_vec(),C8_0.to_vec().get_dna());
    }
    
    #[test]
    fn chromosome_decompretion(){
        assert_eq!(D12_0_85_27.to_vec(), C3_0_85_27.to_vec().get_dna());
    }


}

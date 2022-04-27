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

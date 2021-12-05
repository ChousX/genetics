mod nucleotide;
mod chromosome;
mod genetic;

pub use nucleotide::*;
pub use chromosome::*;
pub use genetic::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn basic() {}
}


#![feature(const_generics)]
#![feature(const_generic_impls_guard)]
#![feature(trivial_bounds)]

#![recursion_limit = "1024"]

extern crate ff;
extern crate rand;

pub mod const_repr;
// mod alt;

#[cfg(test)]
mod tests {
    use super::const_repr::*;

    #[test]
    fn make_naive() {
        let u256 = BigintRepresentation::<4>([0u64; 4]);
        // println!("{:?}", u256);
    }
}
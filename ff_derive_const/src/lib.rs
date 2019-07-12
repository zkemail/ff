
#![feature(const_generics)]
#![feature(const_generic_impls_guard)]

#![recursion_limit = "1024"]

extern crate ff;
extern crate rand;

pub mod const_repr;
pub mod const_field_element;
// mod alt;

#[cfg(test)]
mod tests {
    use super::const_repr::*;

    #[test]
    fn make_naive() {
        let u256 = BigintRepresentation::<4>([0u64; 4]);
        println!("{:?}", u256);
    }
}
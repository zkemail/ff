use crate::const_repr::BigintRepresentation;
use crate::const_repr::FullMultiplication;

use ff::*;

// #[macro_use]
// use crunchy::*;

pub struct PrimeFieldElement<
    P,
    const N: usize
>(pub BigintRepresentation<{N}>, std::marker::PhantomData<P>) 
    where BigintRepresentation<{N}>: FullMultiplication, 
        P: FieldParameters<{N}>, 
        [u64; N]: std::array::LengthAtMost32;

pub trait FieldParameters<const N:usize> {
    const MODULUS: BigintRepresentation<{N}>;
    const R: BigintRepresentation<{N}>;
    const R2: BigintRepresentation<{N}>;
    const INV: u64;
}

impl<P, const N: usize> PrimeFieldElement<P, {N}> 
    where P: FieldParameters<{N}>, 
    [u64; N]: std::array::LengthAtMost32 
{
    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.0 < P::MODULUS
    }

    #[inline(always)]
    fn reduce(&mut self) {
        if !self.is_valid() {
            self.0.sub_noborrow(&P::MODULUS);
        }
    }

    #[inline(always)]
    fn mont_reduce(&mut self, mut mul_res:< BigintRepresentation<{N}> as FullMultiplication >::MulResult) {
        let mut carry2 = 0u64;

        for j in 0..N {
            let mut carry = 0u64;
            let k = mul_res.0[j].wrapping_mul(P::INV);
            for i in 0..N {
                mul_res.0[i+j] = ::ff::mac_with_carry(mul_res.0[i+j], k, P::MODULUS.0[i], &mut carry);
            }
            mul_res.0[N + j] = ::ff::adc(mul_res.0[{N} + j], carry2, &mut carry);
            carry2 = carry;
        }

        for j in 0..N {
            (self.0).0[j] = (mul_res.0)[N + j];
        }

        self.reduce();
    } 

    #[inline(always)]
    fn mul_assign(&mut self, other: &Self) {
        let mut interm = < BigintRepresentation<{N}> as FullMultiplication >::MulResult::default();

        for j in 0..N {
            let mut carry = 0u64;
            let this_limb = (self.0).0[j];
            for i in 0..N {
                interm.0[i+j] = ::ff::mac_with_carry(interm.0[i+j], this_limb, (other.0).0[i], &mut carry);
            }
            interm.0[N + j] = carry;
        }

        self.mont_reduce(interm);
    }
}
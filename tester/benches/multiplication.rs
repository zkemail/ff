extern crate ff;
extern crate rand;

use self::ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
#[PrimeFieldGenerator = "2"]
struct Fr(FrRepr);

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(always)]
fn mul<F: PrimeField>(a: F, b: &F) -> F {
    let mut c = a;
    c.mul_assign(b);

    c
}

fn multiplication_benchmark(c: &mut Criterion) {
    use rand::{Rng, XorShiftRng, SeedableRng};

    let rng = &mut XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
    let a: Fr = rng.gen();
    let b: Fr = rng.gen();

    c.bench_function("Mont mul 256", |bencher| bencher.iter(|| mul(black_box(a), &black_box(b))));
}

fn mul_assing_benchmark(c: &mut Criterion) {
    use rand::{Rng, XorShiftRng, SeedableRng};

    let rng = &mut XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
    let a: Fr = rng.gen();
    let b: Fr = rng.gen();

    c.bench_function("Mont mul assign 256", |bencher| bencher.iter(|| black_box(a).mul_assign(&black_box(b))));
}

criterion_group!(benches, multiplication_benchmark, mul_assing_benchmark);
criterion_main!(benches);
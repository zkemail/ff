use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
#[PrimeFieldGenerator = "2"]
struct Fr(FrRepr);

#[test]
fn test_to_hex() {
    let a = Fr::from_repr(FrRepr::from(2)).unwrap();
    assert_eq!("0000000000000000000000000000000000000000000000000000000000000002", to_hex(&a));
    println!("`2` into hex = {}", to_hex(&a));
}
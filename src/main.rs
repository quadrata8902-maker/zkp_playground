mod curve;
mod field;
mod poly;
mod r1cs;

use curve::{Curve, ECPoint};
use field::FieldElement;
use poly::Polynomial;
use r1cs::R1CS;

fn fe(value: u64, prime: u64) -> FieldElement {
    FieldElement::new(value, prime)
}

fn test_field(prime: u64) {
    let a = fe(7, prime);
    let b = fe(5, prime);

    assert_eq!((a + b).value, 12);
    assert_eq!((a - b).value, 2);
    assert_eq!((a * b).value, 1);
    assert_eq!(a.pow(3).value, 3);
    assert_eq!((a / b).value, 15);
}

fn test_curve(prime: u64) {
    let curve = Curve {
        a: fe(2, prime),
        b: fe(2, prime),
    };

    let p = ECPoint::Point {
        x: fe(5, prime),
        y: fe(1, prime),
    };
    assert!(curve.contains(p));

    let two_p = curve.scalar_mul(2, p);
    let three_p = curve.scalar_mul(3, p);

    match two_p {
        ECPoint::Point { x, y } => {
            assert_eq!(x.value, 6);
            assert_eq!(y.value, 3);
        }
        ECPoint::Infinity => panic!("2P should be a finite point"),
    }

    match three_p {
        ECPoint::Point { x, y } => {
            assert_eq!(x.value, 10);
            assert_eq!(y.value, 6);
        }
        ECPoint::Infinity => panic!("3P should be a finite point"),
    }
}

fn test_polynomial(prime: u64) {
    let p = Polynomial::new(vec![fe(1, prime), fe(2, prime), fe(3, prime)]);
    let q = Polynomial::new(vec![fe(3, prime), fe(1, prime)]);

    assert_eq!(p.evaluate(fe(2, prime)).value, 0);

    let product = p.clone() * q.clone();
    let (quotient, remainder) = product.div_rem(&q);

    assert_eq!(quotient, p);
    assert_eq!(remainder, Polynomial::new(vec![fe(0, prime)]));
}

fn test_r1cs(prime: u64) {
    let r1cs = R1CS {
        a: vec![vec![fe(0, prime), fe(1, prime), fe(0, prime), fe(0, prime)]],
        b: vec![vec![fe(0, prime), fe(0, prime), fe(1, prime), fe(0, prime)]],
        c: vec![vec![fe(0, prime), fe(0, prime), fe(0, prime), fe(1, prime)]],
    };

    let valid_witness = vec![fe(1, prime), fe(3, prime), fe(4, prime), fe(12, prime)];
    assert!(r1cs.verify(&valid_witness));

    let invalid_witness = vec![fe(1, prime), fe(3, prime), fe(4, prime), fe(11, prime)];
    assert!(!r1cs.verify(&invalid_witness));
}

fn main() {
    let prime = 17;

    test_field(prime);
    test_curve(prime);
    test_polynomial(prime);
    test_r1cs(prime);

    println!("All playground checks passed.");
}

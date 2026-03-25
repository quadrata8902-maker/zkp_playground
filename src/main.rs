mod field;
mod poly;
mod r1cs;
mod qap;
mod QAP;
mod curve;
mod he;

fn main() {

use crate::field::FieldElement;
use crate::r1cs::R1CS;
use crate::qap::QAP;
use crate::curve::{Curve, ECPoint};
use crate::he::{add_ciphertexts, decrypt, encrypt, keygen, Ciphertext};

// Example polynomial: x^3 + x + 5 = out

// Let's introduce variables:
// v0: 1 (constant, always 1)
// v1: x (input)
// v2: x^2
// v3: x^3
// v4: out
// So, our witness vector: [1, x, x^2, x^3, out]

let prime = 97; // a small prime for demonstration

// Example: let x = 3, then x^2=9, x^3=27, out=3^3+3+5=35
let x = FieldElement::new(3, prime);
let x2 = FieldElement::new(9, prime);
let x3 = FieldElement::new(27, prime);
let out = FieldElement::new(35, prime);
let one = FieldElement::new(1, prime);

// R1CS matrices:
// 1. Ensure x^2 = x * x
// 2. Ensure x^3 = x^2 * x
// 3. Ensure out = x^3 + x + 5

// Number of constraints (rows): 3
// Number of variables (cols): 5 (v0, v1, v2, v3, v4)
// Matrices are 3 x 5

// Gate 1: v2 = v1 * v1 <=> (0*v0 + 1*v1 + 0*v2 + 0*v3 + 0*v4) ∘ (0*v0 + 1*v1 + 0*v2 + 0*v3 + 0*v4) = (0*v0 + 0*v1 + 1*v2 + 0*v3 + 0*v4)
// Gate 2: v3 = v2 * v1 <=> (0*v0 + 0*v1 + 1*v2 + 0*v3 + 0*v4) ∘ (0*v0 + 1*v1 + 0*v2 + 0*v3 + 0*v4) = (0*v0 + 0*v1 + 0*v2 + 1*v3 + 0*v4)
// Gate 3: v4 = v3 + v1 + 5 <=> (1*v0 + 1*v1 + 0*v2 + 1*v3 + 0*v4) ∘ (1*v0) = (0*v0 + 0*v1 + 0*v2 + 0*v3 + 1*v4)

// Matrix rows (A, B, C):
let A = vec![
    vec![FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime)], // Gate 1
    vec![FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(0, prime)], // Gate 2
    // Gate 3: out = x^3 + x + 5  => A•s = 5*v0 + 1*v1 + 1*v3
    vec![FieldElement::new(5, prime), FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime)], // Gate 3
];

let B = vec![
    vec![FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime)], // Gate 1
    vec![FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime)], // Gate 2
    vec![FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime)], // Gate 3 (multiply by 1)
];

let C = vec![
    vec![FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime), FieldElement::new(0, prime)], // Gate 1
    vec![FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(1, prime), FieldElement::new(0, prime)], // Gate 2
    vec![FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(0, prime), FieldElement::new(1, prime)], // Gate 3
];

// Create the R1CS struct
let r1cs = R1CS { a: A, b: B, c: C };

// Call QAP conversion and interpolation
let qap = QAP::from_r1cs(&r1cs);

// Witness vector s aligned with R1CS column order: [v0, v1, v2, v3, v4]
let s = vec![one, x, x2, x3, out];

println!("R1CS verify: {}", r1cs.verify(&s));
println!("QAP verify:  {}", qap.verify(&s));

// Print out each interpolated polynomial for A, B, C
println!("Interpolated QAP polynomials (coefficients shown in increasing degree):");
for (idx, poly) in qap.a_polys.iter().enumerate() {
    println!("A_{}(x): {:?}", idx, poly.coeffs.iter().map(|e| e.value).collect::<Vec<_>>());
}
for (idx, poly) in qap.b_polys.iter().enumerate() {
    println!("B_{}(x): {:?}", idx, poly.coeffs.iter().map(|e| e.value).collect::<Vec<_>>());
}
for (idx, poly) in qap.c_polys.iter().enumerate() {
    println!("C_{}(x): {:?}", idx, poly.coeffs.iter().map(|e| e.value).collect::<Vec<_>>());
}

// --------------------
// Demo: additive homomorphic encryption on curve.rs (EC-ElGamal style)
// --------------------
let prime_ec: u64 = 97;
let curve = Curve {
    a: FieldElement::new(2, prime_ec),
    b: FieldElement::new(3, prime_ec),
};

// Find a valid point on the curve (brute-force small prime demo).
fn find_first_point(curve: &Curve, prime: u64) -> ECPoint {
    for x in 0..prime {
        let x_fe = FieldElement::new(x, prime);
        for y in 0..prime {
            let y_fe = FieldElement::new(y, prime);
            let p = ECPoint::Point { x: x_fe, y: y_fe };
            if curve.contains(p) {
                return p;
            }
        }
    }
    ECPoint::Infinity
}

let g = find_first_point(&curve, prime_ec);
if g == ECPoint::Infinity {
    panic!("could not find a generator point on the chosen curve");
}

// For the demo, use the same point for the plaintext encoding base.
let h = g;

let sk: u64 = 7;
let (_priv_key, pub_key) = keygen(curve, g, h, sk);

let m1: u64 = 3;
let m2: u64 = 4;
let r1: u64 = 11;
let r2: u64 = 13;

let ct1: Ciphertext = encrypt(&pub_key, m1, r1);
let ct2: Ciphertext = encrypt(&pub_key, m2, r2);

// Homomorphic addition: Enc(m1) + Enc(m2) = Enc(m1 + m2)
let ct_sum = add_ciphertexts(&curve, &ct1, &ct2);

// Decrypt by brute-force discrete log (works because m is small in this demo).
let (priv_key, _pub_key2) = keygen(curve, g, h, sk);
let dec = decrypt(&priv_key, &ct_sum, 20).expect("decryption failed in demo range");
println!("HE demo: m1+m2 decrypted as {}", dec);

}

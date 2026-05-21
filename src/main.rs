mod commit;
mod curve;
mod field;
mod he;
mod poly;
mod qap;
mod r1cs;

use crate::curve::{Curve, ECPoint};
use crate::field::FieldElement;
use crate::he::{add_ciphertexts, decrypt, encrypt, keygen, Ciphertext};
use crate::qap::QAP;
use crate::r1cs::R1CS;

fn find_first_point(curve: &Curve, prime: u64) -> ECPoint {
    // Brute-force: only suitable for small demo primes.
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

fn main() {
    // ==========================
    // R1CS -> QAP + verification demo
    // ==========================
    let prime: u64 = 97;

    // Witness variables:
    // v0: 1 (constant)
    // v1: x
    // v2: x^2
    // v3: x^3
    // v4: out = x^3 + x + 5
    let x = FieldElement::new(3, prime);
    let x2 = FieldElement::new(9, prime);
    let x3 = FieldElement::new(27, prime);
    let out = FieldElement::new(35, prime);
    let one = FieldElement::new(1, prime);

    // Gates / constraints (3x5 matrices).
    // Gate 1: v2 = v1 * v1
    // Gate 2: v3 = v2 * v1
    // Gate 3: v4 = v3 + v1 + 5
    let a = vec![
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
        ],
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
        ],
        // Gate 3: out = x^3 + x + 5  => A•s = 5*v0 + 1*v1 + 1*v3
        vec![
            FieldElement::new(5, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
        ],
    ];

    let b = vec![
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
        ],
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
        ],
        // Gate 3 multiplies by v0 = 1.
        vec![
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
        ],
    ];

    let c = vec![
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
        ],
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
            FieldElement::new(0, prime),
        ],
        vec![
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(0, prime),
            FieldElement::new(1, prime),
        ],
    ];

    let r1cs = R1CS { a, b, c };
    let qap = QAP::from_r1cs(&r1cs);

    // Witness vector s aligned with R1CS column order: [v0, v1, v2, v3, v4]
    let s = vec![one, x, x2, x3, out];

    println!("R1CS verify: {}", r1cs.verify(&s));
    println!("QAP verify:  {}", qap.verify(&s));

    // ==========================
    // Additive homomorphic encryption demo (curve.rs)
    // ==========================
    // Use a small curve y^2 = x^3 + ax + b over the same prime (demo-only).
    let prime_ec: u64 = 97;
    let curve = Curve {
        a: FieldElement::new(2, prime_ec),
        b: FieldElement::new(3, prime_ec),
    };

    let g = find_first_point(&curve, prime_ec);
    if g == ECPoint::Infinity {
        panic!("could not find a valid curve point for demo");
    }
    let h = g; // demo encoding base

    let sk: u64 = 7;
    let (priv_key, pub_key) = keygen(curve, g, h, sk);

    let m1: u64 = 3;
    let m2: u64 = 4;
    let r1: u64 = 11;
    let r2: u64 = 13;

    let ct1: Ciphertext = encrypt(&pub_key, m1, r1);
    let ct2: Ciphertext = encrypt(&pub_key, m2, r2);

    // Homomorphic addition: Enc(m1) + Enc(m2) => Enc(m1+m2)
    let ct_sum = add_ciphertexts(&curve, &ct1, &ct2);

    // Decrypt by brute-force discrete log (works because demo uses small plaintext).
    let dec = decrypt(&priv_key, &ct_sum, 20).expect("decryption failed in demo range");
    println!("HE demo: m1+m2 decrypted as {}", dec);

    // ==========================
    // Trusted setup demo
    // ==========================
    println!("\n=== zk-SNARKs Trusted Setup Demo ===");

    let tau: u64 = 8;

    // Use a curve different from the HE demo curve (still over the same field prime).
    let my_curve = Curve {
        a: FieldElement::new(0, prime),
        b: FieldElement::new(7, prime),
    };

    // For the demo, pick a valid generator point on `my_curve`.
    let mut g_point = ECPoint::Point {
        x: FieldElement::new(1, prime),
        y: FieldElement::new(5, prime),
    };
    if !my_curve.contains(g_point) {
        g_point = find_first_point(&my_curve, prime);
    }
    if g_point == ECPoint::Infinity {
        panic!("could not find a valid generator point on my_curve");
    }

    // 1) Compute A(x) from QAP and witness, then size CRS accordingly.
    let a_of_x = QAP::combine_with_witness(&qap.a_polys, &s);
    let max_degree = a_of_x.coeffs.len().max(1);

    // 2) Trusted setup: CRS = [G * tau^0, G * tau^1, ...]
    println!("1) Trusted Setup: build CRS (tau powers hidden on curve)...");
    let mut crs: Vec<ECPoint> = Vec::with_capacity(max_degree);
    let mut current_tau_power: u64 = 1;
    for _ in 0..max_degree {
        crs.push(my_curve.scalar_mul(current_tau_power, g_point));
        current_tau_power = (current_tau_power * tau) % prime;
    }

    // 3) Blind evaluation: compute [A] = Σ coeff_i * (tau^i * G) without revealing tau.
    println!("2) Blind Evaluation: compute encrypted A...");
    let blind_evaluate = |poly: &crate::poly::Polynomial, crs: &Vec<ECPoint>| -> ECPoint {
        let mut result_point = ECPoint::Infinity;
        for (i, coeff) in poly.coeffs.iter().enumerate() {
            if i >= crs.len() {
                break;
            }
            if coeff.value == 0 {
                continue;
            }
            let term_point = my_curve.scalar_mul(coeff.value, crs[i]);
            result_point = my_curve.add(result_point, term_point);
        }
        result_point
    };

    let encrypted_a = blind_evaluate(&a_of_x, &crs);
    print!("Alice encrypted [A] proof point: ");
    encrypted_a.print();
}
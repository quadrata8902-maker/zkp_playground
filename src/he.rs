use crate::curve::{Curve, ECPoint};
use crate::field::FieldElement;

/// Demo-only additive homomorphic encryption on an elliptic curve.
///
/// This is based on EC-ElGamal with plaintext encoded as a curve point:
///   encode(m) = m * H
/// The homomorphism is with respect to addition of messages:
///   Enc(m1) + Enc(m2)  =>  Enc(m1 + m2)
///
/// Decryption uses brute-force discrete log against H, so it only works for
/// small plaintext ranges.

#[derive(Clone)]
pub struct PublicKey {
    pub curve: Curve,
    pub g: ECPoint,
    pub h: ECPoint,
    pub pk: ECPoint, // pk = sk * g
}

#[derive(Clone)]
pub struct PrivateKey {
    pub curve: Curve,
    pub g: ECPoint,
    pub h: ECPoint,
    pub sk: u64,
}

#[derive(Clone)]
pub struct Ciphertext {
    pub c1: ECPoint, // r * g
    pub c2: ECPoint, // encode(m) + r * pk
}

fn negate_point(p: ECPoint) -> ECPoint {
    match p {
        ECPoint::Infinity => ECPoint::Infinity,
        ECPoint::Point { x, y } => {
            let prime = y.prime;
            ECPoint::Point {
                x,
                y: FieldElement::new(prime - y.value, prime),
            }
        }
    }
}

pub fn keygen(curve: Curve, g: ECPoint, h: ECPoint, sk: u64) -> (PrivateKey, PublicKey) {
    if sk == 0 {
        panic!("sk must be non-zero");
    }
    if !curve.contains(g) || !curve.contains(h) {
        panic!("g/h must be valid points on the curve");
    }

    let pk = curve.scalar_mul(sk, g);
    let priv_key = PrivateKey {
        curve,
        g,
        h,
        sk,
    };
    let pub_key = PublicKey {
        curve,
        g,
        h,
        pk,
    };
    (priv_key, pub_key)
}

pub fn encrypt(pub_key: &PublicKey, m: u64, r: u64) -> Ciphertext {
    if r == 0 {
        panic!("r must be non-zero for this demo");
    }

    // encode(m) = m * H
    let m_point = pub_key.curve.scalar_mul(m, pub_key.h);
    let c1 = pub_key.curve.scalar_mul(r, pub_key.g);
    let r_pk = pub_key.curve.scalar_mul(r, pub_key.pk);
    let c2 = pub_key.curve.add(m_point, r_pk);

    Ciphertext { c1, c2 }
}

pub fn decrypt(
    priv_key: &PrivateKey,
    ct: &Ciphertext,
    max_m: u64,
) -> Option<u64> {
    // encode(m) = c2 - sk*c1 = c2 + (-(sk*c1))
    let sk_c1 = priv_key.curve.scalar_mul(priv_key.sk, ct.c1);
    let neg_sk_c1 = negate_point(sk_c1);
    let encoded = priv_key.curve.add(ct.c2, neg_sk_c1);

    // Brute-force discrete log: find m in [0, max_m] such that m*H == encoded.
    for m in 0..=max_m {
        if priv_key.curve.scalar_mul(m, priv_key.h) == encoded {
            return Some(m);
        }
    }
    None
}

pub fn add_ciphertexts(curve: &Curve, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext {
    // Homomorphism: (r1G, m1H+r1PK) + (r2G, m2H+r2PK)
    // => ((r1+r2)G, (m1+m2)H + (r1+r2)PK)
    let c1 = curve.add(ct1.c1, ct2.c1);
    let c2 = curve.add(ct1.c2, ct2.c2);
    Ciphertext { c1, c2 }
}

pub fn mul_ciphertext_scalar(curve: &Curve, ct: &Ciphertext, k: u64) -> Ciphertext {
    // Scalar multiplication: k * Enc(m) => Enc(km) (for the same encoding).
    Ciphertext {
        c1: curve.scalar_mul(k, ct.c1),
        c2: curve.scalar_mul(k, ct.c2),
    }
}


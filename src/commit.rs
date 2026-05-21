use crate::field::FieldElement;
use crate::curve::{Curve, ECPoint};
use crate::poly::Polynomial;

/// A toy "trusted setup" Common Reference String: the points [G, tau*G, tau^2*G, ..., tau^d*G].
/// In a real SNARK, tau ("toxic waste") is never revealed to anyone and is destroyed right after
/// setup. Here `setup` takes tau only so we can demonstrate the construction; a real ceremony
/// would generate it inside a secure black box and discard it immediately.
pub struct CRS {
    pub curve: Curve,
    pub g: ECPoint,
    pub powers: Vec<ECPoint>, // powers[i] = (tau^i) * G
}

impl CRS {
    pub fn setup(curve: Curve, g: ECPoint, tau: FieldElement, max_degree: usize) -> CRS {
        let mut powers = Vec::with_capacity(max_degree + 1);
        let mut acc = g; // tau^0 * G = G
        powers.push(acc);
        for _ in 1..=max_degree {
            acc = curve.scalar_mul(tau.value, acc); // tau^k * G = tau * (tau^{k-1} * G)
            powers.push(acc);
        }
        CRS { curve, g, powers }
    }

    /// Homomorphic hiding of P(tau): commit(P) = sum_i a_i * (tau^i * G) = P(tau) * G,
    /// computed WITHOUT knowing tau -- only from the public CRS points.
    pub fn commit(&self, poly: &Polynomial) -> ECPoint {
        if poly.coeffs.len() > self.powers.len() {
            panic!("polynomial degree exceeds CRS size");
        }
        let mut result = ECPoint::Infinity;
        for (i, coeff) in poly.coeffs.iter().enumerate() {
            if coeff.value == 0 { continue; } // 0 * anything = identity, skip
            let term = self.curve.scalar_mul(coeff.value, self.powers[i]);
            result = self.curve.add(result, term);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::field::FieldElement;
    use crate::curve::{Curve, ECPoint};
    use crate::poly::Polynomial;
    use crate::commit::CRS;

    // Curve y^2 = x^3 + 2x + 2 over F_17, generator G=(5,1) has prime order 19.
    // The curve lives over the base field F_17, but scalars (polynomial coefficients)
    // live in F_19 = order of G, because k*G only depends on k mod 19.
    const BP: u64 = 17; // curve base field
    const SP: u64 = 19; // scalar field = group order
    fn bf(v: u64) -> FieldElement { FieldElement::new(v, BP) }
    fn sf(v: u64) -> FieldElement { FieldElement::new(v, SP) }
    fn curve() -> Curve { Curve { a: bf(2), b: bf(2) } }
    fn g() -> ECPoint { ECPoint::Point { x: bf(5), y: bf(1) } }
    fn spoly(cs: &[u64]) -> Polynomial { Polynomial::new(cs.iter().map(|&v| sf(v)).collect()) }

    #[test]
    fn commit_equals_evaluation_in_the_exponent() {
        // The defining property of homomorphic hiding:
        //   commit(P) (built only from the CRS) == P(tau) * G.
        let c = curve();
        let tau = sf(7);
        let crs = CRS::setup(c, g(), tau, 5);
        let p = spoly(&[3, 2, 5]); // 3 + 2x + 5x^2
        let committed = crs.commit(&p);
        let direct = c.scalar_mul(p.evaluate(tau).value, g());
        assert!(committed == direct);
    }

    #[test]
    fn additive_homomorphism_of_commitments() {
        // commit(P + Q) == commit(P) (+) commit(Q) -- the homomorphism E(x)+E(y)=E(x+y)
        let c = curve();
        let crs = CRS::setup(c, g(), sf(11), 5);
        let p = spoly(&[3, 2, 5]);
        let q = spoly(&[1, 4]);
        let lhs = crs.commit(&(p.clone() + q.clone()));
        let rhs = c.add(crs.commit(&p), crs.commit(&q));
        assert!(lhs == rhs);
    }

    #[test]
    fn scalar_hiding_is_homomorphic() {
        // The underlying map E(x) = x*G satisfies E(x) + E(y) = E(x+y), even with wraparound.
        let c = curve();
        let x = 13u64; let y = 9u64; // x + y = 22 == 3 (mod 19)
        let lhs = c.add(c.scalar_mul(x, g()), c.scalar_mul(y, g()));
        let rhs = c.scalar_mul((x + y) % SP, g());
        assert!(lhs == rhs);
    }

    #[test]
    fn zero_polynomial_commits_to_identity() {
        let c = curve();
        let crs = CRS::setup(c, g(), sf(7), 5);
        assert!(crs.commit(&spoly(&[0, 0, 0])) == ECPoint::Infinity);
    }

    #[test]
    fn commitment_is_deterministic() {
        // Same polynomial + same CRS => identical commitment point.
        let c = curve();
        let crs = CRS::setup(c, g(), sf(7), 5);
        let p = spoly(&[4, 0, 6, 1]);
        assert!(crs.commit(&p) == crs.commit(&p));
    }

    #[test]
    fn crs_powers_are_consecutive_powers_of_tau() {
        // powers[i] must equal (tau^i) * G.
        let c = curve();
        let tau = sf(7);
        let crs = CRS::setup(c, g(), tau, 4);
        let mut tau_pow = 1u64;
        for i in 0..=4 {
            assert!(crs.powers[i] == c.scalar_mul(tau_pow % SP, g()), "power {} mismatch", i);
            tau_pow = (tau_pow * tau.value) % SP;
        }
    }

    #[test]
    #[should_panic]
    fn committing_beyond_crs_size_panics() {
        // CRS only supports up to degree 2; committing a degree-3 polynomial must panic.
        let c = curve();
        let crs = CRS::setup(c, g(), sf(7), 2);
        let _ = crs.commit(&spoly(&[1, 1, 1, 1]));
    }
}
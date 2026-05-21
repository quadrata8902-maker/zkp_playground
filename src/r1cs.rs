use crate::field::FieldElement;

//the struct of R1CS, which is consisted of 3 matrices (vec in vec)
#[derive(Clone, Debug)]
pub struct R1CS {
    pub a: Vec<Vec<FieldElement>>,
    pub b: Vec<Vec<FieldElement>>,
    pub c: Vec<Vec<FieldElement>>,
}

impl R1CS {
    //dot product
    fn dot_product(row: &Vec<FieldElement>, s: &Vec<FieldElement>) -> FieldElement {
        if row.len() != s.len() {
            panic!("!length of row is different from witness vector");
        }
        
        let prime = row[0].prime;
        let mut result = FieldElement::new(0, prime);
        
        for i in 0..row.len() {
            result = result + (row[i] * s[i]);
        }
        result
    }

    //the main function, to verify the correctness of AxB=C
    pub fn verify(&self, s: &Vec<FieldElement>) -> bool {
        let num_equations = self.a.len();

        //check every row wether that A•s*B•s=C•s
        for i in 0..num_equations {
            let a_val = Self::dot_product(&self.a[i], s);
            let b_val = Self::dot_product(&self.b[i], s);
            let c_val = Self::dot_product(&self.c[i], s);

            if a_val * b_val != c_val {
                println!("!test failed at step {} ", i + 1);
                println!("!LHS: {} * {}, RHS: {}", a_val.value, b_val.value, c_val.value);
                return false;
            }
        }
        
        println!("Verification Passed!");
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // f(x) = x^3 + x + 5. Use prime 149 (> the largest value 135 that appears),
    // so the integer relations are not affected by modular reduction.
    const P: u64 = 149;
    fn fe(v: u64) -> FieldElement { FieldElement::new(v, P) }
    fn rows(data: &[[u64; 6]]) -> Vec<Vec<FieldElement>> {
        data.iter().map(|r| r.iter().map(|&v| fe(v)).collect()).collect()
    }
    fn s_from(vals: [u64; 6]) -> Vec<FieldElement> { vals.iter().map(|&v| fe(v)).collect() }

    // A and B are as in the article; C row 3 is the correct [0,0,0,0,0,1].
    fn r1cs() -> R1CS {
        R1CS {
            a: rows(&[[0,1,0,0,0,0], [0,0,0,1,0,0], [0,1,0,0,1,0], [5,0,0,0,0,1]]),
            b: rows(&[[0,1,0,0,0,0], [0,1,0,0,0,0], [1,0,0,0,0,0], [1,0,0,0,0,0]]),
            c: rows(&[[0,0,0,1,0,0], [0,0,0,0,1,0], [0,0,0,0,0,1], [0,0,1,0,0,0]]),
        }
    }

    #[test]
    fn correct_witness_passes() {
        // S(3) = [1, x, out, v1, v2, v3] = [1, 3, 35, 9, 27, 30]
        assert!(r1cs().verify(&s_from([1, 3, 35, 9, 27, 30])));
    }

    #[test]
    fn tampered_output_fails() {
        // out changed from 35 to 36; constraint r4 (v3+5)*1 = out must fail
        assert!(!r1cs().verify(&s_from([1, 3, 36, 9, 27, 30])));
    }

    #[test]
    fn tampered_intermediate_fails() {
        // v1 changed from 9 to 8 (x*x no longer holds); constraint r1 fails
        assert!(!r1cs().verify(&s_from([1, 3, 35, 8, 27, 30])));
    }

    #[test]
    fn fake_witness_without_secret_fails() {
        // Eve has no secret and fills in arbitrary numbers; constraints unsatisfied
        assert!(!r1cs().verify(&s_from([1, 7, 35, 2, 11, 4])));
    }

    #[test]
    fn r1cs_checks_consistency_not_the_value_35() {
        // Key point: R1CS only checks that the circuit was executed correctly,
        // NOT that out == 35. Claiming x = 5 gives a fully consistent trace
        // S(5) = [1, 5, 135, 25, 125, 130] with out = f(5) = 135, so verify()
        // returns true. Pinning the statement to "f(x) = 35" requires fixing
        // out as a public input separately.
        assert!(r1cs().verify(&s_from([1, 5, 135, 25, 125, 130])));
    }

    #[test]
    #[should_panic]
    fn witness_wrong_length_panics() {
        // dot_product panics when the witness length does not match a row
        let _ = r1cs().verify(&s_from([1, 3, 35, 9, 27, 30])[..4].to_vec());
    }
}
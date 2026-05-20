//in this section, we will construct the structure of Polynomials
//we will store Polynomials in form of vectors, and construct the operations of +-*/
use crate::field::FieldElement;
use std::ops::{Add, Sub, Mul};

//the structure of polynomals itself
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    pub coeffs: Vec<FieldElement>,
}

impl Polynomial {
    //new polynomails
    pub fn new(mut coeffs: Vec<FieldElement>) -> Self {
        //construct the polynomial and then trim those coeff=0
        let mut poly = Polynomial { coeffs };
        poly.trim();
        poly
    }

    //take the highest degree of all terms as the "degree" of the polynomial
    pub fn degree(&self) -> usize {
        if self.coeffs.is_empty() {
            return 0;
        }
        self.coeffs.len() - 1
    }

    // to check wether the polynomial is 0
    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|c| c.value == 0)
    }

    //to trim the vector if the last coefficient is 0
    pub fn trim(&mut self) {
        while self.coeffs.len() > 1 && self.coeffs.last().unwrap().value == 0 {
            self.coeffs.pop();
        }
    }

    //the funtion here returns P(x) for a given x
    pub fn evaluate(&self, x: FieldElement) -> FieldElement {
        if self.coeffs.is_empty() {
            panic!("!empty polynomial");
        }
        
        //initialize the sum as 0, then add each term on it
        let mut result = FieldElement::new(0, x.prime);
        let mut x_pow = FieldElement::new(1, x.prime);

        for c in &self.coeffs {
            result = result + (*c * x_pow);
            x_pow = x_pow * x;
        }
        
        result
    }

    //polynomial division, which gives quotients and reminders (for polynomial is an Euclidean Domain)
    pub fn div_rem(&self, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        //make sure that the divisor cannot be 0
        if divisor.coeffs.is_empty() || (divisor.coeffs.len() == 1 && divisor.coeffs[0].value == 0) {
            panic!("!Cannot divide by zero polynomial");
        }

        let prime = self.coeffs[0].prime;
        let mut quotient_coeffs = vec![FieldElement::new(0, prime); self.coeffs.len()];
        //initialize the remainder as dividend itself
        let mut remainder = self.clone();

        //the loop is basiclly long division
        while !remainder.is_zero() && remainder.degree() >= divisor.degree() {
            let deg_diff = remainder.degree() - divisor.degree();
            let lead_rem = remainder.coeffs.last().unwrap();
            let lead_div = divisor.coeffs.last().unwrap();
            
            let lead_quotient = *lead_rem / *lead_div; 

            quotient_coeffs[deg_diff] = lead_quotient;

            let mut term_coeffs = vec![FieldElement::new(0, prime); deg_diff + 1];
            term_coeffs[deg_diff] = lead_quotient;
            let term_poly = Polynomial::new(term_coeffs);
            
            let subtract_poly = term_poly * divisor.clone();
            remainder = remainder - subtract_poly;
        }

        let mut quotient = Polynomial::new(quotient_coeffs);
        quotient.trim();
        remainder.trim();

        (quotient, remainder)
    }

    pub fn print(&self) {
        if self.coeffs.is_empty() {
            println!("0");
            return;
        }
        
        let mut terms = Vec::new();
        for (i, c) in self.coeffs.iter().enumerate().rev() {
            if c.value != 0 {
                if i == 0 {
                    terms.push(format!("{}", c.value));
                } else if i == 1 {
                    terms.push(format!("{}x", c.value));
                } else {
                    terms.push(format!("{}x^{}", c.value, i));
                }
            }
        }
        
        if terms.is_empty() {
            println!("0 (mod {})", self.coeffs[0].prime);
        } else {
            let poly_str = terms.join(" + ");
            println!("{} (mod {})", poly_str, self.coeffs[0].prime);
        }
    }
}

//traits
impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        //compair two polynomials and let max deg to be the one with higher degree
        let max_len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(max_len);
        
        let prime = self.coeffs[0].prime;

        //term by term addition, similar in subtraction
        for i in 0..max_len {            
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { FieldElement::new(0, prime) };
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { FieldElement::new(0, prime) };
            
            result_coeffs.push(a + b);
        }
        Polynomial::new(result_coeffs)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        let max_len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(max_len);
        let prime = self.coeffs[0].prime;

        for i in 0..max_len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { FieldElement::new(0, prime) };
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { FieldElement::new(0, prime) };
            
            result_coeffs.push(a - b);
        }

        Polynomial::new(result_coeffs)
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let prime = self.coeffs[0].prime;
        
        let result_len = self.coeffs.len() + other.coeffs.len() - 1;
        
        let mut result_coeffs = vec![FieldElement::new(0, prime); result_len];

        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                let term = self.coeffs[i] * other.coeffs[j];
                result_coeffs[i + j] = result_coeffs[i + j] + term;
            }
        }

        Polynomial::new(result_coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 7;
    fn fe(v: u64) -> FieldElement { FieldElement::new(v, P) }
    // 由係數 (低次到高次) 建立多項式，例如 [1, 2] = 2x + 1
    fn poly(cs: &[u64]) -> Polynomial { Polynomial::new(cs.iter().map(|&v| fe(v)).collect()) }

    #[test]
    fn new_trims_trailing_zeros() {
        // 2x + 1 後面多餘的高次零項應被修剪
        let p = poly(&[1, 2, 0, 0]);
        assert_eq!(p.coeffs, vec![fe(1), fe(2)]);
        assert_eq!(p.degree(), 1);
    }

    #[test]
    fn zero_polynomial_is_zero() {
        assert!(poly(&[0]).is_zero());
        assert!(poly(&[0, 0, 0]).is_zero());
        assert!(!poly(&[0, 1]).is_zero());
    }

    #[test]
    fn evaluate_basic() {
        // P(x) = 2x^2 + 3x + 1 ; P(2) = 15 = 1 (mod 7)
        assert_eq!(poly(&[1, 3, 2]).evaluate(fe(2)), fe(1));
    }

    #[test]
    fn evaluate_at_roots() {
        // (x-1)(x-2) = x^2 - 3x + 2 ; 根 x=1,2 求值為 0, x=3 為 2
        let p = poly(&[2, 4, 1]); // 常數 2, 一次 -3=4, 二次 1
        assert_eq!(p.evaluate(fe(1)), fe(0));
        assert_eq!(p.evaluate(fe(2)), fe(0));
        assert_eq!(p.evaluate(fe(3)), fe(2));
    }

    #[test]
    fn addition() {
        // (x^2 + 1) + (x + 2) = x^2 + x + 3
        assert_eq!(poly(&[1, 0, 1]) + poly(&[2, 1]), poly(&[3, 1, 1]));
    }

    #[test]
    fn addition_cancels_and_trims() {
        // (x + 1) + (-x + 1) = 2，高次相消後應降次
        let sum = poly(&[1, 1]) + poly(&[1, 6]); // 6 = -1
        assert_eq!(sum, poly(&[2]));
        assert_eq!(sum.degree(), 0);
    }

    #[test]
    fn subtraction() {
        // (x^2 + 3x + 2) - (x^2 + 1) = 3x + 1
        assert_eq!(poly(&[2, 3, 1]) - poly(&[1, 0, 1]), poly(&[1, 3]));
    }

    #[test]
    fn subtraction_to_zero() {
        let p = poly(&[2, 3, 1]);
        assert!((p.clone() - p).is_zero());
    }

    #[test]
    fn multiplication() {
        // (x + 1)(x + 2) = x^2 + 3x + 2
        assert_eq!(poly(&[1, 1]) * poly(&[2, 1]), poly(&[2, 3, 1]));
    }

    #[test]
    fn multiplication_by_zero() {
        assert!((poly(&[1, 1]) * poly(&[0])).is_zero());
    }

    #[test]
    fn div_rem_exact() {
        // (x^2 - 1) / (x + 1) = (x - 1)，餘 0
        let (q, r) = poly(&[6, 0, 1]).div_rem(&poly(&[1, 1]));
        assert_eq!(q, poly(&[6, 1])); // x - 1 = x + 6
        assert!(r.is_zero());
    }

    #[test]
    fn div_rem_with_remainder() {
        // (x^2 + 1) / (x + 1) = (x - 1) 餘 2
        let (q, r) = poly(&[1, 0, 1]).div_rem(&poly(&[1, 1]));
        assert_eq!(q, poly(&[6, 1]));
        assert_eq!(r, poly(&[2]));
    }

    #[test]
    fn div_rem_by_constant_terminates() {
        // 這正是原本會無窮迴圈的情況: (5x + 6) / 3 = (4x + 2) 餘 0
        let (q, r) = poly(&[6, 5]).div_rem(&poly(&[3]));
        assert_eq!(q, poly(&[2, 4]));
        assert!(r.is_zero());
    }

    #[test]
    fn div_rem_satisfies_euclidean_identity() {
        // 對多組輸入驗證歐幾里得性質: a = q*d + r 且 (r=0 或 deg r < deg d)
        let cases = [
            (vec![5u64, 1, 3, 2], vec![1u64, 1]),
            (vec![6, 0, 0, 1],    vec![2, 0, 1]),
            (vec![4, 5, 6],       vec![3]),       // 常數除式
            (vec![1, 2, 3, 4, 5], vec![1, 0, 2]),
        ];
        for (a_c, d_c) in cases {
            let a = poly(&a_c);
            let d = poly(&d_c);
            let (q, r) = a.div_rem(&d);
            assert_eq!(q.clone() * d.clone() + r.clone(), a, "q*d + r 應還原 a");
            assert!(r.is_zero() || r.degree() < d.degree(), "餘式次數應 < 除式次數");
        }
    }

    #[test]
    #[should_panic]
    fn div_by_zero_panics() {
        let _ = poly(&[1, 1]).div_rem(&poly(&[0]));
    }
}
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
        while remainder.degree() >= divisor.degree() && !remainder.coeffs.is_empty() {
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
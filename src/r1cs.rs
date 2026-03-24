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
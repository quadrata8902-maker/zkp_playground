//in this section, we will construct the structure of QAP
use crate::field::FieldElement;
use crate::poly::Polynomial;
use crate::r1cs::R1CS;

//implement the lagrange interpolation
pub fn lagrange_interpolate(x_coords: &[FieldElement], y_coords: &[FieldElement]) -> Polynomial {
    if x_coords.len() != y_coords.len() || x_coords.is_empty() {
        panic!("!number of x and y coordinates are different or empty");
    }
    
    let prime = x_coords[0].prime;

    let mut result = Polynomial::new(vec![FieldElement::new(0, prime)]);

    // outer loop, to visit each given point (x_i, y_i)
    for i in 0..x_coords.len() {
        // if the y coordinate of this point is 0, then it multiply any thing is 0, so skip it
        // this is very common in R1CS matrix, because there are a lot of 0s in the matrix
        if y_coords[i].value == 0 {
            continue;
        }

        // construct the basis polynomial L_i(x) for this point, initial value is 1
        let mut l_i = Polynomial::new(vec![FieldElement::new(1, prime)]);
        
        // prepare a variable to store the denominator
        let mut denominator = FieldElement::new(1, prime);

        // inner loop, to multiply all other (x - x_j)
        for j in 0..x_coords.len() {
            if i == j { 
                continue; // cannot subtract itself, otherwise the denominator will be 0
            }

            // 1. handle the numerator: multiply the polynomial (x - x_j)
            // in the array, (x - x_j) is the polynomial with constant term -x_j and linear term 1: [-x_j, 1]
            let term_poly = Polynomial::new(vec![
                FieldElement::new(0, prime) - x_coords[j], // constant term: 0 - x_j
                FieldElement::new(1, prime),               // linear term: 1x
            ]);
            l_i = l_i * term_poly; // trigger the polynomial multiplication magic in poly.rs!

            // 2. handle the denominator: multiply the constant (x_i - x_j)
            denominator = denominator * (x_coords[i] - x_coords[j]);
        }

        // divide y_i by the denominator, to get the coefficient of the basis polynomial
        let coef = y_coords[i] / denominator;

        // multiply the basis polynomial l_i(x) by this coefficient (equal to multiply a polynomial with only constant term)
        let coef_poly = Polynomial::new(vec![coef]);
        let term = l_i * coef_poly;

        // add to the final result polynomial
        result = result + term;
    }

    result.trim(); // ensure the highest term has no invalid 0
    result
}
#[derive(Clone, Debug)] 
pub struct QAP {
    pub a_polys: Vec<Polynomial>,
    pub b_polys: Vec<Polynomial>,
    pub c_polys: Vec<Polynomial>,
    /// Evaluation points `t_i` for each gate (used to build `Z(x)`).
    pub t_coords: Vec<FieldElement>,
}

impl QAP {
    // convert the R1CS matrix to a collection of QAP polynomials
    pub fn from_r1cs(r1cs: &R1CS) -> Self {
        let num_equations = r1cs.a.len(); // number of gates (Rows)
        let num_vars = r1cs.a[0].len();   // number of variables (Columns)
        let prime = r1cs.a[0][0].prime;

        // 1. prepare x coordinates: the gate number 1, 2, 3...
        // mathematically, we agree that the x coordinate of Gate 1 is 1, Gate 2 is 2... and so on
        let mut x_coords = Vec::with_capacity(num_equations);
        for i in 1..=num_equations {
            x_coords.push(FieldElement::new(i as u64, prime));
        }

        let mut a_polys = Vec::with_capacity(num_vars);
        let mut b_polys = Vec::with_capacity(num_vars);
        let mut c_polys = Vec::with_capacity(num_vars);

        // 2. for each variable (Column), collect its values in all gates (y coordinates), then perform interpolation!
        for var_idx in 0..num_vars {
            let mut a_y_coords = Vec::with_capacity(num_equations);
            let mut b_y_coords = Vec::with_capacity(num_equations);
            let mut c_y_coords = Vec::with_capacity(num_equations);

            // for each gate (Row), collect its values
            for gate_idx in 0..num_equations {
                a_y_coords.push(r1cs.a[gate_idx][var_idx]);
                b_y_coords.push(r1cs.b[gate_idx][var_idx]);
                c_y_coords.push(r1cs.c[gate_idx][var_idx]);
            }

            // call the interpolation function to generate the corresponding polynomials
            a_polys.push(lagrange_interpolate(&x_coords, &a_y_coords));
            b_polys.push(lagrange_interpolate(&x_coords, &b_y_coords));
            c_polys.push(lagrange_interpolate(&x_coords, &c_y_coords));
        }

        QAP {
            a_polys,
            b_polys,
            c_polys,
            t_coords: x_coords,
        }
    }

    /// Compute A(x) = sum_j s_j * a_j(x) (and similarly for B(x), C(x)).
    fn combine_with_witness(polys: &[Polynomial], s: &[FieldElement]) -> Polynomial {
        if polys.len() != s.len() {
            panic!("witness length does not match number of QAP polynomials");
        }
        let prime = polys[0].coeffs[0].prime;

        let mut result = Polynomial::new(vec![FieldElement::new(0, prime)]);
        for (j, poly_j) in polys.iter().enumerate() {
            // Skip zero coefficients; common because many R1CS entries are 0.
            if s[j].value == 0 {
                continue;
            }
            let coef_poly = Polynomial::new(vec![s[j]]);
            result = result + (poly_j.clone() * coef_poly);
        }
        result
    }

    fn is_zero_poly(p: &Polynomial) -> bool {
        p.coeffs.iter().all(|c| c.value == 0)
    }

    /// Build Z(x) = Π_i (x - t_i) from the stored evaluation points.
    fn build_z_poly(&self) -> Polynomial {
        let prime = self.t_coords[0].prime;
        let mut z = Polynomial::new(vec![FieldElement::new(1, prime)]);

        for t in &self.t_coords {
            // (x - t) = [-t, 1]
            let term = Polynomial::new(vec![
                FieldElement::new(0, prime) - *t,
                FieldElement::new(1, prime),
            ]);
            z = z * term;
        }
        z
    }

    /// Verify the QAP condition:
    ///   Z(x) | (A(x) * B(x) - C(x))
    pub fn verify(&self, s: &[FieldElement]) -> bool {
        let a_of_x = Self::combine_with_witness(&self.a_polys, s);
        let b_of_x = Self::combine_with_witness(&self.b_polys, s);
        let c_of_x = Self::combine_with_witness(&self.c_polys, s);

        let ab = a_of_x.clone() * b_of_x.clone();
        let w = ab - c_of_x;
        let z = self.build_z_poly();

        let (_q, r) = w.div_rem(&z);
        Self::is_zero_poly(&r)
    }
}
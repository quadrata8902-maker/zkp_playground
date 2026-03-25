//in this section, we will construct the structure of QAP
use crate::field::FieldElement;
use crate::poly::Polynomial;

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
                continue; // cannot subtract itself, otherwise the denominator will be 0!
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
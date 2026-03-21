mod field;

use field::FieldElement;

fn main() {
    let p = 17;
    let a = FieldElement::new(10, p);
    let b = FieldElement::new(15, p);

    print!("Element A: ");
    a.print();
    print!("Element B: ");
    b.print();

    let e = a.sub(&b);
    print!("A - B = ");
    e.print();

    let f = a.div(&b);
    print!("A / B = ");
    f.print();

    let verify = f.mul(&b);
    print!("(A / B) * B = ");
    verify.print();
}
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

    let c = a + b;
    print!("A + B = ");
    c.print();

    let d = a - b;
    print!("A - B = ");
    d.print();

    let e = a * b;
    print!("A * B = ");
    e.print();

    let f = a / b;
    print!("A / B = ");
    f.print();

    let verify = (a / b) * b;
    print!("(A / B) * B = ");
    verify.print();
}
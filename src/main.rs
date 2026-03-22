mod field;
mod curve;

use field::FieldElement;
use curve::{Curve, ECPoint};

fn main() {
    let p = 17;

    //setup a toy EC: y^2 = x^3 + 7 (mod 17)
    let a = FieldElement::new(0, p);
    let b = FieldElement::new(7, p);
    let my_curve = Curve { a, b };

    println!("=== EC test: y^2 = x^3 + 7 (mod 17) ===");

    //test point:(1,5) should lies on the EC
    let x1 = FieldElement::new(1, p);
    let y1 = FieldElement::new(5, p);
    let p1 = ECPoint::Point { x: x1, y: y1 };

    print!("test point P1 ");
    p1.print();
    println!("Is P1 ont the EC? {}", my_curve.contains(p1));
    println!("---------------------------------");

    //test the doubling:P1+P1
    let p2 = my_curve.add(p1, p1);
    print!("P1 + P1 = P2 ");
    p2.print();
    println!("Is P2 on the EC? {}", my_curve.contains(p2));
    println!("---------------------------------");

    //test the regular addition
    let p3 = my_curve.add(p1, p2);
    print!("P1 + P2 = P3 ");
    p3.print();
    println!("Is P3 on the EC? {}", my_curve.contains(p3));
}
struct FieldElement {
    value: u64,
    prime: u64,
}

impl FieldElement{
    fn new (value: u64, prime: u64) -> FieldElement {
        FieldElement {
            value: value % prime,
            prime: prime,
        }
    }

    fn print(&self){
        println!("{} (mod{})", self.value, self.prime);
    }

    fn add(&self, other: &FieldElement) -> FieldElement{
    if self.prime != other.prime {
        panic!("Different Field Elements");
    }
    let new_value=(self.value + other.value) % self.prime;
    FieldElement::new(new_value, self.prime)
    }
    
    fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Different Field Elements");
        }
        let new_value = (self.value * other.value) % self.prime;
        FieldElement::new(new_value, self.prime)
    }
}

fn main () {
    let p = 17;
    let a = FieldElement::new(10,p);
    let b = FieldElement::new(15,p);

    print!("Element A:");
    a.print();
    print!("Element B:");
    b.print();

    let c = a.add(&b);
    print!("A+B=");
    c.print();

    let d = a.mul(&b);
    print!("A*B=");
    d.print();
}
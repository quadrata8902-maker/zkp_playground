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
        panic!("!different field elements");
    }
    let new_value=(self.value + other.value) % self.prime;
    FieldElement::new(new_value, self.prime)
    }
    
    fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("!different field elements");
        }
        let new_value = (self.value * other.value) % self.prime;
        FieldElement::new(new_value, self.prime)
    }

    fn sub(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("!different field elements");
        }
        let new_value = (self.value + self.prime - other.value) % self.prime;
        FieldElement::new(new_value, self.prime)
    }

    fn pow(&self, exponent: u64) -> FieldElement {
        let mut result = 1;
        let mut base = self.value;
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % self.prime;
            }
            base = (base * base) % self.prime;
            exp /= 2;
        }
        FieldElement::new(result, self.prime)
    }
    fn inv(&self) -> FieldElement {
        if self.value == 0 {
            panic!("!no iverse for zero");
        }
        self.pow(self.prime - 2)
    }
    fn div(&self, other: &FieldElement) -> FieldElement {
        let other_inv = other.inv();
        self.mul(&other_inv)
    }
}

fn main () {
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
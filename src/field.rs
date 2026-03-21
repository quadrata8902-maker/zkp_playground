#[derive(Clone, Copy)]
pub struct FieldElement {
    value: u64,
    prime: u64,
}

impl FieldElement{
    pub fn new (value: u64, prime: u64) -> FieldElement {
        FieldElement {
            value: value % prime,
            prime: prime,
        }
    }

    pub fn print(&self){
        println!("{} (mod{})", self.value, self.prime);
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement{
    if self.prime != other.prime {
        panic!("!different field elements");
    }
    let new_value=(self.value + other.value) % self.prime;
    FieldElement::new(new_value, self.prime)
    }
    
    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("!different field elements");
        }
        let new_value = (self.value * other.value) % self.prime;
        FieldElement::new(new_value, self.prime)
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("!different field elements");
        }
        let new_value = (self.value + self.prime - other.value) % self.prime;
        FieldElement::new(new_value, self.prime)
    }

    pub fn pow(&self, exponent: u64) -> FieldElement {
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
    pub fn inv(&self) -> FieldElement {
        if self.value == 0 {
            panic!("!no iverse for zero");
        }
        self.pow(self.prime - 2)
    }
    pub fn div(&self, other: &FieldElement) -> FieldElement {
        let other_inv = other.inv();
        self.mul(&other_inv)
    }
}

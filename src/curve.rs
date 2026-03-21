use crate::field::FieldElement;

#[derive(Clone, Copy, PartialEq)]

pub enum ECPoint {
    Point { x: FieldElement, y: FieldElement },
    Infinity,
}

#[derive(Clone, Copy)]
pub struct Curve {
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Curve {
    pub fn contains(&self, point: ECPoint) -> bool {
        match point {
            ECPoint::Infinity => true,             
            ECPoint::Point { x, y } => {
                let left = y * y;
                let right = (x * x * x) + (self.a * x) + self.b;               
                left == right
            }
        }
    }
}
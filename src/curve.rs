use crate::field::FieldElement;

#[derive(Clone, Copy, PartialEq)]

//ECPoints are either a regular point of (x,y), x,y \in FiniteFiled or a Infinite point
pub enum ECPoint {
    Point { x: FieldElement, y: FieldElement },
    Infinity,
}

//print the point
impl ECPoint {
    pub fn print(&self) {
        match self {
            ECPoint::Infinity => println!("(Infinity)"),
            ECPoint::Point { x, y } => println!("({}, {})", x.value, y.value),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Curve {
    pub a: FieldElement,
    pub b: FieldElement,
}

//To check wether a point is on a certain EC or not.
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
    //the following is the addition of ECPoints
    pub fn add(&self, p1: ECPoint, p2: ECPoint) -> ECPoint {
        match (p1, p2) {
            //if one of the addend is Infinity then return the other
            (ECPoint::Infinity, p) => p,
            (p, ECPoint::Infinity) => p,
            //the following is when both points are finite points
            (ECPoint::Point { x: x1, y: y1 }, ECPoint::Point { x: x2, y: y2 }) => {
                //a special case, utilizes the case of inverse adition    
                if x1 == x2 && y1 != y2 {
                    return ECPoint::Infinity;
                }
            
                //another special case, the case of "doubling"
                if x1 == x2 && y1 == y2 {
                    if y1.value == 0 {
                        return ECPoint::Infinity;
                    }
                    //the point here is that we need to calculate the slope of a single point
                    //use the technique of implicit differentiation
                    let two = FieldElement::new(2, x1.prime);
                    let three = FieldElement::new(3, x1.prime);
                    
                    let s = ((three * x1 * x1) + self.a) / (two * y1);

                    let x3 = (s * s) - x1 - x2;
                    let y3 = s * (x1 - x3) - y1;

                    return ECPoint::Point { x: x3, y: y3 };
                }
                //rest of the cases, the usual ones
                let s = (y2 - y1) / (x2 - x1);
                
                let x3 = (s * s) - x1 - x2;
                let y3 = s * (x1 - x3) - y1;

                ECPoint::Point { x: x3, y: y3 }
            }
        }
    }
}
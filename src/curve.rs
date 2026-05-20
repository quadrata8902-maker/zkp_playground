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
    //scalar multiplication, to calculate kP faster to O(logk)
    pub fn scalar_mul(&self, k: u64, point: ECPoint) -> ECPoint {
        let mut result = ECPoint::Infinity; 
        let mut addend = point;             
        let mut multiplier = k;            

        while multiplier > 0 {
            if multiplier % 2 == 1 {
                result = self.add(result, addend);
            }
            addend = self.add(addend, addend); 
            multiplier /= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 測試曲線 E: y^2 = x^3 + 2x + 2 over F_17
    // G = (5, 1) 為生成元，整個群的階為 19（質數）。
    const P: u64 = 17;
    fn fe(v: u64) -> FieldElement { FieldElement::new(v, P) }
    fn pt(x: u64, y: u64) -> ECPoint { ECPoint::Point { x: fe(x), y: fe(y) } }
    fn curve() -> Curve { Curve { a: fe(2), b: fe(2) } }
    fn g() -> ECPoint { pt(5, 1) }

    #[test]
    fn contains_on_curve_points() {
        let c = curve();
        assert!(c.contains(g()));                 // 真點
        assert!(c.contains(ECPoint::Infinity));   // 無窮遠點一律視為在曲線上
    }

    #[test]
    fn rejects_off_curve_point() {
        assert!(!curve().contains(pt(5, 2)));     // (5,2) 不滿足曲線方程式
    }

    #[test]
    fn identity_law() {
        // P + O = P 且 O + P = P
        let c = curve();
        assert!(c.add(g(), ECPoint::Infinity) == g());
        assert!(c.add(ECPoint::Infinity, g()) == g());
    }

    #[test]
    fn inverse_sums_to_infinity() {
        // P + (-P) = O，其中 -P = (x, prime - y) = (5, 16)
        assert!(curve().add(g(), pt(5, 16)) == ECPoint::Infinity);
    }

    #[test]
    fn point_doubling() {
        // 2G = (6, 3)
        let c = curve();
        let two_g = c.add(g(), g());
        assert!(two_g == pt(6, 3));
        assert!(c.contains(two_g));               // 結果仍在曲線上
    }

    #[test]
    fn general_addition() {
        // G + 2G = 3G = (10, 6)，此處 x1 != x2 走一般相加分支
        let c = curve();
        let three_g = c.add(g(), c.add(g(), g()));
        assert!(three_g == pt(10, 6));
        assert!(c.contains(three_g));
    }

    #[test]
    fn doubling_with_y_zero_is_infinity() {
        // 當 y = 0 時切線垂直，2P = O。
        // 改用曲線 y^2 = x^3 + x over F_5，其上 (0,0) 是真實的 2-撓點。
        let c2 = Curve { a: FieldElement::new(1, 5), b: FieldElement::new(0, 5) };
        let p0 = ECPoint::Point { x: FieldElement::new(0, 5), y: FieldElement::new(0, 5) };
        assert!(c2.contains(p0));                 // 確認 (0,0) 真的在曲線上
        assert!(c2.add(p0, p0) == ECPoint::Infinity);
    }

    #[test]
    fn scalar_mul_small_values() {
        let c = curve();
        assert!(c.scalar_mul(0, g()) == ECPoint::Infinity);
        assert!(c.scalar_mul(1, g()) == g());
        assert!(c.scalar_mul(2, g()) == pt(6, 3));
        assert!(c.scalar_mul(3, g()) == pt(10, 6));
    }

    #[test]
    fn scalar_mul_matches_repeated_addition() {
        // kG 應等於把 G 連加 k 次；範圍跨過群階 19 以驗證循環
        let c = curve();
        let mut acc = ECPoint::Infinity;
        for k in 0..25 {
            assert!(c.scalar_mul(k, g()) == acc, "scalar_mul({}) 與逐次相加結果不符", k);
            acc = c.add(acc, g());
        }
    }

    #[test]
    fn group_order_is_19() {
        // 群階為 19：19G = O，且 20G 繞回 G
        let c = curve();
        assert!(c.scalar_mul(19, g()) == ECPoint::Infinity);
        assert!(c.scalar_mul(20, g()) == g());
    }

    #[test]
    fn all_multiples_stay_on_curve() {
        // 1G ~ 18G 全部都應落在曲線上
        let c = curve();
        let mut acc = ECPoint::Infinity;
        for _ in 1..19 {
            acc = c.add(acc, g());
            assert!(c.contains(acc));
        }
    }
}
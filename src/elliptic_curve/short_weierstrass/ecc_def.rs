use std::fmt::Display;

use crate::{elliptic_curve::short_weierstrass::ecc_point::EllipticPoint, finite_field::field::{FiniteField, FiniteFieldTrait}};

#[derive(Debug, Clone, Copy)]
pub struct EllipticCurve {
    pub a: FiniteField,
    pub b: FiniteField,
}

pub trait EllipticCurveTraits {
    fn new(a: FiniteField, b: FiniteField) -> Self;
    fn satisfy_curve(&self, x: FiniteField, y: FiniteField) -> bool;
}

impl Display for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y^2 = x^3 + {0}x + {1}", self.a.a, self.b.a)
    }
}

impl EllipticCurveTraits for EllipticCurve {
    fn new(a: FiniteField, b: FiniteField) -> Self {
        EllipticCurve { a, b }
    }

    // y^2 == x3 + ax + b
    fn satisfy_curve(&self, x: FiniteField, y: FiniteField) -> bool {
        y ^ 2 == (x ^ 3) + (self.a * x) + self.b
    }
}

pub fn main() {
    println!("Hello World from the Elliptic Curve Village");
    let a = FiniteField::new(5, 7);
    let b = FiniteField::new(2, 7);
    let curve_a = EllipticCurve::new(a, b);
    println!("The elliptic curve is: {}", curve_a);

    let a1 = FiniteField::new(0, 907);
    let a2 = FiniteField::new(7, 907);
    let secp256k1 = EllipticCurve::new(a1, a2);
    println!("The elliptic curve is {}", secp256k1);

    println!("ELLIPTIC CURVE");
    let a = FiniteField::new(3, 907);
    let b = FiniteField::new(4, 907);
    let curve = EllipticCurve::new(a, b);
    let x = FiniteField::new(5, 907);
    let y = FiniteField::new(12, 907);
    let point = EllipticPoint::new(x, y, curve);
    println!("{}", point);
}

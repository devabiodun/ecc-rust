// use std::fmt::{Display, Formatter};

// use super::ecc_def::EllipticCurve;
// use crate::finite_field::field::FiniteField;

// #[derive(Debug, Clone, Copy)]
// pub struct EllipticPoint {
//     pub x: FiniteField,
//     pub y: FiniteField,
//     curve: EllipticCurve,
// }

// impl EllipticPoint {
//     pub fn new(x: FiniteField, y: FiniteField, curve: EllipticCurve) -> Self {
//         EllipticPoint { x, y, curve }
//     }
// }

// impl Display for EllipticPoint {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
//         write!(
//             f,
//             "({},{}) is a point in {}",
//             self.x.a, self.y.a, self.curve
//         )
//     }
// }

use std::fmt::{Display,Formatter};

use super::ecc_def::EllipticCurve;
use crate::{elliptic_curve::short_weierstrass::ecc_def::EllipticCurveTraits, finite_field::field::FiniteField};


#[derive(Debug, Clone, Copy)]
pub struct EllipticPoint{
    pub x: FiniteField,
    pub y: FiniteField,
    curve: EllipticCurve
}

impl EllipticPoint {
    pub fn new(x: FiniteField, y: FiniteField, curve: EllipticCurve) -> Self {
        assert!(curve.satisfy_curve(x, y),"Not a point");
        EllipticPoint { x, y, curve }
    }
}

impl Display for EllipticPoint{
    fn fmt(&self,f: &mut Formatter) -> Result<(), std::fmt::Error>{
        write!(f,"({},{}) is a point in {}",self.x.a, self.y.a , self.curve)
    }
}
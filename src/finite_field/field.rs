use super::traits::{isPrime, is_prime};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, BitXor, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct FiniteField {
    pub a: u64,
    p: u64,
}

pub trait FiniteFieldTrait {
    fn new(a: u64, p: u64) -> Self;
    fn inv(self) -> Self;
}

pub trait Pow<RHS> {
    type Output;

    // Required method
    fn pow(self, rhs: RHS) -> Self::Output;
}

impl FiniteFieldTrait for FiniteField {
    fn new(a: u64, p: u64) -> Self {
        assert!(a < p, "Not an element of the prime field");
        FiniteField { a, p }
    }

    fn inv(self) -> Self {
        self ^ (self.p as usize - 2)
    }
}

impl Display for FiniteField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} F[{}]", self.a, self.p)
    }
}

impl isPrime for FiniteField {
    fn isPrime(&self) -> bool {
        is_prime(self.p, 12)
    }
}

impl Add for FiniteField {
    type Output = Self;
    fn add(self, other: FiniteField) -> Self {
        if other.p != self.p {
            panic!("Add Operation should be within the field");
        }

        FiniteField {
            a: (self.a + other.a) % self.p,
            p: self.p,
        }
    }
}

impl Mul for FiniteField {
    type Output = Self;
    fn mul(self, other: FiniteField) -> Self {
        if other.p != self.p {
            panic!("Mul Operation should be within the field");
        }
        FiniteField {
            a: (self.a * other.a) % self.p,
            p: self.p,
        }
    }
}

impl Div for FiniteField {
    type Output = Self;

    fn div(self, other: FiniteField) -> Self {
        if other.p != self.p {
            panic!("Div Operation should be within the field");
        }

        let p_minus_2 = self.p - 2;
        let inv_mul = other.a * p_minus_2 % self.p;

        FiniteField {
            a: (self.a * inv_mul) % self.p,
            p: self.p,
        }
    }
}

impl Sub for FiniteField {
    type Output = Self;
    fn sub(self, other: FiniteField) -> Self {
        if other.p != self.p {
            panic!("Div Operation should be within the field");
        }

        println!("THIS IS THE SUB SOLUTION: {} {}", self.a, other.a);

        // inverse of subtraction where p - a
        let inv_add = self.p - other.a;

        FiniteField {
            a: (self.a + inv_add) % self.p,
            p: self.p,
        }
    }
}

impl PartialEq for FiniteField {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.p, other.p, "You can only compare between same field");
        self.a == other.a
    }
}

impl Eq for FiniteField {}

impl BitXor<usize> for FiniteField {
    type Output = FiniteField;
    fn bitxor(self, rhs: usize) -> FiniteField {
        let n = if rhs > self.p as usize {
            rhs % self.p as usize
        } else {
            rhs
        };
        let mut res = self;
        for _ in 2..=n {
            res = res * self;
        }
        res
    }
}

pub fn main() {
    println!("A main function from the finite field village");
    let a = FiniteField::new(3, 5);
    let b = 5;
    // let b = FiniteField::new(2, 7);
    let c = a ^ b;
    println!("{a}^{b} = {c}");

    println!("INVERSE ELEMENT");
    let a = FiniteField::new(436, 563);
    let b = a.inv();
    println!("The inverse of {a} is {b}");
    // let zero = Field::new(0,13);
    // let any_number = Field::new(2,7);
    // let identity = any_number + zero == any_number;
    // println!("Zero is an identity: {identity}");
}

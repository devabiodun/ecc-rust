use ecc::{
    elliptic_curve::short_weierstrass::ecc_def,
    finite_field::field::{self, FiniteField, FiniteFieldTrait},
};

fn main() {
    let a = FiniteField::new(8, 9);
    let b = FiniteField::new(3, 9);
    let c = a + b;
    println!("{:?} is a finite field element", a);
    println!("{:?} is a finite field element", b);
    println!("{}+{}={}", a, b, c);

    let c = a * b;
    println!("{:?} is a finite field element", a);
    println!("{:?} is a finite field element", b);
    println!("{}*{}={}", a, b, c);

    let c = a / b;
    println!("{:?} is a finite field element", a);
    println!("{:?} is a finite field element", b);
    println!("{}/{}={}", a, b, c);

    let c = a - b;
    println!("{:?} is a finite field element", a);
    println!("{:?} is a finite field element", b);
    println!("{}-{}={}", a, b, c);

    println!("Hello, world!");

    field::main();
    ecc_def::main();
}

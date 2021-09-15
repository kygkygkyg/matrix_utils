mod lib;

use crate::lib::math::complex;

fn main() {
    let tmp = complex::Complex{real: 0.0, imag: 3.0};
    println!("{} + {}i", tmp.real, tmp.imag);
    println!("Hello, world!");
}

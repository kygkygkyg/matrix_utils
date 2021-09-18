mod lib;

use crate::lib::math::complex;

fn main() {
    let tmp_1 = complex::Complex{real: 0.0, imag: 3.0};
    let tmp_2 = complex::Complex{real: 1.0, imag: 10.0};
    let tmp_3 = tmp_1.add(&tmp_2);
    let abs = tmp_3.mul(&tmp_3.conjugate());
    println!("{} + {}i", tmp_1.real, tmp_1.imag);
    println!("{} + {}i", tmp_2.real, tmp_2.imag);
    println!("tmp_1 + tmp_2 = {}", tmp_3.to_string());

    // 絶対値が正しいかどうか
    println!("tmp_3 abs                    :: {}", abs.to_string());
    println!("tmp_3 abs(using abs function)::{}", tmp_3.abs());

    println!("{}, {}, {}", tmp_1.to_string(), tmp_2.to_string(), tmp_3.to_string());
}

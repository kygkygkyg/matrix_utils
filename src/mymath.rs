#![allow(dead_code)]

// mod matrix_utils


pub mod complex {

    pub struct Complex {
        pub real: f64,
        pub imag: f64,
    }
    impl Complex { // imple <構造体名> でメンバ関数を定義できる
        // Comple構造体コンストラクタ
        pub fn new(real: f64, imag: f64) -> Complex {
            Complex {real, imag}
        }
        // 足し算定義
        pub fn add(&self, x: &Complex) -> Complex {
            Complex {
                real: self.real + x.real,
                imag: self.imag + x.imag
            }
        }
        // 引き算定義
        pub fn sub(&self, x: &Complex) -> Complex {
            Complex {
                real: self.real - x.real,
                imag: self.imag - x.imag
            }
        }
        // 掛け算定義
        pub fn mul(&self, x: &Complex) -> Complex {
            Complex {
                real: self.real * x.real - self.imag * x.imag,
                imag: self.real * x.imag + self.imag * x.real
            }
        }
        // 絶対値定義
        pub fn abs(&self) -> f64 {
            self.mul(&self.conjugate()).real
        }
        // 文字列化
        pub fn to_string(&self) -> String {
            format!("{} + {} I", self.real, self.imag)
            // String::from("{} + i{}", self.real, self.imag)
        }
        // エルミート共役
        pub fn conjugate(&self) -> Complex {
            Complex{real: self.real, imag: - self.imag}
        }

    }

}

pub mod mat {
    struct Mat {
        shape: Vec<i32>,
    }
}


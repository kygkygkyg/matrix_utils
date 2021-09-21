#![allow(dead_code)]

// mod matrix_utils
// mod traits::operators;
// use crate::mat_utils::utils::traits::operators::*; // 最終的にはこっちを使いたい


pub mod complex {
    pub trait Operators {
        /* 関連型といわれる;
        この記法を用いると、トレイとをさらに一般的に扱いたい場合に
        ジェネリクスのハードコードが避けられる*/
        type A;
        type B;
    
        fn add(&self, _: &Self::A) -> Self::A;
        fn sub(&self, _: &Self::A) -> Self::A;
        fn mul(&self, _: &Self::A) -> Self::A;
        fn conjugate(&self) -> Self::A;
        fn abs(&self) -> Self::B;
        fn to_string(&self) -> String;
    
    }

    pub struct Complex {
        pub real: f64,
        pub imag: f64,
    }
    impl Complex { // imple <構造体名> でメンバ関数を定義できる
        // Comple構造体コンストラクタ
        pub fn new(real: f64, imag: f64) -> Complex {
            Complex {real, imag}
        }
    }

    impl Operators for Complex {
        type A = Complex;
        type B = f64;

        // 足し算定義
        fn add(&self, x: &Complex) -> Complex {
            Complex {
                real: self.real + x.real,
                imag: self.imag + x.imag
            }
        }
        // 引き算定義
        fn sub(&self, x: &Complex) -> Complex {
            Complex {
                real: self.real - x.real,
                imag: self.imag - x.imag
            }
        }
        // 掛け算定義
        fn mul(&self, x: &Complex) -> Complex {
            Complex {
                real: self.real * x.real - self.imag * x.imag,
                imag: self.real * x.imag + self.imag * x.real
            }
        }
        // エルミート共役
        fn conjugate(&self) -> Complex {
            Complex{real: self.real, imag: - self.imag}
        }
        // 絶対値定義
        fn abs(&self) -> f64 {
            self.mul(&self.conjugate()).real
        }
        // 文字列化
        fn to_string(&self) -> String {
            format!("{} + {} I", self.real, self.imag)
            // String::from("{} + i{}", self.real, self.imag)
        }

    }

}
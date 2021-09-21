// define algebra for scalar, vector, array

pub mod operators {

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
}
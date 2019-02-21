#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

use traits::*;

pub trait ArrayTools: Sized + Sealed {
    type Element;
    const LEN: usize;
    fn as_slice(&self) -> &[Self::Element];
    fn as_mut_slice(&mut self) -> &mut [Self::Element];

    type Tuple;
    fn from_tuple(tuple: Self::Tuple) -> Self;
    fn into_tuple(self) -> Self::Tuple;

    fn generate<F>(f: F) -> Self
        where Self: ArrayGenerate<F>
    {
        ArrayGenerate::generate(f)
    }

    fn repeat<T: Clone>(x: T) -> Self
        where Self: ArrayRepeat<T>
    {
        ArrayRepeat::repeat(x)
    }

    fn indices() -> Self
        where Self: ArrayIndices
    {
        ArrayIndices::indices()
    }

    fn map<T, F>(self, f: F) -> <Self as ArrayMap<T, F>>::Output
        where Self: ArrayMap<T, F>
    {
        ArrayMap::map(self, f)
    }

    fn zip<T>(self, other: T) -> <Self as ArrayZip<T>>::Output
        where Self: ArrayZip<T>
    {
        ArrayZip::zip(self, other)
    }

    fn as_ref_array<'a>(&'a self) -> <&'a Self as ArrayAsRef>::Output
        where &'a Self: ArrayAsRef
    {
        ArrayAsRef::as_ref(self)
    }

    fn as_mut_array<'a>(&'a mut self) -> <&'a mut Self as ArrayAsMut>::Output
        where &'a mut Self: ArrayAsMut
    {
        ArrayAsMut::as_mut(self)
    }

    fn push_back<U>(self, item: U) -> <Self as ArrayPush<U>>::Output
        where Self: ArrayPush<U>
    {
        ArrayPush::push_back(self, item)
    }
    fn push_front<U>(self, item: U) -> <Self as ArrayPush<U>>::Output
        where Self: ArrayPush<U>
    {
        ArrayPush::push_front(self, item)
    }

    fn pop_back<U>(self) -> (<Self as ArrayPop<U>>::Output, U)
        where Self: ArrayPop<U>
    {
        ArrayPop::pop_back(self)
    }
    fn pop_front<U>(self) -> (<Self as ArrayPop<U>>::Output, U)
        where Self: ArrayPop<U>
    {
        ArrayPop::pop_front(self)
    }
}

mod traits {
    pub trait Sealed {}

    pub trait ArrayGenerate<F> {
        fn generate(f: F) -> Self;
    }

    pub trait ArrayRepeat<T> {
        fn repeat(x: T) -> Self;
    }

    pub trait ArrayIndices {
        fn indices() -> Self;
    }

    pub trait ArrayMap<T, F> {
        type Output;
        fn map(array: Self, f: F) -> Self::Output;
    }

    pub trait ArrayZip<T> {
        type Output;
        fn zip(array: Self, other: T) -> Self::Output;
    }

    pub trait ArrayAsRef {
        type Output;
        fn as_ref(array: Self) -> Self::Output;
    }

    pub trait ArrayAsMut {
        type Output;
        fn as_mut(array: Self) -> Self::Output;
    }

    pub trait ArrayPush<T> {
        type Output;
        fn push_back(array: Self, item: T) -> Self::Output;
        fn push_front(array: Self, item: T) -> Self::Output;
    }

    pub trait ArrayPop<T> {
        type Output;
        fn pop_back(array: Self) -> (Self::Output, T);
        fn pop_front(array: Self) -> (Self::Output, T);
    }
}

#[allow(unused_mut, unused_variables)]
mod impls {
    use super::*;

    macro_rules! replace_ident {
        ($i:ident => $($j:tt)*) => ($($j)*)
    }

    macro_rules! array_by_cloning {
        ($x:ident:) => ( [] );
        ($x:ident: $first:ident $($i:ident)*) => ( [$(replace_ident!($i => $x.clone()),)* $x] );
    }

    macro_rules! impl_for_size {
        ($n:literal $fn_trait:ident => $($i:ident)* / $($j:ident)*) => (

            impl<T> Sealed for [T; $n] {}
            impl<T> ArrayTools for [T; $n] {
                type Element = T;
                const LEN: usize = $n;
                fn as_slice(&self) -> &[Self::Element] { self }
                fn as_mut_slice(&mut self) -> &mut [Self::Element] { self }

                type Tuple = ($(replace_ident!($i => T),)*);
                fn from_tuple(tuple: Self::Tuple) -> Self {
                    let ($($i,)*) = tuple;
                    [$($i,)*]
                }
                fn into_tuple(self) -> Self::Tuple {
                    let [$($i,)*] = self;
                    ($($i,)*)
                }
            }
            impl<T, F> ArrayGenerate<F> for [T; $n]
                where F: $fn_trait() -> T
            {
                fn generate(mut f: F) -> Self {
                    [$(replace_ident!($i => f()),)*]
                }
            }
            impl<T> ArrayRepeat<T> for [T; $n]
                where T: Clone
            {
                fn repeat(x: T) -> Self {
                    array_by_cloning!(x: $($i)*)
                }
            }
            impl ArrayIndices for [usize; $n] {
                fn indices() -> Self {
                    let mut i = 0;
                    ArrayTools::generate(|| { let t = i; i += 1; t })
                }
            }
            impl<T, U, F> ArrayMap<U, F> for [T; $n]
                where F: $fn_trait(T) -> U
            {
                type Output = [U; $n];
                fn map(array: Self, mut f: F) -> Self::Output {
                    let [$($i,)*] = array;
                    [$(f($i),)*]
                }
            }
            impl<T, U> ArrayZip<[U; $n]> for [T; $n] {
                type Output = [(T, U); $n];
                fn zip(array: Self, other: [U; $n]) -> Self::Output {
                    let [$($i,)*] = array;
                    let [$($j,)*] = other;
                    [$(($i,$j),)*]
                }
            }
            impl<'a, T> ArrayAsRef for &'a [T; $n]
            {
                type Output = [&'a T; $n];
                fn as_ref(array: Self) -> Self::Output {
                    let [$($i,)*] = array;
                    [$($i,)*]
                }
            }
            impl<'a, T> ArrayAsMut for &'a mut [T; $n]
            {
                type Output = [&'a mut T; $n];
                fn as_mut(array: Self) -> Self::Output {
                    let [$($i,)*] = array;
                    [$($i,)*]
                }
            }
            impl<T> ArrayPush<T> for [T; $n] {
                type Output = [T; $n+1];
                fn push_back(array: Self, item: T) -> Self::Output {
                    let [$($i,)*] = array;
                    [$($i,)* item]
                }
                fn push_front(array: Self, item: T) -> Self::Output {
                    let [$($i,)*] = array;
                    [item, $($i,)*]
                }
            }
            impl<T> ArrayPop<T> for [T; $n+1] {
                type Output = [T; $n];
                fn pop_back(array: Self) -> (Self::Output, T) {
                    let [$($i,)* item] = array;
                    ([$($i,)*], item)
                }
                fn pop_front(array: Self) -> (Self::Output, T) {
                    let [item, $($i,)*] = array;
                    ([$($i,)*], item)
                }
            }

        )
    }

    // <https://play.rust-lang.org/?gist=090fb0d6437dd5e5430180fb80b77b76>
    impl_for_size!(0 FnOnce => /);
    impl_for_size!(1 FnOnce => a0 / b0);
    impl_for_size!(2 FnMut => a0 a1 / b0 b1);
    impl_for_size!(3 FnMut => a0 a1 a2 / b0 b1 b2);
    impl_for_size!(4 FnMut => a0 a1 a2 a3 / b0 b1 b2 b3);
    impl_for_size!(5 FnMut => a0 a1 a2 a3 a4 / b0 b1 b2 b3 b4);
    impl_for_size!(6 FnMut => a0 a1 a2 a3 a4 a5 / b0 b1 b2 b3 b4 b5);
    impl_for_size!(7 FnMut => a0 a1 a2 a3 a4 a5 a6 / b0 b1 b2 b3 b4 b5 b6);
    impl_for_size!(8 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 / b0 b1 b2 b3 b4 b5 b6 b7);
    impl_for_size!(9 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 / b0 b1 b2 b3 b4 b5 b6 b7 b8);
    impl_for_size!(10 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9);
    impl_for_size!(11 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10);
    impl_for_size!(12 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11);
    impl_for_size!(13 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12);
    impl_for_size!(14 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13);
    impl_for_size!(15 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14);
    impl_for_size!(16 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15);
    impl_for_size!(17 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16);
    impl_for_size!(18 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17);
    impl_for_size!(19 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18);
    impl_for_size!(20 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19);
    impl_for_size!(21 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20);
    impl_for_size!(22 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21);
    impl_for_size!(23 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22);
    impl_for_size!(24 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23);
    impl_for_size!(25 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24);
    impl_for_size!(26 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25);
    impl_for_size!(27 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26);
    impl_for_size!(28 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27);
    impl_for_size!(29 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28);
    impl_for_size!(30 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29);
    impl_for_size!(31 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 a30 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30);
    impl_for_size!(32 FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 a30 a31 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31);
}

#[cfg(test)]
mod tests {
    use super::ArrayTools;

    #[test]
    fn it_works() {
        let mut a = [1];
        *a.as_mut_array()[0] = 2;
        assert_eq!(a, [2]);

        a = ArrayTools::from_tuple((3,));
        assert_eq!(a, [3]);
        assert_eq!(a.into_tuple(), (3,));

        let a = a.map(|x| x as f32);
        assert_eq!(a, [3.0]);

        let a0: [u8; 0] = [];
        let a1 = a0.push_back(Default::default());
        assert_eq!(a1, [0]);
        let a2 = a1.push_back(2);
        assert_eq!(a2, [0, 2]);
        let b1 = a2.pop_back();
        assert_eq!(b1, ([0], 2));

        let iota: [_; 10] = ArrayTools::indices();
        assert_eq!(iota, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut v = Vec::with_capacity(111);
        v.push(1);
        let a: [_; 3] = ArrayTools::repeat(v);
        assert_eq!(a[0], [1]);
        assert_eq!(a[1], [1]);
        assert_eq!(a[2], [1]);
        assert_eq!(a[0].capacity(), 1);
        assert_eq!(a[1].capacity(), 1);
        assert_eq!(a[2].capacity(), 111);
    }
}

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

    fn map<T, F>(self, f: F) -> <Self as ArrayMap<T, F>>::Output
        where Self: ArrayMap<T, F>
    {
        ArrayMap::map(self, f)
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

    pub trait ArrayMap<T, F> {
        type Output;
        fn map(self, f: F) -> Self::Output;
    }

    pub trait ArrayAsRef {
        type Output;
        fn as_ref(self) -> Self::Output;
    }

    pub trait ArrayAsMut {
        type Output;
        fn as_mut(self) -> Self::Output;
    }

    pub trait ArrayPush<T> {
        type Output;
        fn push_back(self, item: T) -> Self::Output;
        fn push_front(self, item: T) -> Self::Output;
    }

    pub trait ArrayPop<T> {
        type Output;
        fn pop_back(self) -> (Self::Output, T);
        fn pop_front(self) -> (Self::Output, T);
    }
}

mod impls {
    use super::*;

    impl<T> Sealed for [T; 0] {}
    impl<T> ArrayTools for [T; 0] {
        type Element = T;
        const LEN: usize = 0;
        fn as_slice(&self) -> &[Self::Element] { self }
        fn as_mut_slice(&mut self) -> &mut [Self::Element] { self }

        type Tuple = ();
        fn from_tuple(_tuple: Self::Tuple) -> Self { [] }
        fn into_tuple(self) -> Self::Tuple { () }
    }
    impl<T, U, F> ArrayMap<U, F> for [T; 0] {
        type Output = [U; 0];
        fn map(self, _f: F) -> Self::Output { [] }
    }
    impl<'a, T> ArrayAsRef for &'a [T; 0]
    {
        type Output = [&'a T; 0];
        fn as_ref(self) -> Self::Output { [] }
    }
    impl<'a, T> ArrayAsMut for &'a mut [T; 0]
    {
        type Output = [&'a mut T; 0];
        fn as_mut(self) -> Self::Output { [] }
    }
    impl<T> ArrayPush<T> for [T; 0] {
        type Output = [T; 1];
        fn push_back(self, item: T) -> Self::Output { [item] }
        fn push_front(self, item: T) -> Self::Output { [item] }
    }
    impl<T> ArrayPop<T> for [T; 1] {
        type Output = [T; 0];
        fn pop_back(self) -> (Self::Output, T) {
            let [item,] = self;
            ([], item)
        }
        fn pop_front(self) -> (Self::Output, T) {
            let [item,] = self;
            ([], item)
        }
    }

    impl<T> Sealed for [T; 1] {}
    impl<T> ArrayTools for [T; 1] {
        type Element = T;
        const LEN: usize = 1;
        fn as_slice(&self) -> &[Self::Element] { self }
        fn as_mut_slice(&mut self) -> &mut [Self::Element] { self }

        type Tuple = (T,);
        fn from_tuple(tuple: Self::Tuple) -> Self {
            let (a,) = tuple;
            [a,]
        }
        fn into_tuple(self) -> Self::Tuple {
            let [a,] = self;
            (a,)
        }
    }
    impl<T, U, F> ArrayMap<U, F> for [T; 1]
        where F: FnOnce(T)->U
    {
        type Output = [U; 1];
        fn map(self, f: F) -> Self::Output {
            let [a,] = self;
            [f(a),]
        }
    }
    impl<'a, T> ArrayAsRef for &'a [T; 1]
    {
        type Output = [&'a T; 1];
        fn as_ref(self) -> Self::Output {
            let [a,] = self;
            [a,]
        }
    }
    impl<'a, T> ArrayAsMut for &'a mut [T; 1]
    {
        type Output = [&'a mut T; 1];
        fn as_mut(self) -> Self::Output {
            let [a,] = self;
            [a,]
        }
    }
    impl<T> ArrayPush<T> for [T; 1] {
        type Output = [T; 2];
        fn push_back(self, item: T) -> Self::Output {
            let [a,] = self;
            [a, item]
        }
        fn push_front(self, item: T) -> Self::Output {
            let [a,] = self;
            [item, a,]
        }
    }
    impl<T> ArrayPop<T> for [T; 2] {
        type Output = [T; 1];
        fn pop_back(self) -> (Self::Output, T) {
            let [a,item,] = self;
            ([a,], item)
        }
        fn pop_front(self) -> (Self::Output, T) {
            let [item,a,] = self;
            ([a,], item)
        }
    }

    macro_rules! replace_ident {
        ($i:ident => $j:ident) => ($j)
    }
    macro_rules! impl_for_size {
        ($n:literal => $($i:ident)*) => (

            impl<T> Sealed for [T; $n] {}
            impl<T> ArrayTools for [T; $n] {
                type Element = T;
                const LEN: usize = $n;
                fn as_slice(&self) -> &[Self::Element] { self }
                fn as_mut_slice(&mut self) -> &mut [Self::Element] { self }

                type Tuple = ($(replace_ident!($i => T)),*);
                fn from_tuple(tuple: Self::Tuple) -> Self {
                    let ($($i),*) = tuple;
                    [$($i),*]
                }
                fn into_tuple(self) -> Self::Tuple {
                    let [$($i),*] = self;
                    ($($i),*)
                }
            }
            impl<T, U, F> ArrayMap<U, F> for [T; $n]
                where F: FnMut(T)->U
            {
                type Output = [U; $n];
                fn map(self, mut f: F) -> Self::Output {
                    let [$($i),*] = self;
                    [$(f($i)),*]
                }
            }
            impl<'a, T> ArrayAsRef for &'a [T; $n]
            {
                type Output = [&'a T; $n];
                fn as_ref(self) -> Self::Output {
                    let [$($i),*] = self;
                    [$($i),*]
                }
            }
            impl<'a, T> ArrayAsMut for &'a mut [T; $n]
            {
                type Output = [&'a mut T; $n];
                fn as_mut(self) -> Self::Output {
                    let [$($i),*] = self;
                    [$($i),*]
                }
            }
            impl<T> ArrayPush<T> for [T; $n] {
                type Output = [T; $n+1];
                fn push_back(self, item: T) -> Self::Output {
                    let [$($i),*] = self;
                    [$($i),*, item]
                }
                fn push_front(self, item: T) -> Self::Output {
                    let [$($i),*] = self;
                    [item, $($i),*]
                }
            }
            impl<T> ArrayPop<T> for [T; $n+1] {
                type Output = [T; $n];
                fn pop_back(self) -> (Self::Output, T) {
                    let [$($i),*, item] = self;
                    ([$($i),*], item)
                }
                fn pop_front(self) -> (Self::Output, T) {
                    let [item, $($i),*] = self;
                    ([$($i),*], item)
                }
            }

        )
    }
    impl_for_size!(2 => i0 i1);
    impl_for_size!(3 => i0 i1 i2);
    impl_for_size!(4 => i0 i1 i2 i3);
    impl_for_size!(5 => i0 i1 i2 i3 i4);
    impl_for_size!(6 => i0 i1 i2 i3 i4 i5);
    impl_for_size!(7 => i0 i1 i2 i3 i4 i5 i6);
    impl_for_size!(8 => i0 i1 i2 i3 i4 i5 i6 i7);
    impl_for_size!(9 => i0 i1 i2 i3 i4 i5 i6 i7 i8);
    impl_for_size!(10 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9);
    impl_for_size!(11 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10);
    impl_for_size!(12 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11);
    impl_for_size!(13 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12);
    impl_for_size!(14 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13);
    impl_for_size!(15 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14);
    impl_for_size!(16 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15);
    impl_for_size!(17 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16);
    impl_for_size!(18 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17);
    impl_for_size!(19 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18);
    impl_for_size!(20 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19);
    impl_for_size!(21 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20);
    impl_for_size!(22 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21);
    impl_for_size!(23 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22);
    impl_for_size!(24 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23);
    impl_for_size!(25 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24);
    impl_for_size!(26 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25);
    impl_for_size!(27 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25 i26);
    impl_for_size!(28 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25 i26 i27);
    impl_for_size!(29 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25 i26 i27 i28);
    impl_for_size!(30 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25 i26 i27 i28 i29);
    impl_for_size!(31 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25 i26 i27 i28 i29 i30);
    impl_for_size!(32 => i0 i1 i2 i3 i4 i5 i6 i7 i8 i9 i10 i11 i12 i13 i14 i15 i16 i17 i18 i19 i20 i21 i22 i23 i24 i25 i26 i27 i28 i29 i30 i31);}

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
    }
}

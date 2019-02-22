#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

use self::traits::*;

/// An extension trait for working with fixed-length arrays.
///
/// Use it with
/// ```rust
/// use arraytools::ArrayTools;
/// ```
///
/// For more details, see the [crate-level documentation](index.html).
///
/// (This trait is sealed; you are not allowed to implement it yourself.)
pub trait ArrayTools: Sized + Sealed {
    /// The type of the elements in this array
    ///
    /// ```rust
    /// # type T = usize;
    /// # const N: usize = 1;
    /// use arraytools::ArrayTools;
    ///
    /// # fn _foo() where
    /// [T; N]: ArrayTools<Element = T>
    /// # {}
    /// ```
    type Element;

    /// The number of the elements in this array
    ///
    /// ```rust
    /// # type T = usize;
    /// # const N: usize = 1;
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!(<[T; N] as ArrayTools>::LEN, N);
    /// ```
    const LEN: usize;

    /// Extracts a slice containing the entire array.
    ///
    /// ```rust
    /// # type T = usize;
    /// # const N: usize = 1;
    /// use arraytools::ArrayTools;
    ///
    /// let array: [i32; 5] = [1, 2, 3, 4, 5];
    /// let slice: &[i32] = array.as_slice();
    /// assert_eq!(slice.len(), 5);
    /// ```
    fn as_slice(&self) -> &[Self::Element];

    /// Extracts a mutable slice containing the entire array.
    ///
    /// ```rust
    /// # type T = usize;
    /// # const N: usize = 1;
    /// use arraytools::ArrayTools;
    ///
    /// let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    /// let slice: &mut [i32] = array.as_mut_slice();
    /// assert_eq!(slice.len(), 5);
    /// ```
    fn as_mut_slice(&mut self) -> &mut [Self::Element];

    /// The homogeneous tuple type equivalent to this array type.
    ///
    /// ```rust
    /// # type T = usize;
    /// use arraytools::ArrayTools;
    ///
    /// # fn _foo() where
    /// [T; 4]: ArrayTools<Tuple = (T, T, T, T)>
    /// # {}
    /// ```
    type Tuple;

    /// Converts a homogeneous tuple into the equivalent array.
    ///
    /// Type: `(T, T, ..., T) -> [T; N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!(<[_; 3]>::from_tuple((1, 2, 3)), [1, 2, 3]);
    /// ```
    fn from_tuple(tuple: Self::Tuple) -> Self;

    /// Converts this array into the equivalent homogeneous tuple.
    ///
    /// Type: `[T; N] -> (T, T, ..., T)`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 2, 3].into_tuple(), (1, 2, 3));
    /// ```
    fn into_tuple(self) -> Self::Tuple;

    /// Builds an array by calling the provided function.
    ///
    /// Type: `F -> [T; N]`
    /// - when `N <= 1` this requires `F: FnOnce() -> T`
    /// - when `N > 1` this requires `F: FnMut() -> T`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// let mut x = 1;
    /// let array: [_; 5] = ArrayTools::generate(|| { x *= 2; x });
    /// assert_eq!(array, [2, 4, 8, 16, 32]);
    /// ```
    fn generate<F>(f: F) -> Self
        where Self: ArrayGenerate<F>
    {
        ArrayGenerate::generate(f)
    }

    /// Builds an array by cloning the provided value.
    ///
    /// Type: `T -> [T; N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// let mut v = Vec::with_capacity(10);
    /// v.push(42);
    /// let array: [_; 5] = ArrayTools::repeat(v);
    /// assert_eq!(array, [[42], [42], [42], [42], [42]]);
    /// assert_eq!(array[3].capacity(), 1);
    /// assert_eq!(array[4].capacity(), 10); // The last one is moved
    /// ```
    fn repeat<T: Clone>(x: T) -> Self
        where Self: ArrayRepeat<T>
    {
        ArrayRepeat::repeat(x)
    }

    /// Builds the array `[0, 1, 2, ..., LEN-1]`.
    ///
    /// Type: `() -> [usize; N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// let array: [_; 5] = ArrayTools::indices();
    /// assert_eq!(array, [0, 1, 2, 3, 4]);
    /// ```
    fn indices() -> Self
        where Self: ArrayIndices
    {
        ArrayIndices::indices()
    }

    /// Builds a new array by applying the provided function to each element of this array.
    ///
    /// Type: `([T; N], F) -> [U; N]`
    /// - when `N <= 1` this requires `F: FnOnce(T) -> U`
    /// - when `N > 1` this requires `F: FnMut(T) -> U`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 10, 100].map(|x| x + 10), [11, 20, 110]);
    /// ```
    fn map<T, F>(self, f: F) -> <Self as ArrayMap<T, F>>::Output
        where Self: ArrayMap<T, F>
    {
        ArrayMap::map(self, f)
    }

    /// Combines two equal-length arrays into an array of tuples.
    ///
    /// Type: `([T; N], [U; N]) -> [(T, U); N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([10, 20, 30].zip([1.0, 2.0, 3.0]), [(10, 1.0), (20, 2.0), (30, 3.0)]);
    /// ```
    fn zip<T>(self, other: T) -> <Self as ArrayZip<T>>::Output
        where Self: ArrayZip<T>
    {
        ArrayZip::zip(self, other)
    }

    /// Combines two equal-length arrays using the provided function.
    ///
    /// Type: `([T; N], [U; N], F) -> [V; N]`
    /// - when `N <= 1` this requires `F: FnOnce(T, U) -> V`
    /// - when `N > 1` this requires `F: FnMut(T, U) -> V`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([10, 20, 30].zip_with([3, 2, 1], std::ops::Add::add), [13, 22, 31]);
    /// ```
    fn zip_with<T, F>(self, other: T, f: F) -> <Self as ArrayZipWith<T, F>>::Output
        where Self: ArrayZipWith<T, F>
    {
        ArrayZipWith::zip_with(self, other, f)
    }

    /// Builds an array of references to the elements of this array.
    ///
    /// Type: `&'a [T; N] -> [&'a T; N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// let array = &[1, 2, 3, 4, 5];
    /// assert_eq!(array.as_ref_array(), [&1, &2, &3, &4, &5]);
    /// ```
    fn as_ref_array<'a>(&'a self) -> <&'a Self as ArrayAsRef>::Output
        where &'a Self: ArrayAsRef
    {
        ArrayAsRef::as_ref(self)
    }

    /// Builds an array of mutable references to the elements of this array.
    ///
    /// Type: `&'a mut [T; N] -> [&'a mut T; N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// let array = &mut [1, 2, 3];
    /// assert_eq!(array.as_ref_array(), [&mut 1, &mut 2, &mut 3]);
    /// ```
    fn as_mut_array<'a>(&'a mut self) -> <&'a mut Self as ArrayAsMut>::Output
        where &'a mut Self: ArrayAsMut
    {
        ArrayAsMut::as_mut(self)
    }

    /// Appends an item to this array, returning the new array
    ///
    /// Type: `([T; N], T) -> [T; N+1]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 2].push_back(10), [1, 2, 10]);
    /// ```
    fn push_back<U>(self, item: U) -> <Self as ArrayPush<U>>::Output
        where Self: ArrayPush<U>
    {
        ArrayPush::push_back(self, item)
    }

    /// Prepends an item to this array, returning the new array
    ///
    /// Type: `([T; N], T) -> [T; N+1]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 2].push_front(10), [10, 1, 2]);
    /// ```
    fn push_front<U>(self, item: U) -> <Self as ArrayPush<U>>::Output
        where Self: ArrayPush<U>
    {
        ArrayPush::push_front(self, item)
    }

    /// Splits the last item off from this array, returning a tuple of
    /// an array of the other elements and the split-off item.
    ///
    /// Type: `[T; N+1] -> ([T; N], T)`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 2, 3].pop_back(), ([1, 2], 3));
    /// ```
    fn pop_back<U>(self) -> (<Self as ArrayPop<U>>::Output, U)
        where Self: ArrayPop<U>
    {
        ArrayPop::pop_back(self)
    }

    /// Splits the first item off from this array, returning a tuple of
    /// an array of the other elements and the split-off item.
    ///
    /// Type: `[T; N+1] -> ([T; N], T)`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 2, 3].pop_front(), ([2, 3], 1));
    /// ```
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

    pub trait ArrayZipWith<T, F> {
        type Output;
        fn zip_with(array: Self, other: T, f: F) -> Self::Output;
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
        ($n:expr; $fn_trait:ident => $($i:ident)* / $($j:ident)*) => (

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
            impl<T, U, V, F> ArrayZipWith<[U; $n], F> for [T; $n]
                where F: $fn_trait(T, U) -> V
            {
                type Output = [V; $n];
                fn zip_with(array: Self, other: [U; $n], mut f: F) -> Self::Output {
                    let [$($i,)*] = array;
                    let [$($j,)*] = other;
                    [$(f($i,$j),)*]
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

    // <https://play.rust-lang.org/?gist=10a054305dfabf05f0c652e2df75fdcc>
    impl_for_size!(0; FnOnce => /);
    impl_for_size!(1; FnOnce => a0 / b0);
    impl_for_size!(2; FnMut => a0 a1 / b0 b1);
    impl_for_size!(3; FnMut => a0 a1 a2 / b0 b1 b2);
    impl_for_size!(4; FnMut => a0 a1 a2 a3 / b0 b1 b2 b3);
    impl_for_size!(5; FnMut => a0 a1 a2 a3 a4 / b0 b1 b2 b3 b4);
    impl_for_size!(6; FnMut => a0 a1 a2 a3 a4 a5 / b0 b1 b2 b3 b4 b5);
    impl_for_size!(7; FnMut => a0 a1 a2 a3 a4 a5 a6 / b0 b1 b2 b3 b4 b5 b6);
    impl_for_size!(8; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 / b0 b1 b2 b3 b4 b5 b6 b7);
    impl_for_size!(9; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 / b0 b1 b2 b3 b4 b5 b6 b7 b8);
    impl_for_size!(10; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9);
    impl_for_size!(11; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10);
    impl_for_size!(12; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11);
    impl_for_size!(13; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12);
    impl_for_size!(14; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13);
    impl_for_size!(15; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14);
    impl_for_size!(16; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15);
    impl_for_size!(17; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16);
    impl_for_size!(18; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17);
    impl_for_size!(19; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18);
    impl_for_size!(20; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19);
    impl_for_size!(21; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20);
    impl_for_size!(22; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21);
    impl_for_size!(23; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22);
    impl_for_size!(24; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23);
    impl_for_size!(25; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24);
    impl_for_size!(26; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25);
    impl_for_size!(27; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26);
    impl_for_size!(28; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27);
    impl_for_size!(29; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28);
    impl_for_size!(30; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29);
    impl_for_size!(31; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 a30 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30);
    impl_for_size!(32; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 a30 a31 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31);
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

        let sums = [1, 2, 3].zip_with([30, 20, 10], std::ops::Add::add);
        assert_eq!(sums, [31, 22, 13]);
    }
}

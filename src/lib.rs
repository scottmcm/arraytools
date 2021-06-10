#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

//! This crate offers the [`ArrayTools`] extension trait, which provides
//! a variety of helpful methods for working with fixed-size arrays.
//!
//! [`ArrayTools`]: trait.ArrayTools.html
//!
//! # Examples
//!
//! `Iterator`-like methods over arrays:
//!
//! ```rust
//! use arraytools::ArrayTools;
//!
//! assert_eq!([1, 2, 3].map(|x| x+1), [2, 3, 4]);
//! assert_eq!([1, 2].zip(["one", "two"]), [(1, "one"), (2, "two")]);
//! ```
//!
//! Ways to simplify array creation:
//!
//! ```rust
//! use arraytools::ArrayTools;
//!
//! let mut state = 1;
//! assert_eq!(<[_; 4]>::generate(|| { state *= 2; state }), [2, 4, 8, 16]);
//! assert_eq!(<[usize; 4]>::indices(), [0, 1, 2, 3]);
//!
//! let s = "hello".to_string(); // Something `!Copy`
//! assert_eq!(<[String; 3]>::repeat(s).as_ref_array(), ["hello", "hello", "hello"]);
//! ```
//!
//! Conversion to and from homogeneous tuples:
//!
//! ```rust
//! #[cfg(not(feature = "const-generics"))]
//! # {
//! use arraytools::ArrayTools;
//!
//! let mut array = [2, 3, 5, 7, 11];
//! assert_eq!(array.into_tuple(), (2, 3, 5, 7, 11));
//! array = ArrayTools::from_tuple((1, 1, 2, 3, 5));
//! assert_eq!(array, [1, 1, 2, 3, 5]);
//! # }
//! ```
//!
//! Like `Option`, most combinators here take `self`.  To not move something,
//! you can use [`.as_ref_array()`] or [`.as_mut_array()`]:
//!
//! [`.as_ref_array()`]: trait.ArrayTools.html#method.as_ref_array
//! [`.as_mut_array()`]: trait.ArrayTools.html#method.as_mut_array
//!
//! ```rust
//! use arraytools::ArrayTools;
//!
//! struct SevenStrings([String; 7]);
//! impl SevenStrings {
//!     fn push_str(&mut self, s: &str) {
//!         self.0.as_mut_array().for_each(|x: &mut String| x.push_str(s));
//!     }
//! }
//! ```
//!
//!
//! ## Const generics
//!
//! Some features in this crate also work for `const-generic` arrays. For this, you will need at least
//! rust version **1.51**. To enable this, use the `const-generics` feature. The api is exactly the same except:
//!
//! * `push_front`, `push_back`, `pop_front`, `pop_back`, `to_tuple` and `from_tuple` can only be used on arrays of which the length is *not* specified by const generics and are shorter than 32 elements
//! * methods on arrays which take closures allways take `FnMut`, where in the version without `const-generics` they could take `FnOnce` when the size was known to be 1 or 0.
//!
//! Using the `const-generics` feature, the following code will work:
//!
//! ```rust
//! # #[cfg(feature = "const-generics")]
//! # {
//! use arraytools::ArrayTools;
//!
//! fn add_ten<const N: usize>(arr: [usize; N]) -> [usize; N] {
//!     arr.map(|i| i + 10)
//! }
//!
//! assert_eq!(add_ten([1, 2, 3]), [11, 12, 13]);
//! assert_eq!(add_ten([1, 2, 3, 4, 5]), [11, 12, 13, 14, 15]);
//! assert_eq!(add_ten([1, 2, 3, 4, 5, 6, 7]), [11, 12, 13, 14, 15, 16, 17]);
//! # }
//! ```
//!


use self::traits::*;

/// An extension trait for working with fixed-length arrays.
///
/// Use it with
/// ```rust
/// use arraytools::ArrayTools;
/// ```
///
/// For an overview, see the [crate-level documentation](index.html).
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

    /// Converts a homogeneous tuple into the equivalent array.
    ///
    /// Type: `(T, T, ..., T) -> [T; N]`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!(<[_; 3]>::from_tuple((1, 2, 3)), [1, 2, 3]);
    /// ```
    fn from_tuple(tuple: <Self as ArrayTuple>::Tuple) -> Self where Self: ArrayTuple {
        ArrayTuple::from_tuple(tuple)
    }

    /// Converts this array into the equivalent homogeneous tuple.
    ///
    /// Type: `[T; N] -> (T, T, ..., T)`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!([1, 2, 3].into_tuple(), (1, 2, 3));
    /// ```
    fn into_tuple(self) -> <Self as ArrayTuple>::Tuple where Self: ArrayTuple {
        ArrayTuple::into_tuple(self)
    }

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
    /// ```
    fn repeat<T: Clone>(x: T) -> Self
        where Self: ArrayRepeat<T>
    {
        ArrayRepeat::repeat(x)
    }

    /// Builds an array containing a prefix of the provided iterator,
    /// or returns `None` if it didn't contain sufficient items.
    ///
    /// Type: `impl IntoIterator<Item = T> -> Option<[T; N]>`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// assert_eq!(<[i16; 4]>::from_iter(1..), Some([1, 2, 3, 4]));
    ///
    /// assert_eq!(<[_; 4]>::from_iter(1..4), None);
    /// assert_eq!(<[_; 4]>::from_iter(1..5), Some([1, 2, 3, 4]));
    /// assert_eq!(<[_; 4]>::from_iter(1..6), Some([1, 2, 3, 4]));
    ///
    /// assert_eq!(<[u8; 1]>::from_iter(Some(1)), Some([1]));
    /// assert_eq!(<[u8; 1]>::from_iter(None), None);
    /// ```
    fn from_iter<I: IntoIterator>(it: I) -> Option<Self>
        where Self: ArrayFromIter<I::IntoIter>
    {
        ArrayFromIter::from_iter(it.into_iter())
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
    #[must_use = "if you don't need the result, use `for_each`"]
    fn map<F>(self, f: F) -> <Self as ArrayMap<F>>::Output
        where Self: ArrayMap<F>
    {
        ArrayMap::map(self, f)
    }

    /// Runs the provided function on each element of this array.
    ///
    /// Type: `([T; N], F) -> ()`
    /// - when `N <= 1` this requires `F: FnOnce(T) -> ()`
    /// - when `N > 1` this requires `F: FnMut(T) -> ()`
    ///
    /// ```rust
    /// use arraytools::ArrayTools;
    ///
    /// let mut array = [1, 10, 100];
    /// array.as_mut_array().for_each(|x: &mut u8| *x += 10);
    /// assert_eq!(array, [11, 20, 110]);
    /// ```
    fn for_each<F>(self, f: F)
        where Self: ArrayMap<F, OutputElement = ()>
    {
        ArrayMap::map(self, f);
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
    fn as_ref_array<'a>(&'a self) -> <Self as ArrayAsRef<'a>>::Output
        where Self: ArrayAsRef<'a>
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
    fn as_mut_array<'a>(&'a mut self) -> <Self as ArrayAsMut<'a>>::Output
        where Self: ArrayAsMut<'a>
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
    #[must_use = "this returns the new array; it doesn't update the existing one"]
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
    #[must_use = "this returns the new array; it doesn't update the existing one"]
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
    #[must_use = "this returns the new array; it doesn't update the existing one"]
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
    #[must_use = "this returns the new array; it doesn't update the existing one"]
    fn pop_front<U>(self) -> (<Self as ArrayPop<U>>::Output, U)
        where Self: ArrayPop<U>
    {
        ArrayPop::pop_front(self)
    }
}

mod traits {
    pub trait Sealed {}

    pub trait ArrayTuple {
        type Tuple;

        fn from_tuple(tuple: Self::Tuple) -> Self;

        fn into_tuple(self) -> Self::Tuple;
    }

    pub trait ArrayGenerate<F> {
        fn generate(f: F) -> Self;
    }

    pub trait ArrayRepeat<T> {
        fn repeat(x: T) -> Self;
    }

    pub trait ArrayFromIter<I> {
        fn from_iter(it: I) -> Option<Self> where Self: Sized;
    }

    pub trait ArrayIndices {
        fn indices() -> Self;
    }

    pub trait ArrayMap<F> {
        type Output;
        type OutputElement;
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

    pub trait ArrayAsRef<'a> {
        type Output: 'a;
        fn as_ref(array: &'a Self) -> Self::Output;
    }

    pub trait ArrayAsMut<'a> {
        type Output: 'a;
        fn as_mut(array: &'a mut Self) -> Self::Output;
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

    #[cfg(not(feature = "const-generics"))]
    macro_rules! array_by_cloning {
        (:) => ( [] );
        ($x:ident: $($i:ident)*) => ( [$(replace_ident!($i => $x.clone()),)*] );
    }

    macro_rules! impl_tuple {
        ($n:expr; $fn_trait:ident => $($i:ident)* / $($j:ident)*) => {
            impl<T> ArrayTuple for [T; $n] {
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
        };
    }

    macro_rules! impl_push_pop {
        ($n:expr; $fn_trait:ident => $($i:ident)* / $($j:ident)*) => {
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
        };
    }

    #[cfg(not(feature = "const-generics"))]
    macro_rules! impl_for_size {
        ($n:expr; $fn_trait:ident => $($i:ident)* / $($j:ident)*) => (

            impl<T> Sealed for [T; $n] {}
            impl<T> ArrayTools for [T; $n] {
                type Element = T;
                const LEN: usize = $n;
                fn as_slice(&self) -> &[Self::Element] { self }
                fn as_mut_slice(&mut self) -> &mut [Self::Element] { self }
            }

            impl_tuple!($n; $fn_trait => $($i)* / $($j)*);

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
            impl<T, I> ArrayFromIter<I> for [T; $n]
                where I: Iterator<Item = T>
            {
                fn from_iter(mut it: I) -> Option<Self> {
                    Some([$(replace_ident!($i => it.next()?),)*])
                }
            }
            impl ArrayIndices for [usize; $n] {
                fn indices() -> Self {
                    let mut i = 0;
                    ArrayTools::generate(|| { let t = i; i += 1; t })
                }
            }
            impl<T, U, F> ArrayMap<F> for [T; $n]
                where F: $fn_trait(T) -> U
            {
                type Output = [U; $n];
                type OutputElement = U;
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
            impl<'a, T: 'a> ArrayAsRef<'a> for [T; $n]
            {
                type Output = [&'a T; $n];
                fn as_ref(array: &'a Self) -> Self::Output {
                    let [$($i,)*] = array;
                    [$($i,)*]
                }
            }
            impl<'a, T: 'a> ArrayAsMut<'a> for [T; $n]
            {
                type Output = [&'a mut T; $n];
                fn as_mut(array: &'a mut Self) -> Self::Output {
                    let [$($i,)*] = array;
                    [$($i,)*]
                }
            }

            impl_push_pop!($n; $fn_trait => $($i)* / $($j)*);
        )
    }

    macro_rules! implement {
        ($macro_name: ident) => {
            // <https://play.rust-lang.org/?gist=10a054305dfabf05f0c652e2df75fdcc>
            $macro_name!(0; FnOnce => /);
            $macro_name!(1; FnOnce => a0 / b0);
            $macro_name!(2; FnMut => a0 a1 / b0 b1);
            $macro_name!(3; FnMut => a0 a1 a2 / b0 b1 b2);
            $macro_name!(4; FnMut => a0 a1 a2 a3 / b0 b1 b2 b3);
            $macro_name!(5; FnMut => a0 a1 a2 a3 a4 / b0 b1 b2 b3 b4);
            $macro_name!(6; FnMut => a0 a1 a2 a3 a4 a5 / b0 b1 b2 b3 b4 b5);
            $macro_name!(7; FnMut => a0 a1 a2 a3 a4 a5 a6 / b0 b1 b2 b3 b4 b5 b6);
            $macro_name!(8; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 / b0 b1 b2 b3 b4 b5 b6 b7);
            $macro_name!(9; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 / b0 b1 b2 b3 b4 b5 b6 b7 b8);
            $macro_name!(10; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9);
            $macro_name!(11; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10);
            $macro_name!(12; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11);
            $macro_name!(13; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12);
            $macro_name!(14; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13);
            $macro_name!(15; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14);
            $macro_name!(16; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15);
            $macro_name!(17; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16);
            $macro_name!(18; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17);
            $macro_name!(19; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18);
            $macro_name!(20; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19);
            $macro_name!(21; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20);
            $macro_name!(22; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21);
            $macro_name!(23; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22);
            $macro_name!(24; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23);
            $macro_name!(25; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24);
            $macro_name!(26; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25);
            $macro_name!(27; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26);
            $macro_name!(28; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27);
            $macro_name!(29; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28);
            $macro_name!(30; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29);
            $macro_name!(31; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 a30 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30);
            $macro_name!(32; FnMut => a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 a19 a20 a21 a22 a23 a24 a25 a26 a27 a28 a29 a30 a31 / b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16 b17 b18 b19 b20 b21 b22 b23 b24 b25 b26 b27 b28 b29 b30 b31);
        };
    }

    #[cfg(not(feature = "const-generics"))]
    mod normal {
        use super::*;

        implement!(impl_for_size);
    }
    #[cfg(not(feature = "const-generics"))]
    use normal::*;

    #[cfg(feature = "const-generics")]
    mod const_generics {
        use super::*;

        impl<T, const N: usize> Sealed for [T; N] {}
        impl<T, const N: usize> ArrayTools for [T; N] {
            type Element = T;
            const LEN: usize = N;

            fn as_slice(&self) -> &[Self::Element] { self }
            fn as_mut_slice(&mut self) -> &mut [Self::Element] { self }
        }

        impl<T, F, const N: usize> ArrayGenerate<F> for [T; N]
            where F: FnMut() -> T {
            fn generate(mut f: F) -> Self {
                array_init::array_init(|_| f())
            }
        }

        impl<T, const N: usize> ArrayRepeat<T> for [T; N]
            where T: Clone
        {
            fn repeat(x: T) -> Self {
                array_init::array_init(|_| x.clone())
            }
        }

        impl<T, I, const N: usize> ArrayFromIter<I> for [T; N]
            where I: Iterator<Item = T>
        {
            fn from_iter(mut it: I) -> Option<Self> {
                array_init::from_iter(it)
            }
        }

        impl<const N: usize> ArrayIndices for [usize; N] {
            fn indices() -> Self {
                let mut i = 0;
                ArrayTools::generate(|| { let t = i; i += 1; t })
            }
        }

        const EQUAL_SIZE_ERROR_MESSAGE_ASSERTION: &str =
            "can't fail because the sizes of input and output are equal as ensured by the type system.";

        impl<T, U, F, const N: usize> ArrayMap<F> for [T; N]
            where F: FnMut(T) -> U {

            type Output = [U; N];
            type OutputElement = U;

            fn map(array: Self, mut f: F) -> Self::Output {
                let mut items = core::array::IntoIter::new(array);
                array_init::array_init(|_| f(items.next()
                    .expect(EQUAL_SIZE_ERROR_MESSAGE_ASSERTION))
                )
            }
        }

        impl<T, U, const N: usize> ArrayZip<[U; N]> for [T; N] {
            type Output = [(T, U); N];
            fn zip(array: Self, other: [U; N]) -> Self::Output {
                let mut items = core::array::IntoIter::new(array);
                let mut other_items = core::array::IntoIter::new(other);


                array_init::array_init(|_|
                    (
                        items.next().expect(EQUAL_SIZE_ERROR_MESSAGE_ASSERTION),
                        other_items.next().expect(EQUAL_SIZE_ERROR_MESSAGE_ASSERTION),
                    )
                )
            }
        }

        impl<T, U, V, F, const N: usize> ArrayZipWith<[U; N], F> for [T; N]
            where F: FnMut(T, U) -> V
        {
            type Output = [V; N];
            fn zip_with(array: Self, other: [U; N], mut f: F) -> Self::Output {
                let mut items = core::array::IntoIter::new(array);
                let mut other_items = core::array::IntoIter::new(other);


                array_init::array_init(|_|
                    f(
                        items.next().expect(EQUAL_SIZE_ERROR_MESSAGE_ASSERTION),
                        other_items.next().expect(EQUAL_SIZE_ERROR_MESSAGE_ASSERTION),
                    )
                )
            }
        }

        impl<'a, T: 'a, const N: usize> ArrayAsRef<'a> for [T; N] {
            type Output = [&'a T; N];

            fn as_ref(array: &'a Self) -> Self::Output {
                array_init::array_init(|i| &array[i])
            }
        }

        impl<'a, T: 'a, const N: usize> ArrayAsMut<'a> for [T; N] {
            type Output = [&'a mut T; N];

            fn as_mut(array: &'a mut Self) -> Self::Output {
                let mut items = array.iter_mut();

                array_init::array_init(|i| items.next().expect(EQUAL_SIZE_ERROR_MESSAGE_ASSERTION))
            }
        }

        implement!(impl_tuple);
        implement!(impl_push_pop);
    }
    #[cfg(feature = "const-generics")]
    use const_generics::*;
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
        assert_eq!(a[2].capacity(), 1);

        let sums = [1, 2, 3].zip_with([30, 20, 10], std::ops::Add::add);
        assert_eq!(sums, [31, 22, 13]);
    }

    #[test]
    fn from_iter_is_not_ambiguous_with_std() {
        #[allow(unused_imports)]
        use std::iter::FromIterator;
        <[i16; 5]>::from_iter(1..);
    }
}

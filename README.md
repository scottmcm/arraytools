# arraytools

A variety of helpful methods for working with fixed-size arrays.

[![docs.rs-hosted documentation](https://docs.rs/arraytools/badge.svg)](https://docs.rs/arraytools)
[![travis build status](https://travis-ci.com/scottmcm/arraytools.svg)](https://travis-ci.com/scottmcm/arraytools)
[![crates.io latest version](https://meritbadge.herokuapp.com/arraytools)](https://crates.io/crates/arraytools)

## Examples

`Iterator`-like methods over arrays:

```rust
use arraytools::ArrayTools;

assert_eq!([1, 2, 3].map(|x| x+1), [2, 3, 4]);
assert_eq!([1, 2].zip(["one", "two"]), [(1, "one"), (2, "two")]);
```

Ways to simplify array creation:

```rust
use arraytools::ArrayTools;

let mut state = 1;
assert_eq!(<[_; 4]>::generate(|| { state *= 2; state }), [2, 4, 8, 16]);
assert_eq!(<[usize; 4]>::indices(), [0, 1, 2, 3]);

let s = "hello".to_string(); // Something `!Copy`
assert_eq!(<[String; 3]>::repeat(s).as_ref_array(), ["hello", "hello", "hello"]);
```

Conversion to and from homogeneous tuples:

```rust
use arraytools::ArrayTools;

let mut array = [2, 3, 5, 7, 11];
assert_eq!(array.into_tuple(), (2, 3, 5, 7, 11));
array = ArrayTools::from_tuple((1, 1, 2, 3, 5));
assert_eq!(array, [1, 1, 2, 3, 5]);
```

## Usage

How to use with cargo:

```toml
[dependencies]
arraytools = "0.2"
```

How to use in your 2018-edition crate:

```rust
use arraytools::ArrayTools;
```

Because this needs non-`Copy` slice patterns, it needs at least **Rust 1.31.0**.

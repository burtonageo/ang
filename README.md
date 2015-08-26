# angular [![Build Status](https://travis-ci.org/b52/angular-rust.svg?branch=master)](https://travis-ci.org/b52/angular-rust)

Angular types and common helper methods to work with them while using the [Rust]
type system in our favor.

## Features

* Tested and documented
* Safe while don't sacrificing performance and conversion related errors

## Usage

Everything evolves around the `Angle<T>` type and its two variants
`Degrees(v)` and `Radians(v)`. This ensures proper typing and allows for safe
helper methods evolving around it.

```rust
extern crate angular;

use angular::*;
use std::f64::consts::{SQRT_2, FRAC_PI_4};

fn calc_hypotenuse(opposite: f64, alpha: Angle) -> f64 {
    opposite / alpha.sin()
}

let a = calc_hypotenuse(SQRT_2, Radians(FRAC_PI_4));
let b = calc_hypotenuse(SQRT_2, Degrees(45.0));
let c = calc_hypotenuse(SQRT_2, Angle::eighth());

assert!((a - 2.0).abs() < 1.0e10);
assert!((a - b).abs() < 1.0e10);
```

## Documentation

For an exhaustive documentation head over to the [API docs].

## License

This software is licensed under the terms of the MIT license. Please see the
[LICENSE](LICENSE) for full details.

[Rust]: http://www.rust-lang.org/
[API docs]: https://b52.github.io/angular-rust

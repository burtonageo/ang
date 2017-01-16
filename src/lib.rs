extern crate num;

#[cfg(test)] extern crate hamcrest;
#[cfg(test)] extern crate quickcheck;


use num::{Float, Num, NumCast, Signed, Zero, cast};
use std::f64::consts::PI;
use std::fmt::{Display, Formatter, Error};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};


/// An angle.
///
/// Might be a value in degrees or in radians.
#[derive(Copy, Clone, Debug)]
pub enum Angle<T=f64> {
    Radians(T),
    Degrees(T)
}

impl<T: NumCast> Angle<T> {
    /// Yield the value encoded in radians.
    pub fn in_radians(self) -> T {
        match self {
            Radians(v) => v,
            Degrees(v) => cast(cast::<T, f64>(v).unwrap() / 180.0 * PI).unwrap()
        }
    }

    /// Yield the value encoded in degrees.
    pub fn in_degrees(self) -> T {
        match self {
            Radians(v) => cast(cast::<T, f64>(v).unwrap() / PI * 180.0).unwrap(),
            Degrees(v) => v
        }
    }

    /// An angle of 45°.
    pub fn eighth() -> Angle<T> {
        Degrees(cast(45).unwrap())
    }

    /// An angle of 90° (right angle).
    pub fn quarter() -> Angle<T> {
        Degrees(cast(90).unwrap())
    }

    /// An angle of 180° (straight).
    pub fn half() -> Angle<T> {
        Degrees(cast(180).unwrap())
    }

    /// An angle of 360° (perigon).
    pub fn full() -> Angle<T> {
        Degrees(cast(360).unwrap())
    }
}

impl<T: Copy + Num + NumCast + PartialOrd> Angle<T> {
    /// Create a new angle by normalizing the value into the range of
    /// [0, 2π) rad.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use angular::*;
    /// # use std::f64::consts::PI;
    /// let alpha = Degrees(-90.0f64).normalized();
    /// assert!((alpha.in_degrees() - 270.0).abs() < 1.0e-10);
    ///
    /// let beta = Radians(2.0 * PI).normalized();
    /// assert!((beta.in_radians() - 0.0).abs() < 1.0e-10);
    /// ```
    pub fn normalized(self) -> Self {
        let (v, upper) = match self {
            Radians(v) => (v, cast(2.0 * PI).unwrap()),
            Degrees(v) => (v, cast(360.0).unwrap())
        };

        let normalized = if v < upper && v >= Zero::zero() {
            v
        } else {
            let v = v % upper;

            if v >= Zero::zero() {
                v
            } else {
                v + upper
            }
        };

        match self {
            Radians(_) => Radians(normalized),
            Degrees(_) => Degrees(normalized)
        }
    }
}

impl<T: Float> Angle<T> {
    /// Computes the minimal unsigned distance between two normalized angles. Returns an
    /// angle in the range of [0, π] rad.
    ///
    /// ```rust
    /// # use angular::*;
    /// let distance = Degrees(345.0).min_dist(Degrees(15.0));
    /// assert!((distance.in_degrees() - 30.0) < 1.0e-10);
    /// ```
    pub fn min_dist(self, other: Angle<T>) -> Angle<T> {
        let pi = cast(PI).unwrap();
        let two_pi = cast(2.0 * PI).unwrap();

        let a = self.in_radians();
        let b = other.in_radians();

        let d = (a - b).abs();

        // short-circout if both angles are normalized
        Radians(if a >= T::zero() && a < two_pi && b >= T::zero() && b < two_pi {
            d.min(two_pi - d)
        } else {
            pi - ((d % two_pi) - pi).abs()
        })
    }
}

impl<T: Signed> Angle<T> {
    /// Compute the absolute angle.
    pub fn abs(self) -> Self {
        match self {
            Radians(v) => Radians(v.abs()),
            Degrees(v) => Degrees(v.abs())
        }
    }
}

impl<T: Float + NumCast> Angle<T> {
    /// Compute the sine of the angle.
    pub fn sin(self) -> T {
        self.in_radians().sin()
    }

    /// Compute the cosine of the angle.
    pub fn cos(self) -> T {
        self.in_radians().cos()
    }

    /// Compute the tangent of the angle.
    pub fn tan(self) -> T {
        self.in_radians().tan()
    }

    /// Simultaneously compute the sine and cosine of the number, `x`.
    /// Return `(sin(x), cos(x))`.
    pub fn sin_cos(self) -> (T, T) {
        self.in_radians().sin_cos()
    }
}

impl<T: Zero + Copy + NumCast> Zero for Angle<T> {
    fn zero() -> Self {
        Radians(T::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            &Radians(ref v) => v.is_zero(),
            &Degrees(ref v) => v.is_zero()
        }
    }
}

impl<T: PartialEq + Copy + NumCast> PartialEq for Angle<T> {
    fn eq(&self, other: &Angle<T>) -> bool {
        if let (Degrees(a), Degrees(b)) = (*self, *other) {
            a == b
        } else {
            self.in_radians() == other.in_radians()
        }
    }
}

macro_rules! math_additive(
    ($bound:ident, $func:ident, $assign_bound:ident, $assign_func:ident) => (
        impl<T: $bound + Copy + NumCast> $bound for Angle<T> {
            type Output = Angle<T::Output>;
            fn $func(self, rhs: Angle<T>) -> Self::Output {
                if let (Degrees(a), Degrees(b)) = (self, rhs) {
                    Degrees(a.$func(b))
                } else {
                    Radians(self.in_radians().$func(rhs.in_radians()))
                }
            }
        }

        impl<T: $assign_bound + Copy + NumCast  > $assign_bound for Angle<T> {
            fn $assign_func(&mut self, rhs: Angle<T>) {
                if let (Degrees(ref mut a), Degrees(b)) = (*self, rhs)  {
                    a.$assign_func(b);
                    *self = Degrees(*a);
                } else {
                    let mut val = self.in_radians();
                    val.$assign_func(rhs.in_radians());
                    *self = Radians(val);
                }
            }
        }
    );
);

math_additive!(Add, add, AddAssign, add_assign);
math_additive!(Sub, sub, SubAssign, sub_assign);

macro_rules! math_multiplicative(
    ($bound:ident, $func:ident, $assign_bound:ident, $assign_func:ident, $($t:ident),*) => (
        impl<T: $bound> $bound<T> for Angle<T> {
            type Output = Angle<T::Output>;
            fn $func(self, rhs: T) -> Self::Output {
                match self {
                    Radians(v) => Radians(v.$func(rhs)),
                    Degrees(v) => Degrees(v.$func(rhs))
                }
            }
        }

        impl<T: $assign_bound> $assign_bound<T> for Angle<T> {
            fn $assign_func(&mut self, rhs: T) {
                match *self {
                    Radians(ref mut v) => { v.$assign_func(rhs) }
                    Degrees(ref mut v) => { v.$assign_func(rhs) }
                }
            }
        }

        $(
            impl $bound<Angle<$t>> for $t {
                type Output = Angle<$t>;
                fn $func(self, rhs: Angle<$t>) -> Self::Output {
                    match rhs {
                        Radians(v) => Radians(self.$func(v)),
                        Degrees(v) => Degrees(self.$func(v))
                    }
                }
            }
        )*
    );
);

math_multiplicative!(Mul, mul, MulAssign, mul_assign, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64);
math_multiplicative!(Div, div, DivAssign, div_assign, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64);

impl<T: Neg> Neg for Angle<T> {
    type Output = Angle<T::Output>;
    fn neg(self) -> Self::Output {
        match self {
            Radians(v) => Radians(-v),
            Degrees(v) => Degrees(-v)
        }
    }
}

impl<T: Display> Display for Angle<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Radians(ref v) => write!(f, "{}rad", v),
            Degrees(ref v) => write!(f, "{}°", v)
        }
    }
}

unsafe impl<T: Send> Send for Angle<T> {  }


/// Compute the arcsine of a number. Return value is in the range of
/// [-π/2, π/2] rad or `None` if the number is outside the range [-1, 1].
pub fn asin<T: Float>(value: T) -> Option<Angle<T>> {
    let value = value.asin();
    if value.is_nan() {
        None
    } else {
        Some(Radians(value))
    }
}

/// Compute the arccosine of a number. Return value is in the range of
/// [0, π] rad or `None` if the number is outside the range [-1, 1].
pub fn acos<T: Float>(value: T) -> Option<Angle<T>> {
    let value = value.acos();
    if value.is_nan() {
        None
    } else {
        Some(Radians(value))
    }
}

/// Compute the arctangent of a number. Return value is in the range of
/// [-π/2, π/2] rad.
pub fn atan<T: Float>(value: T) -> Angle<T> {
    Radians(value.atan())
}

/// Compute the four quadrant arctangent of `y` and `x`.
pub fn atan2<T: Float>(y: T, x: T) -> Angle<T> {
    Radians(y.atan2(x))
}

/// Compute the approximate mean of a list of angles by averaging the
/// Cartesian coordinates of the angles on the unit circle. Return the
/// normalized angle.
///
/// # Examples
///
/// ```rust
/// # use angular::*;
/// let angles = [Degrees(270.0f64), Degrees(360.0), Degrees(90.0)];
///
/// let mu = mean_angle(&angles);
/// assert!(mu.min_dist(Radians(0.0)).in_radians() < 1.0e-10);
/// ```
pub fn mean_angle<'a, T, I>(angles: I) -> Angle<T>
    where T: 'a + Float, I: IntoIterator<Item=&'a Angle<T>>
{
    let mut x = T::zero();
    let mut y = T::zero();
    let mut n = 0;

    for angle in angles {
        let (sin, cos) = angle.sin_cos();

        x = x + cos;
        y = y + sin;
        n += 1;
    }

    let n = cast(n).unwrap();
    let a = (y / n).atan2(x / n);

    Radians(a).normalized()
}


// re-exports
pub use Angle::{Radians, Degrees};


#[cfg(test)]
mod tests {
    use hamcrest::{assert_that, is, close_to};
    use num::{Float, cast};
    use quickcheck::{Arbitrary, Gen, quickcheck};
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_angle_conversions() {
        fn prop(angle: Angle) -> bool {
            are_close(angle.in_radians(), Degrees(angle.in_degrees()).in_radians())
        }
        quickcheck(prop as fn(Angle) -> bool);
    }

    #[test]
    fn test_angle_math_multiplicative() {
        fn prop(a: Angle, x: f64) -> bool {
            match a {
                Radians(v) => {
                    let div_res= {
                        let mut a1 = a.clone();
                        a1 /= x;
                        a1.in_radians() == v / x
                    };
                    let mult_res = {
                        let mut a1 = a.clone();
                        a1 *= x;
                        a1.in_radians() == v * x
                    };
                    (a * x).in_radians() == v * x &&
                    (a / x).in_radians() == v / x &&
                    div_res &&
                    mult_res
                }
                Degrees(v) => {
                    let div_res= {
                        let mut a1 = a.clone();
                        a1 *= x;
                        a1.in_degrees() == v * x
                    };
                    let mult_res = {
                        let mut a1 = a.clone();
                        a1 /= x;
                        a1.in_degrees() == v / x
                    };
                    (a * x).in_degrees() == v * x &&
                    (a / x).in_degrees() == v / x &&
                    div_res &&
                    mult_res
                }
            }
        }
        quickcheck(prop as fn(Angle, f64) -> bool);
    }

    #[test]
    fn test_angle_math_additive() {
        fn prop(a: Angle, b: Angle) -> bool {
            if let (Radians(x), Radians(y)) = (a, b) {
                let add_res = {
                    let mut a1 = a.clone();
                    a1 += b;
                    a1.in_radians() == x + y
                };
                let sub_res = {
                    let mut a1 = a.clone();
                    a1 -= b;
                    a1.in_radians() == x - y
                };
                (a + b).in_radians() == x + y &&
                (a - b).in_radians() == x - y &&
                add_res &&
                sub_res
            } else if let (Degrees(x), Degrees(y)) = (a, b) {
                let add_res = {
                    let mut a1 = a.clone();
                    a1 += b;
                    a1.in_degrees() == x + y
                };
                let sub_res = {
                    let mut a1 = a.clone();
                    a1 -= b;
                    a1.in_degrees() == x - y
                };
                (a + b).in_degrees() == x + y &&
                (a - b).in_degrees() == x - y &&
                add_res &&
                sub_res
            } else {
                let add_res = {
                    let mut a1 = a.clone();
                    a1 += b;
                    a1.in_radians() == a.in_radians() + b.in_radians()
                };
                let sub_res = {
                    let mut a1 = a.clone();
                    a1 -= b;
                    a1.in_radians() == a.in_radians() - b.in_radians()
                };
                (a + b).in_radians() == a.in_radians() + b.in_radians() && add_res && sub_res
            }
        }
        quickcheck(prop as fn(Angle, Angle) -> bool);
    }

    #[test]
    fn test_angle_normalization() {
        fn prop(angle: Angle) -> bool {
            let v = angle.normalized();
            let rad = v.in_radians();
            let deg = v.in_degrees();

            0.0 <= rad && rad < 2.0 * PI &&
            0.0 <= deg && deg < 360.0 &&
            are_close(rad.cos(), angle.cos())
        }
        quickcheck(prop as fn(Angle) -> bool);
    }

    #[test]
    fn test_angle_minimal_distance() {
        fn prop(a: Angle, b: Angle) -> bool {
            let d = a.min_dist(b);
            0.0 <= d.in_radians() && d.in_radians() <= PI
        }
        quickcheck(prop as fn(Angle, Angle) -> bool);

        assert_that(Degrees(180.0).min_dist(Degrees(0.0)).in_degrees(), is(close_to(180.0, 0.000001)));
        assert_that(Degrees(0.1).min_dist(Degrees(359.9)).in_degrees(), is(close_to(0.2, 0.000001)));
        assert_that(Degrees(1.0).min_dist(Degrees(2.0)).in_degrees(), is(close_to(1.0, 0.000001)));
    }

    #[test]
    pub fn test_mean_angle() {
        assert_that(mean_angle(&[Degrees(90.0)]).in_degrees(), is(close_to(90.0, 0.000001)));
        assert_that(mean_angle(&[Degrees(90.0), Degrees(90.0)]).in_degrees(), is(close_to(90.0, 0.000001)));
        assert_that(mean_angle(&[Degrees(90.0), Degrees(180.0), Degrees(270.0)]).in_degrees(), is(close_to(180.0, 0.000001)));
        assert_that(mean_angle(&[Degrees(20.0), Degrees(350.0)]).in_degrees(), is(close_to(5.0, 0.000001)));
    }

    fn are_close<T: Float>(a: T, b: T) -> bool {
        (a - b).abs() < cast(1.0e-10).unwrap()
    }

    impl<T: Arbitrary> Arbitrary for Angle<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let v = Arbitrary::arbitrary(g);
            if bool::arbitrary(g) {
                Radians(v)
            } else {
                Degrees(v)
            }
        }
    }
}

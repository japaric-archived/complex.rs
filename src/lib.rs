//! Complex numbers

#![deny(missing_docs, warnings)]
#![feature(macro_rules, associated_types, default_type_params)]

extern crate onezero;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::rand::{Rand, Rng};

use onezero::{One, Zero};

mod ffi;
mod math;

pub mod f32;
pub mod f64;

/// A complex number in Cartesian form.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex<T> {
    /// The real part
    pub re: T,
    /// The imaginary part
    pub im: T,
}

/// Single precision complex number
#[allow(non_camel_case_types)]
pub type c64 = Complex<f32>;
/// Double precision complex number
#[allow(non_camel_case_types)]
pub type c128 = Complex<f64>;

impl<T> Complex<T> {
    /// Create a new complex number
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex {
            im: im,
            re: re,
        }
    }
}

impl<T> Complex<T> where T: Add<Output=T> + Clone + Mul<Output=T> {
    fn norm_sqr(self) -> T {
        self.re.clone() * self.re + self.im.clone() * self.im
    }
}

impl<T> Add<T> for Complex<T> where T: Add<Output=T> + Clone {
    type Output = Complex<T>;
    fn add(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re + rhs,
            im: self.im.clone(),
        }
    }
}

impl<T> Add for Complex<T> where T: Add<Output=T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Add<Complex<T>> for T where T: Add<Output=T> + Clone {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        rhs + self
    }
}

impl<T> Div<T> for Complex<T> where T: Clone + Div<Output=T> {
    type Output = Complex<T>;
    fn div(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re / rhs.clone(),
            im: self.im / rhs,
        }
    }
}

impl<T> Div for Complex<T> where
    T: Add<Output=T> + Clone + Div<Output=T> + Mul<Output=T> + Sub<Output=T>
{
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let den = rhs.clone().norm_sqr();

        Complex {
            re: {
                self.re.clone() * rhs.re.clone() + self.im.clone() * rhs.im.clone()
            } / den.clone(),
            im: (self.im * rhs.re - self.re * rhs.im) / den,
        }
    }
}

impl<T> Div<Complex<T>> for T where
    T: Add<Output=T> + Clone + Div<Output=T> + Mul<Output=T> + Neg<Output=T>,
{
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let den = rhs.clone().norm_sqr();

        Complex {
            re: (self.clone() * rhs.re) / den.clone(),
            im: -(self * rhs.im) / den,
        }
    }
}

impl<T> Mul<T> for Complex<T> where T: Clone + Mul<Output=T> {
    type Output = Complex<T>;
    fn mul(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re * rhs.clone(),
            im: self.im * rhs,
        }
    }
}

impl<T> Mul for Complex<T> where
    T: Add<Output=T> + Clone + Mul<Output=T> + Sub<Output=T>,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> Mul<Complex<T>> for T where T: Clone + Mul<Output=T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        rhs * self
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> One for Complex<T> where T: Mul<Output=T> + One + Sub<Output=T> + Zero {
    fn one() -> Complex<T> {
        Complex {
            re: One::one(),
            im: Zero::zero(),
        }
    }
}

impl<T> Rand for Complex<T> where T: Rand {
    fn rand<R>(rng: &mut R) -> Complex<T> where R: Rng {
        Complex::new(rng.gen(), rng.gen())
    }
}

impl<T> fmt::Show for Complex<T> where T: PartialOrd + fmt::Show + Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < Zero::zero() {
            write!(f, "{}-{}i", self.re, self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

impl<T> Sub<T> for Complex<T> where T: Clone + Sub<Output=T> {
    type Output = Complex<T>;
    fn sub(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re - rhs,
            im: self.im.clone(),
        }
    }
}

impl<T> Sub for Complex<T> where T: Sub<Output=T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> Sub<Complex<T>> for T where T: Neg<Output=T> + Sub<Output=T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self - rhs.re,
            im: -rhs.im,
        }
    }
}

impl<T> Zero for Complex<T> where T: Zero {
    fn zero() -> Complex<T> {
        Complex {
            re: Zero::zero(),
            im: Zero::zero(),
        }
    }
}

/// Mathematical operations on complex numbers
// FIXME (AI) `T` should be an associated output type
pub trait Math<T> {
    /// Computes the complex absolute value (also called norm, modulus or magnitude)
    fn abs(self) -> T;

    /// Computes the arc cosine
    fn acos(self) -> Self;

    /// Computes the arc hyperbolic cosine
    fn acosh(self) -> Self;

    /// Computes the argument (also called the phase angle)
    fn arg(self) -> T;

    /// Computes the arc sine
    fn asin(self) -> Self;

    /// Computes the arc hyperbolic sine
    fn asinh(self) -> Self;

    /// Computes the arc tangent
    fn atan(self) -> Self;

    /// Computes the arc hyperbolic tangent
    fn atanh(self) -> Self;

    /// Computes the arc cosine
    fn cos(self) -> Self;

    /// Computes the arc hyperbolic cosine
    fn cosh(self) -> Self;

    /// Computes the complex base-e exponential
    fn exp(self) -> Self;

    /// Returns the imaginary part
    fn imag(self) -> T;

    /// Computes the complex base-e logarithm
    fn log(self) -> Self;

    /// Computes the complex conjugate
    fn conj(self) -> Self;

    /// Computes the complex power
    fn pow(self, Self) -> Self;

    /// Computes the Riemann sphere projection
    fn proj(self) -> Self;

    /// Returns the real part
    fn real(self) -> T;

    /// Computes the arc sine
    fn sin(self) -> Self;

    /// Computes the arc hyperbolic sine
    fn sinh(self) -> Self;

    /// Computes the square root
    fn sqrt(self) -> Self;

    /// Computes the arc tangent
    fn tan(self) -> Self;

    /// Computes the arc hyperbolic tangent
    fn tanh(self) -> Self;
}

#[cfg(test)]
mod test {
    macro_rules! test {
        ($($ty:ident),+) => {$(
            mod $ty {
                use Complex;
                use $ty::I;

                #[test]
                fn constructor() {
                    assert_eq!(Complex::new(3., 4.), I * 4. + 3.)
                }
            })+
         }
    }

    test!(f32, f64);
}

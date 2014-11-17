//! Complex numbers

#![deny(missing_docs, warnings)]
#![feature(macro_rules)]

extern crate onezero;

use std::fmt;
use std::rand::{Rand, Rng};

use onezero::{One, Zero};

mod ffi;
mod math;

pub mod f32;
pub mod f64;

/// A complex number in Cartesian form.
#[repr(C)]
#[deriving(PartialEq)]
pub struct Complex<T> {
    /// The real part
    pub re: T,
    /// The imaginary part
    pub im: T,
}

#[allow(non_camel_case_types)]
pub type c64 = Complex<f32>;
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

impl<T> Complex<T> where T: Add<T, T> + Mul<T, T> {
    /// Returns the square of the norm
    fn norm_sqr(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T> Add<T, Complex<T>> for Complex<T> where T: Add<T, T> + Clone {
    fn add(&self, rhs: &T) -> Complex<T> {
        Complex {
            re: self.re.add(rhs),
            im: self.im.clone(),
        }
    }
}

impl<T> Add<Complex<T>, Complex<T>> for Complex<T> where T: Add<T, T> {
    fn add(&self, rhs: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Add<Complex<T>, Complex<T>> for T where T: Add<T, T> + Clone {
    fn add(&self, rhs: &Complex<T>) -> Complex<T> {
        rhs.add(self)
    }
}

impl<T> Div<T, Complex<T>> for Complex<T> where T: Div<T, T> {
    fn div(&self, rhs: &T) -> Complex<T> {
        Complex {
            re: self.re.div(rhs),
            im: self.im.div(rhs),
        }
    }
}

impl<T> Div<Complex<T>, Complex<T>> for Complex<T> where
    T: Add<T, T> + Div<T, T> + Mul<T, T> + Sub<T, T>
{
    fn div(&self, rhs: &Complex<T>) -> Complex<T> {
        let den = rhs.norm_sqr();

        Complex {
            re: (self.re * rhs.re + self.im + rhs.im) / den,
            im: (self.im * rhs.re - self.re * rhs.im) / den,
        }
    }
}

impl<T> Div<Complex<T>, Complex<T>> for T where T: Add<T, T> + Div<T, T> + Mul<T, T> + Neg<T> {
    fn div(&self, rhs: &Complex<T>) -> Complex<T> {
        let den = rhs.norm_sqr();

        Complex {
            re: (self.mul(&rhs.re)) / den,
            im: -(self.mul(&rhs.im)) / den,
        }
    }
}

impl<T> Mul<T, Complex<T>> for Complex<T> where T: Mul<T, T> {
    fn mul(&self, rhs: &T) -> Complex<T> {
        Complex {
            re: self.re.mul(rhs),
            im: self.im.mul(rhs),
        }
    }
}

impl<T> Mul<Complex<T>, Complex<T>> for Complex<T> where T: Add<T, T> + Mul<T, T> + Sub<T, T> {
    fn mul(&self, rhs: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> Mul<Complex<T>, Complex<T>> for T where T: Mul<T, T> {
    fn mul(&self, rhs: &Complex<T>) -> Complex<T> {
        rhs.mul(self)
    }
}

impl<T> Neg<Complex<T>> for Complex<T> where T: Neg<T> {
    fn neg(&self) -> Complex<T> {
        Complex {
            re: self.re.neg(),
            im: self.im.neg(),
        }
    }
}

impl<T> One for Complex<T> where T: Mul<T, T> + One + Sub<T, T> + Zero {
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

impl<T> Sub<T, Complex<T>> for Complex<T> where T: Clone + Sub<T, T> {
    fn sub(&self, rhs: &T) -> Complex<T> {
        Complex {
            re: self.re.sub(rhs),
            im: self.im.clone(),
        }
    }
}

impl<T> Sub<Complex<T>, Complex<T>> for Complex<T> where T: Sub<T, T> {
    fn sub(&self, rhs: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> Sub<Complex<T>, Complex<T>> for T where T: Neg<T> + Sub<T, T> {
    fn sub(&self, rhs: &Complex<T>) -> Complex<T> {
        Complex {
            re: self.sub(&rhs.re),
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

    test!(f32, f64)
}

//! Complex numbers

#![deny(missing_docs, warnings)]

extern crate onezero;

use std::fmt;
use std::num::FloatMath;
use std::rand::{Rand, Rng};

use onezero::{One, Zero};

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

impl<T> Complex<T> where T: Clone + Neg<T> {
    /// Returns the complex conjugate
    pub fn conj(&self) -> Complex<T> {
        Complex {
            re: self.re.clone(),
            im: -self.im,
        }
    }
}

impl<T> Complex<T> where T: Add<T, T> + Mul<T, T> {
    /// Returns the square of the norm
    pub fn norm_sqr(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T> Complex<T> where T: FloatMath {
    /// Returns the absolute value
    pub fn abs(&self) -> T {
        self.re.hypot(self.im)
    }

    /// Returns the argument
    pub fn arg(&self) -> T {
        self.im.atan2(self.re)
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

impl<T> Zero for Complex<T> where T: Zero {
    fn zero() -> Complex<T> {
        Complex {
            re: Zero::zero(),
            im: Zero::zero(),
        }
    }
}

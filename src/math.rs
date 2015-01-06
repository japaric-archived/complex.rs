use {Math, c64, c128, ffi};

impl Math<f32> for c64 {
    fn abs(self) -> f32 {
        unsafe { ffi::cabsf(self) }
    }

    fn acos(self) -> c64 {
        unsafe { ffi::cacosf(self) }
    }

    fn acosh(self) -> c64 {
        unsafe { ffi::cacoshf(self) }
    }

    fn arg(self) -> f32 {
        unsafe { ffi::cargf(self) }
    }

    fn asin(self) -> c64 {
        unsafe { ffi::casinf(self) }
    }

    fn asinh(self) -> c64 {
        unsafe { ffi::casinhf(self) }
    }

    fn atan(self) -> c64 {
        unsafe { ffi::catanf(self) }
    }

    fn atanh(self) -> c64 {
        unsafe { ffi::catanhf(self) }
    }

    fn cos(self) -> c64 {
        unsafe { ffi::ccosf(self) }
    }

    fn cosh(self) -> c64 {
        unsafe { ffi::ccoshf(self) }
    }

    fn exp(self) -> c64 {
        unsafe { ffi::cexpf(self) }
    }

    fn imag(self) -> f32 {
        unsafe { ffi::cimagf(self) }
    }

    fn log(self) -> c64 {
        unsafe { ffi::clogf(self) }
    }

    fn conj(self) -> c64 {
        unsafe { ffi::cconjf(self) }
    }

    fn pow(self, n: c64) -> c64 {
        unsafe { ffi::cpowf(self, n) }
    }

    fn proj(self) -> c64 {
        unsafe { ffi::cprojf(self) }
    }

    fn real(self) -> f32 {
        unsafe { ffi::crealf(self) }
    }

    fn sin(self) -> c64 {
        unsafe { ffi::csinf(self) }
    }

    fn sinh(self) -> c64 {
        unsafe { ffi::csinhf(self) }
    }

    fn sqrt(self) -> c64 {
        unsafe { ffi::csqrtf(self) }
    }

    fn tan(self) -> c64 {
        unsafe { ffi::ctanf(self) }
    }

    fn tanh(self) -> c64 {
        unsafe { ffi::ctanhf(self) }
    }
}

impl Math<f64> for c128 {
    fn abs(self) -> f64 {
        unsafe { ffi::cabs(self) }
    }

    fn acos(self) -> c128 {
        unsafe { ffi::cacos(self) }
    }

    fn acosh(self) -> c128 {
        unsafe { ffi::cacosh(self) }
    }

    fn arg(self) -> f64 {
        unsafe { ffi::carg(self) }
    }

    fn asin(self) -> c128 {
        unsafe { ffi::casin(self) }
    }

    fn asinh(self) -> c128 {
        unsafe { ffi::casinh(self) }
    }

    fn atan(self) -> c128 {
        unsafe { ffi::catan(self) }
    }

    fn atanh(self) -> c128 {
        unsafe { ffi::catanh(self) }
    }

    fn cos(self) -> c128 {
        unsafe { ffi::ccos(self) }
    }

    fn cosh(self) -> c128 {
        unsafe { ffi::ccosh(self) }
    }

    fn exp(self) -> c128 {
        unsafe { ffi::cexp(self) }
    }

    fn imag(self) -> f64 {
        unsafe { ffi::cimag(self) }
    }

    fn log(self) -> c128 {
        unsafe { ffi::clog(self) }
    }

    fn conj(self) -> c128 {
        unsafe { ffi::cconj(self) }
    }

    fn pow(self, n: c128) -> c128 {
        unsafe { ffi::cpow(self, n) }
    }

    fn proj(self) -> c128 {
        unsafe { ffi::cproj(self) }
    }

    fn real(self) -> f64 {
        unsafe { ffi::creal(self) }
    }

    fn sin(self) -> c128 {
        unsafe { ffi::csin(self) }
    }

    fn sinh(self) -> c128 {
        unsafe { ffi::csinh(self) }
    }

    fn sqrt(self) -> c128 {
        unsafe { ffi::csqrt(self) }
    }

    fn tan(self) -> c128 {
        unsafe { ffi::ctan(self) }
    }

    fn tanh(self) -> c128 {
        unsafe { ffi::ctanh(self) }
    }
}

#[cfg(test)]
mod test {
    macro_rules! test {
        ($($ty:ident),+) => {$(
            mod $ty {
                use std::num::Float;

                use Math;
                use $ty::I;

                #[test]
                fn abs() {
                    let z = I * 4. + 3.;
                    assert_eq!(z.abs(), 5.)
                }

                #[test]
                fn arg() {
                    let z = I + 1.;
                    assert_eq!(z.arg(), ::std::$ty::consts::FRAC_PI_4)
                }

                #[test]
                fn imag() {
                    let z = I + 2.;
                    assert_eq!(z.imag(), 1.)
                }

                #[test]
                fn real() {
                    let z = I + 2.;
                    assert_eq!(z.real(), 2.)
                }
            })+
         }
    }

    test!(f32, f64);
}

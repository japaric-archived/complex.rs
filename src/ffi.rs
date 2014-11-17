use {c64, c128};

#[link(name="m")]
extern {
    pub fn cabs(z: c128) -> f64;
    pub fn cabsf(z: c64) -> f32;

    pub fn cacos(z: c128) -> c128;
    pub fn cacosf(z: c64) -> c64;

    pub fn cacosh(z: c128) -> c128;
    pub fn cacoshf(z: c64) -> c64;

    pub fn carg(z: c128) -> f64;
    pub fn cargf(z: c64) -> f32;

    pub fn casin(z: c128) -> c128;
    pub fn casinf(z: c64) -> c64;

    pub fn casinh(z: c128) -> c128;
    pub fn casinhf(z: c64) -> c64;

    pub fn catan(z: c128) -> c128;
    pub fn catanf(z: c64) -> c64;

    pub fn catanh(z: c128) -> c128;
    pub fn catanhf(z: c64) -> c64;

    pub fn ccos(z: c128) -> c128;
    pub fn ccosf(z: c64) -> c64;

    pub fn ccosh(z: c128) -> c128;
    pub fn ccoshf(z: c64) -> c64;

    pub fn cexp(z: c128) -> c128;
    pub fn cexpf(z: c64) -> c64;

    pub fn cimag(z: c128) -> f64;
    pub fn cimagf(z: c64) -> f32;

    pub fn clog(z: c128) -> c128;
    pub fn clogf(z: c64) -> c64;

    pub fn cconj(z: c128) -> c128;
    pub fn cconjf(z: c64) -> c64;

    pub fn cpow(x: c128, y: c128) -> c128;
    pub fn cpowf(x: c64, y: c64) -> c64;

    pub fn cproj(z: c128) -> c128;
    pub fn cprojf(z: c64) -> c64;

    pub fn creal(z: c128) -> f64;
    pub fn crealf(z: c64) -> f32;

    pub fn csin(z: c128) -> c128;
    pub fn csinf(z: c64) -> c64;

    pub fn csinh(z: c128) -> c128;
    pub fn csinhf(z: c64) -> c64;

    pub fn csqrt(z: c128) -> c128;
    pub fn csqrtf(z: c64) -> c64;

    pub fn ctan(z: c128) -> c128;
    pub fn ctanf(z: c64) -> c64;

    pub fn ctanh(z: c128) -> c128;
    pub fn ctanhf(z: c64) -> c64;
}

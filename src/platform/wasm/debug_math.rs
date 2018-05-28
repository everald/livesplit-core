#[no_mangle]
pub extern "C" fn fmodf(a: f32, b: f32) -> f32 {
    a - (a / b).floor() * b
}

#[no_mangle]
pub extern "C" fn cos(x: f64) -> f64 {
    gen_cos(x)
}

#[no_mangle]
pub extern "C" fn sin(x: f64) -> f64 {
    gen_sin(x)
}

#[no_mangle]
pub extern "C" fn sinf(x: f32) -> f32 {
    gen_sin(x)
}

#[no_mangle]
pub extern "C" fn exp(p: f64) -> f64 {
    gen_exp(p)
}

#[no_mangle]
pub extern "C" fn expf(p: f32) -> f32 {
    gen_exp(p)
}

fn gen_exp<T: Flt>(p: T) -> T {
    T::ONE + expm1(p)
}

fn expm1_<T: Flt>(p: T, n: i16) -> T {
    let (mut pn, mut f, mut a) = (p, T::ONE, T::ZERO);
    for i in 1..=n {
        f = f * T::from(i);
        a = a + pn / f;
        pn = p * pn;
    }
    a
}

fn expm1<T: Flt>(p: T) -> T {
    expm1_(p, 15)
}

#[no_mangle]
pub extern "C" fn round(mut var0: f64) -> f64 {
    let var2: i64;
    let var3: i32;
    let var4: f64;
    let var5: f64;
    'label0: loop {
        var2 = var0.to_bits() as i64;
        var3 = (var2 as u64).wrapping_shr(52i64 as u32) as i64 as i32 & 2047i32;
        if (var3 as u32 > 1074i32 as u32) as i32 != 0 {
            break 'label0;
        }
        var4 = { let a = f64::from_bits(var0.to_bits() ^ 0x8000_0000_0000_0000); let b = var0; if (var2 < 0i64) as i32 != 0 { a } else { b } };
        'label1: loop {
            if (var3 as u32 > 1021i32 as u32) as i32 != 0 {
                break 'label1;
            }
            drop((var4 + f64::from_bits(0x4330000000000000)).to_bits());
            return var0 * f64::from_bits(0x0);
        }
        'label2: loop {
            'label3: loop {
                var5 = var4 + f64::from_bits(0x4330000000000000) + f64::from_bits(0xC330000000000000) - var4;
                if (var5 > f64::from_bits(0x3FE0000000000000)) as i32 ^ 1i32 != 0 {
                    break 'label3;
                }
                var0 = var4 + var5 + f64::from_bits(0xBFF0000000000000);
                break 'label2;
            }
            var0 = var4 + var5;
            if (var5 <= f64::from_bits(0xBFE0000000000000)) as i32 ^ 1i32 != 0 {
                break 'label2;
            }
            var0 = var0 + f64::from_bits(0x3FF0000000000000);
            break;
        }
        var0 = { let a = f64::from_bits(var0.to_bits() ^ 0x8000_0000_0000_0000); let b = var0; if (var2 < 0i64) as i32 != 0 { a } else { b } };
        break;
    }
    var0
}

#[no_mangle]
pub extern "C" fn roundf(mut var0: f32) -> f32 {
    let var2: i32;
    let var3: i32;
    let var4: f32;
    let var5: f32;
    'label0: loop {
        var2 = var0.to_bits() as i32;
        var3 = (var2 as u32).wrapping_shr(23i32 as u32) as i32 & 255i32;
        if (var3 as u32 > 149i32 as u32) as i32 != 0 {
            break 'label0;
        }
        var4 = { let a = f32::from_bits(var0.to_bits() ^ 0x8000_0000); let b = var0; if (var2 < 0i32) as i32 != 0 { a } else { b } };
        'label1: loop {
            if (var3 as u32 > 125i32 as u32) as i32 != 0 {
                break 'label1;
            }
            drop((var4 + f32::from_bits(0x4B000000)).to_bits());
            return var0 * f32::from_bits(0x0);
        }
        'label2: loop {
            'label3: loop {
                var5 = var4 + f32::from_bits(0x4B000000) + f32::from_bits(0xCB000000) - var4;
                if (var5 > f32::from_bits(0x3F000000)) as i32 ^ 1i32 != 0 {
                    break 'label3;
                }
                var0 = var4 + var5 + f32::from_bits(0xBF800000);
                break 'label2;
            }
            var0 = var4 + var5;
            if (var5 <= f32::from_bits(0xBF000000)) as i32 ^ 1i32 != 0 {
                break 'label2;
            }
            var0 = var0 + f32::from_bits(0x3F800000);
            break;
        }
        var0 = { let a = f32::from_bits(var0.to_bits() ^ 0x8000_0000); let b = var0; if (var2 < 0i32) as i32 != 0 { a } else { b } };
        break;
    }
    var0
}

#[no_mangle]
pub extern "C" fn tzset() {}

fn gen_sin<T: Flt>(x: T) -> T {
    x - powi(x, 3) / T::C6 + powi(x, 5) / T::C120 - powi(x, 7) / T::C5040 + powi(x, 9) / T::C362880
}

fn gen_cos<T: Flt>(x: T) -> T {
    gen_sin(T::PI2 - x)
}

fn powi<T: Flt>(f: T, p: i16) -> T {
    if p > 0 {
        let (mut f, mut p, mut a) = (f, p, T::ONE);
        while p != 0 {
            if p % 2 == 1 {
                a = a * f;
            }
            p /= 2;
            f = f * f;
        }
        a
    } else {
        T::ONE / powi(f, -p)
    }
}

use std::ops;

pub trait Flt:
    PartialEq
    + PartialOrd
    + ops::Neg<Output = Self>
    + Sized
    + Copy
    + ops::Sub<Output = Self>
    + ops::Rem<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Add<Output = Self>
    + From<i16>
{
    const ZERO: Self;
    const NAN: Self;
    const INF: Self;
    const NEG_INF: Self;
    const HALF: Self;
    const ONE: Self;
    const TWO: Self;
    const LN2: Self;
    const LN10: Self;
    const PI: Self;
    const PI2: Self;

    const C6: Self;
    const C120: Self;
    const C5040: Self;
    const C362880: Self;

    const C3: Self;
    const C2D15: Self;
    const C17D315: Self;
}

use std::{f32::{self as f, consts as fc},
           f64::{self as d, consts as dc}};

impl Flt for f32 {
    const ZERO: Self = 0.0;
    const NAN: Self = f::NAN;
    const INF: Self = f::INFINITY;
    const NEG_INF: Self = f::NEG_INFINITY;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const LN2: Self = fc::LN_2;
    const LN10: Self = fc::LN_10;
    const PI: Self = fc::PI;
    const PI2: Self = fc::FRAC_PI_2;

    const C6: Self = 6.0;
    const C120: Self = 120.0;
    const C5040: Self = 5040.0;
    const C362880: Self = 362880.0;
    const C3: Self = 3.0;
    const C2D15: Self = 2.0 / 15.0;
    const C17D315: Self = 17.0 / 315.0;
}

impl Flt for f64 {
    const ZERO: Self = 0.0;
    const NAN: Self = d::NAN;
    const INF: Self = d::INFINITY;
    const NEG_INF: Self = d::NEG_INFINITY;
    const HALF: Self = 0.5;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const LN2: Self = dc::LN_2;
    const LN10: Self = dc::LN_10;
    const PI: Self = dc::PI;
    const PI2: Self = dc::FRAC_PI_2;

    const C6: Self = 6.0;
    const C120: Self = 120.0;
    const C5040: Self = 5040.0;
    const C362880: Self = 362880.0;
    const C3: Self = 3.0;
    const C2D15: Self = 2.0 / 15.0;
    const C17D315: Self = 17.0 / 315.0;
}

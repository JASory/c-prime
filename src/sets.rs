use crate::ptrait::Primality;
use crate::iterators::Primes;

/// Set of all 128-bit Primes
pub const P : Primes<u128> = Primes::start(1u128);
/// Set of all 64-bit primes
pub const P64 : Primes<u64> = Primes::start(1u64);
/// Set of all 32-bit signed primes 
pub const P32 : Primes<u32> = Primes::start(1u32);
/// Set of all 16-bit primes
pub const P16 : Primes<u16> = Primes::start(1u16);
/// Set of all 8-bit primes
pub const P8 : Primes<u8> = Primes::start(1u8);
/// Set of all 128-bit signed primes
pub const P128I : Primes<i128> = Primes::start(i128::FIRST_PRIME);
/// Set of all 64-bit signed primes
pub const P64I : Primes<i64> = Primes::start(i64::FIRST_PRIME);
/// Set of all 32-bit signed primes 
pub const P32I : Primes<i32> = Primes::start(i32::FIRST_PRIME);
/// Set of all 16-bit signed primes
pub const P16I : Primes<i16> = Primes::start(i16::FIRST_PRIME);
/// Set of all 8-bit signed primes
pub const P8I : Primes<i8> = Primes::start(i8::FIRST_PRIME);
/// Set of all 53-bit primes representable in 64-bit floating-point
pub const P64F : Primes<f64> = Primes::start(f64::FIRST_PRIME);
/// Set of all 24-bit primes representable in 64-bit floating-point
pub const P32F : Primes<f32> = Primes::start(f32::FIRST_PRIME);


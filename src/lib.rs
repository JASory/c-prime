//! Convenience wrapper for machine-prime. Implements primality testing for all primitive types, including floats. 
//! Provides iterators and const functions, and modeling of prime sets. 
//! Primes here are defined as integers that can only be composed of themselves and a unit of the integers {1,-1}.
//! Therefore numbers with no factor P are primes as well as -P.
//! strong-case proves primality for composites with large factors with slightly less computation than is_prime.
//! ```
//!   use c_prime::{P,prime_array};
//!   
//!   assert_eq!(P.contains(17),true);
//!
//!   assert_eq!(P.take_while(|x| *x < 100).count(),25);
//!  // Set intersection of the primes and some collection 
//!   assert_eq!(P.intersection([2,4,5,6,7]),[2,5,7]);
//!  // The 25 primes greater than 100
//! const FIRST : [u128;25] = prime_array(100);
//! ```

//#![feature(generic_const_items)]

pub(crate) mod ptrait;
pub(crate) mod func;
pub(crate) mod prim;
pub(crate) mod sets;
pub(crate) mod iterators;


pub use crate::ptrait::Primality;
pub use crate::func::*;
pub use crate::iterators::*;
pub use crate::sets::*;

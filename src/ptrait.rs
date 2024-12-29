/// Trait for primality over all primitive types
pub trait Primality: Clone + Copy + std::ops::AddAssign + std::ops::SubAssign + std::cmp::PartialOrd{
  /// Unit of the Set
  const ONE : Self;
  
  /// Minimum prime in the set
  const FIRST_PRIME : Self;
  
  /// Maximum prime in the set
  const LAST_PRIME : Self ;
  
  /// Returns true if prime
  fn is_prime(&self) -> bool;
  
  /// Minimal computation for primality proving
  fn strong_case(&self) -> bool;
}


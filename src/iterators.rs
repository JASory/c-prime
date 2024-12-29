use crate::ptrait::Primality;
/// Iterator over the set of primes
///
/// It may start at any point and iterate forwards or backwards 
#[derive(Copy, Clone)]
pub struct Primes<T: Primality>{
    p: T,
}

impl<T: Primality> Primes<T>{
   /// Initialise a new iterator over the Primes starting from 1
   pub const fn new() -> Self{
       Self{p: T::ONE}
   }
   
   /// Initialise a new iterator over the Prime starting from p
   pub const fn start(p: T) -> Self {
        Self { p }
    }
    
   /// Reset iterator to start at new point 
   pub const fn jump_to(&self, p: T) -> Self{
       Self{p}
   } 
   
   /// Returns the last prime
   pub const fn last() -> Self{
      Self{p: T::LAST_PRIME}
   }
   
    /// Syntatic sugar to make it look like a normal iterator
    pub fn iter(&self) -> Self {
        self.clone()
    }
    
   /// Determine if  some x is within the defined set of Primes
    pub fn contains(&self, x: T) -> bool{
        x.is_prime()
    }
    
    /// Returns the primes out of the given collection
    pub fn intersection<F: IntoIterator<Item=T>>(&self, other: F) -> Vec<T>{
        let mut veccy = vec![];
        for i in other.into_iter(){
          if i.is_prime(){
             veccy.push(i)
          }
       }
       veccy
    }
    
    /// Elements in other set that are NOT primes
    pub fn complement<F: IntoIterator<Item=T>>(&self, other: F) -> Vec<T>{
        let mut veccy = vec![];
       for i in other.into_iter(){
          if !i.is_prime(){
             veccy.push(i)
          }
       }
       veccy
    }
  }  

impl<T: Primality> Iterator for Primes<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            
            if self.p >= T::LAST_PRIME {
                return None;
            }
             self.p += T::ONE;
            if self.p.is_prime(){
                return Some(self.p);
            }
        }
    }

    fn last(self) -> Option<Self::Item> {
        Some(T::LAST_PRIME)
    }
}


impl<T: Primality> DoubleEndedIterator for Primes<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if self.p < T::FIRST_PRIME {
                return None;
            }
            self.p -= T::ONE;
           if self.p.is_prime(){
                return Some(self.p);
            }
        }
    }
}

/// Iterator of primes of the form X*RING + RESIDUE
#[derive(Copy, Clone)]
pub struct ResiduePrime<const RING: u128, const RESIDUE: u128> {
    p: u128,
}

impl<const RING: u128, const RESIDUE: u128> ResiduePrime<RING,RESIDUE> {

  pub  fn new(p: u128) -> Option<Self> {
        // Checks that starting value is already a valid residue,
        // but not necessarily prime
        if p % RING == RESIDUE {
            return Some(Self { p });
        }
        None
    }
    
}

impl<const RING: u128, const RESIDUE: u128> Iterator for ResiduePrime<RING,RESIDUE> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.p > u128::LAST_PRIME {
                return None;
            }
            self.p += RING;
            let x: u128 = self.p;
            if machine_prime::is_prime(x) {
                return Some(x);
            }
        }
    }
}


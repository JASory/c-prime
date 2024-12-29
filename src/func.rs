const fn bounded_search(mut x: u128, stride: u128) -> u128{
    loop{
     x= x.wrapping_add(stride);
     
     if x == 0 || x == u128::MAX{
       panic!("Exceeded bounds");
     }
     
     
     if machine_prime::is_prime(x){
        return x;
     }
   }
}

/// Returns the next prime
pub const fn next_prime(x: u128) -> u128{
    bounded_search(x,1)
}

/// Returns the previous prime
pub const fn prev_prime(x: u128) -> u128{
   bounded_search(x,u128::MAX)
}

/// Initialises an  array of primes
pub const fn prime_array<const S: usize>(mut start: u128) -> [u128;S]{
    let mut counter = 0usize;
    let mut valarray : [u128;S] = [0u128;S];
    loop{
        start=next_prime(start);
        valarray[counter]=start;
        counter+=1;    
        if counter == S{
          return valarray;
        }
      }
}

use crate::ptrait::Primality;

impl Primality for f64{
  const ONE : Self = 1f64;
  
  const FIRST_PRIME : Self = -9007199254740881f64;
  
  const LAST_PRIME : Self = 9007199254740881f64;
  
  fn is_prime(&self) -> bool{
    if *self > 9007199254740992f64{
       panic!("Not representable as 64-bit IEEE-754 float")
    }
    machine_prime::is_prime(self.abs() as u64)   
  }
  
  fn strong_case(&self) -> bool{
  
    if *self > 9007199254740992f64{
       panic!("Not representable as 64-bit IEEE-754 float")
    }
    if *self == 1f64{
       return false;
    }
    if (self.abs() as u64)&1 == 0{
      return false
    }
    machine_prime::is_prime_wc(self.abs() as u64)
  }
}

impl Primality for f32{
  const ONE : Self = 1f32;
  
  const FIRST_PRIME : Self = -16777213f32;
  
  const LAST_PRIME : Self = 16777213f32;
  
  fn is_prime(&self) -> bool{
    if *self > 16777216f32{
       panic!("Not representable as 64-bit IEEE-754 float")
    }
    machine_prime::is_prime(self.abs() as u64)   
  }
  
  fn strong_case(&self) -> bool{
  
    if *self > 16777216f32{
       panic!("Not representable as 64-bit IEEE-754 float")
    }
    let n = self.abs() as u64;
    if n == 1{
       return false;
    }
    if n&1 == 0{
      return false
    }
    machine_prime::is_prime_wc(n)
  }
}


impl Primality for u64{

  const ONE : Self = 0x1;
  
  const FIRST_PRIME : Self = 0x2;
  
  const LAST_PRIME : Self = 0xFFFFFFFFFFFFFFC5;
  
  fn is_prime(&self) -> bool{
    machine_prime::is_prime(*self)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(*self)
  }
}

impl Primality for u128{

  const ONE : Self = 0x1;
  
  const FIRST_PRIME : Self = 0x2;
  
  const LAST_PRIME : Self = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF61;
  
  fn is_prime(&self) -> bool{
    machine_prime::is_prime_128(*self)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc_128(*self)
  }
}

impl Primality for i128{

  const ONE : Self = 0x1;
  
  const FIRST_PRIME : Self = -0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
  
  const LAST_PRIME : Self = 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
  
  fn is_prime(&self) -> bool{
    machine_prime::is_prime_128(self.unsigned_abs())   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc_128(self.unsigned_abs())
  }
}

impl Primality for usize{

  const ONE : Self = 0x1;
  
  const FIRST_PRIME : Self = 0x2;
  
  const LAST_PRIME : Self = 0xFFFFFFFFFFFFFFC5;
  
  fn is_prime(&self) -> bool{
    machine_prime::is_prime(*self as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(*self as u64)
  }
}

impl Primality for u32{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = 0x2;
   const LAST_PRIME : Self = 0xFFFFFFFB;
   
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(*self as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(*self as u64)
  }
}


impl Primality for u16{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = 0x2;
   const LAST_PRIME : Self = 0xFFF1;
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(*self as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(*self  as u64)
  }
}

impl Primality for u8{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = 0x2;
   const LAST_PRIME : Self = 0xFB;
   
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(*self as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(*self as u64)
  }
}

impl Primality for i8{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = -0x7F;
   const LAST_PRIME : Self = 0x7F;
   
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(self.unsigned_abs() as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(self.unsigned_abs() as u64)
  }
}

impl Primality for i16{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = -0x7FED;
   const LAST_PRIME : Self = 0x7FED;
   
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(self.unsigned_abs() as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(self.unsigned_abs() as u64)
  }
}

impl Primality for i32{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = -0x7FFFFFFF;
   const LAST_PRIME : Self = 0x7FFFFFFF;
   
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(self.unsigned_abs() as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(self.unsigned_abs() as u64)
  }
}

impl Primality for i64{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = -0x7FFFFFFFFFFFFFE7;
   const LAST_PRIME : Self = 0x7FFFFFFFFFFFFFE7;
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(self.unsigned_abs())   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(self.unsigned_abs())
  }
}

impl Primality for isize{
   const ONE : Self = 0x1;
   const FIRST_PRIME : Self = -0x7FFFFFFFFFFFFFE7;
   const LAST_PRIME : Self = 0x7FFFFFFFFFFFFFE7;
   
   fn is_prime(&self) -> bool{
    machine_prime::is_prime(self.unsigned_abs() as u64)   
  }
  
  fn strong_case(&self) -> bool{
    if *self&1 == 0{
      return false
    }
    if *self==1{
       return false
    }
    machine_prime::is_prime_wc(self.unsigned_abs() as u64)
  }
}


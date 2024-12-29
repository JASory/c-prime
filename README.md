Utilises [machine-prime](https://github.com/JASory/machine-prime/) to model sets of prime numbers.
This library produces primes simply by sequentially checking integers, consequently it is very slow compared to sieving
but may be faster if you start at a larger point. e.g iterating through a million primes starting from 0 is going to be slow compared to sieving,
but much faster than sieving if you start at 2^64.

The "primeset"s don't store any primes. So they can run until they reach the integer max. 



use primes::{Sieve, PrimeSet};
use rand::prelude::*;
fn main() {
    //Gets random number 
    let mut rng = rand::rng();
    //Some woodoo magic by the primes library?? (idk what are they called kek)
    let mut p_set = Sieve::new();
    
    let prime_p = p_set.get(100000);
    println!("{prime_p}");
}

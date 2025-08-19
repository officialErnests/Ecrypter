use primes::{Sieve, PrimeSet};
use rand::prelude::*;
use num_integer::gcd;
fn main() {
    //Gets random number 
    //Some woodoo magic by the primes library?? (idk what are they called kek)
    let mut p_set = Sieve::new();

    let prime_p = p_set.get(rand::rng().random_range(10000..100000));
    let prime_q = p_set.get(rand::rng().random_range(10000..100000));
    println!("{prime_p}, {prime_q}");
    let public_key = prime_p * prime_q;
    println!("{public_key}");
    let coprime_goal = (prime_p - 1) * (prime_q - 1);
    let mut coprime = 0;
    //diy do while loop XDD
    //WAIT d * e = 1 mod 40 what am i doing kek
    while (true){
        //most diabolical line yet XD coprime_goal.try_into().unwrap()
        coprime = p_set.get(rand::rng().random_range(2..coprime_goal.try_into().unwrap()));
        if ((coprime_goal / coprime) % 0 == 0) {
            //im winning awards w this one
            break;
        }
    }
}

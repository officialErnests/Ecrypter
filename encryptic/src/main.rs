use std::array;

use primes::{PrimeSet, PrimeSetBasics, Sieve};
use rand::prelude::*;
use mod_exp::mod_exp;
fn main() {
    //Gets random number 
    //Some woodoo magic by the primes library?? (idk what are they called kek)
    let mut coprime: u64 = 0;
    let mut coprime_2: u64 = 0;
    let mut public_key: u64  = 0;

    println!("{}, {}", coprime, coprime_2);
    let encrypted_message = encrypt(696969, coprime,public_key);
    println!("{:?}", encrypted_message);
    let decoded_message = decrypt(encrypted_message, coprime_2, public_key);
    println!("{:?}", decoded_message);

    // println!("{}", p_set.list()[p_set.len() - 1]);
    // loop {
    //     //most diabolical line yet XD coprime_goal.try_into().unwrap()
    //     coprime = p_set.get(rand::rng().random_range(2..coprime_goal.try_into().unwrap()));
    //     //checks if prime
    //     //checks if 
    //     if (p_set.is_prime(coprime) && (coprime_goal / coprime) % 0 == 0 && coprime * coprime_goal / coprime == 1) {
    //         //im winning awards w this one
    //         break;
    //     }
    // }
    // println!("{coprime}, {}",coprime_goal/coprime)
}   

fn generate_privateInfo(coprime : u64, coprime_2 : u64, public_key :u64) {
    let mut p_set = Sieve::new();

    let prime_p = p_set.get(rand::rng().random_range(100..1000));
    let prime_q = p_set.get(rand::rng().random_range(100..1000));
    println!("{prime_p}, {prime_q}");
    let public_key = prime_p * prime_q;
    println!("{public_key}");
    let coprime_goal: u64 = (prime_p - 1) * (prime_q - 1);
    // let coprime_goal = 40;
    let mut coprime: u64 = 0;
    //diy do while loop XDD
    //WAIT d * e = 1 mod 40 what am i doing kek
    // p_set.find(coprime_goal);
    let x = p_set.prime_factors(coprime_goal);
    println!("{:?}", x);
    println!("{}", coprime);
    loop {
        let rand_prime: Option<&u64> = p_set.list().choose(&mut rand::rng());
        match rand_prime {
            Some(prime) => {
                if !x.contains(prime) && (coprime_goal > *prime){
                    coprime = *prime;
                    break;
                }
            },
            None => println!("Bap"),
        }
    }
    //Tried to learn modular arithamtic and what tf is inverse but i think bruthe forcing it seems faster XXDD
    let mut coprime_2 = 1;
    loop {
        coprime_2 += 1;
        if (coprime * coprime_2) % coprime_goal == 1 {
            break;
        }
    }
    return [534,334];
}

fn encrypt(message: u64, coprime: u64, publick_key: u64) -> u64 {
    return mod_exp(message, coprime, publick_key);
} 

fn decrypt(encoded_message: u64, coprime2: u64, publick_key: u64) -> u64 {
    return mod_exp(encoded_message, coprime2, publick_key);
}
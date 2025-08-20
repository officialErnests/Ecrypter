use primes::{PrimeSet, PrimeSetBasics, Sieve};
use rand::prelude::*;
use mod_exp::mod_exp;
use std::io;
fn main() {
    //Gets random number 
    //Some woodoo magic by the primes library?? (idk what are they called kek)
    println!("Paste keys (Leave empty to generate new):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut coprime = 0;
    let mut coprime_2 = 0;
    let mut public_key = 0;
    input = input.trim().to_string();
    if input.len() < 3{
        let funny = generate_private_info();
        coprime = funny[0];
        coprime_2 = funny[1];
        public_key = funny[2];
    } else {
        let funny: Vec<&str> = input.split(",").collect();
        let mut x: i32 = 0;
        println!("{:?}", funny);
        for i in funny{
            match x {
                0 => {coprime = i.parse().unwrap()},
                1 => {coprime_2 = i.parse().unwrap()},
                2 => {
                    public_key = i.parse().unwrap()
                },
                _ => println!("Well idk what to do with {x}")
            }
            x += 1;
        }
    }
    println!("private key: {coprime}\ncoprime 2: {coprime_2}\npublic key: {public_key}");
    println!("copy : {coprime},{coprime_2},{public_key}\n");
    loop {
        println!("0 - encode mesage, 1 - decode message");
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let temp: i32 = input.parse().unwrap();
        if temp == 0 {
            println!("encode message (numbers kek):");
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            let encrypted_message = encrypt(input.parse().unwrap(), coprime, public_key);
            println!("encrypted : {:?}", encrypted_message);
        }
        else {
            println!("decode message (numbers kek):");
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            let decoded_message = decrypt(input.parse().unwrap(), coprime_2, public_key);
            println!("decrypted : {:?}", decoded_message);
        }
    }

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

fn generate_private_info() -> Vec<u64> {
    let mut p_set = Sieve::new();

    let prime_p = p_set.get(rand::rng().random_range(100..1000));
    let prime_q = p_set.get(rand::rng().random_range(100..1000));
    // println!("{prime_p}, {prime_q}");
    let public_key = prime_p * prime_q;
    let coprime_goal: u64 = (prime_p - 1) * (prime_q - 1);
    // let coprime_goal = 40;
    //diy do while loop XDD
    //WAIT d * e = 1 mod 40 what am i doing kek
    // p_set.find(coprime_goal);
    let coprime: u64 ;
    let x = p_set.prime_factors(coprime_goal);
    // println!("{:?}", x);
    // println!("{}", coprime);
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
    let mut return_vec = Vec::with_capacity(3);
    return_vec.push(coprime);
    return_vec.push(coprime_2);
    return_vec.push(public_key);
    return return_vec;
}

fn encrypt(message: u64, coprimew: u64, publick_key: u64) -> u64 {
    return mod_exp(message, coprimew, publick_key);
} 

fn decrypt(encoded_message: u64, coprime2: u64, publick_key: u64) -> u64 {
    return mod_exp(encoded_message, coprime2, publick_key);
}
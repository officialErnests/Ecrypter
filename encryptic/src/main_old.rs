use std::io;
use std::io::prelude::*;
// use std::fs;
// use serde::Deserialize;
use serde_json::{Result, Value};
//export comments
/*
Tf u mean linking failed kek
Why do you have to delet the previos export :skull:
*/
//WHY JSON SO HARDDDDD ;;;-;;;

fn untyped_example() -> Result<()> {
    //WWHWHYYYY DO YOU NEED THISSSSSSSssss
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let deserialized_primes: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", deserialized_primes["name"], deserialized_primes["phones"][0]);

    Ok(())
}
fn main() {
    untyped_example().unwrap();
    println!("Pleas select action then paste the text :P");
    println!("1-Handshake generation");
    println!(" leave empty to generate private and publick key");
    println!(" paste in public key to set it");
    println!("2-Message generation");
    println!(" write text to generate message");
    println!("3-Message decrypter");
    println!(" paste message to decrypt");
    //wahahahhahaahahahah why json
    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;
    // let prime_file: String = fs::read_to_string("../res/prime.json").expect("Oh god whyyyyyyy");
    // let v: Value = serde_json::from_str(data)?;
    // let deserialized_primes: Value = serde_json::from_str(data);
    //WHy do you have to fight compiler so mutch XDD
    //Why is the snake case absolete kek
    //Whyy the variables so weeeeeird man i love cpp better kek
    let prime_p = gen_private_key();
    let prime_q = 1;
    let _prime_mul = prime_p * prime_q; // publick key :DD
    let _co_prime = (prime_p - 1) * (prime_q - 1); // figure out how to get coprime from this number XDD
    let _reciver_publick_key = 0;
    let _reciver_coprime = 0;
    let _message_to_send = 0;
    let _encoded_message_to_send = 0;
    let _decoder_factor = 0; // d in my math so i don't get confused kek


    // println!("{}", v["name"]);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

}
// damn even functions need snake case XDD
fn gen_private_key() -> u128 {
    //big ass number kek
    return u128::MAX;
}

// fn genPublicKey() {
//     //does alot of goofy shit XD
// }

// fn genHandshake() {

// }

// fn genOutput() {
    
// }

// fn genText() {

// }
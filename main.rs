use std::io;
use std::io::prelude::*;
//export comments
/*
Tf u mean linking failed kek
Why do you have to delet the previos export :skull:
*/
fn main() {
    println!("Pleas select action then paste the text :P");
    println!("1-Handshake generation");
    println!(" leave empty to generate private and publick key");
    println!(" paste in public key to set it");
    println!("2-Message generation");
    println!(" write text to generate message");
    println!("3-Message decrypter");
    println!(" paste message to decrypt");
    ///wahahahhahaahahahah why json
    let json_file_path = Path::new("primes.json");
    let prime_file = File::open(json_file_path);
    let prime_json: serde_json::Value = serde_json::from_str(prime_file).expect("JSON was not well-formatted");
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


    println!("{}", prime_json["200"]);
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
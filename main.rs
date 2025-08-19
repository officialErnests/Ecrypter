use std::io;
use std::io::prelude::*;

fn main() {
    println!("Pleas select action then paste the text :P");
    println!("1-Handshake generation");
    println!(" leave empty to generate private and publick key");
    println!(" paste in public key to set it");
    println!("2-Message generation");
    println!(" write text to generate message");
    println!("3-Handshake generation");


    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

}

// fn genHandshake() {

// }

// fn genOutput() {
    
// }

// fn genText() {

// }
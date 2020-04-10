use std::io;
use std::env;

fn main() {
    // Accept command line args as the rotors and positions
    // There should be four rotors, each separated by spaces
    let args: Vec<String> = env::args().collect();
    let mut settings = Vec::<String>::new();

    enum Rotors {
        // I{ String: new("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
        // II,
        // III,
        // IV,
        // V,
        // VI,
        // VII,
        // VIII,
        // ETW{ category: Vec<String>, alphabet: String: "ABCDEFGHIJKLMNOPQRSTUVWXYZ" },
        // BETA,
        // GAMMA,
        // UKWB,
        // UKWC
    }

    println!("{:?}", args);
    
    let rotor1 = &args[1];
    let rotor2 = &args[2];
    let rotor3 = &args[3];
    let rotor4 = &args[4];

    println!("Please type your message:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    
    println!("{}", input);
}

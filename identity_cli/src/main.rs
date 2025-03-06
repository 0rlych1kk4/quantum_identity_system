# identity_cli/src/main.rs
use clap::{App, Arg};
use identity_core::QuantumIdentity;
use std::fs;

fn main() {
    let matches = App::new("Quantum Identity CLI")
        .version("0.1.0")
        .author("Orly")
        .about("Manages quantum-secure digital identities")
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .about("Generates a new quantum identity"),
        )
        .arg(
            Arg::new("verify")
                .short('v')
                .long("verify")
                .takes_value(true)
                .about("Verifies a quantum identity from a JSON file"),
        )
        .get_matches();

    if matches.is_present("generate") {
        let identity = QuantumIdentity::new();
        let serialized = serde_json::to_string_pretty(&identity).unwrap();
        fs::write("identity.json", &serialized).expect("Failed to save identity");
        println!("Generated Quantum Identity:\n{}", serialized);
    }

    if let Some(file) = matches.value_of("verify") {
        let data = fs::read_to_string(file).expect("Failed to read file");
        let identity: QuantumIdentity = serde_json::from_str(&data).expect("Invalid JSON format");
        if identity.verify() {
            println!("Identity verification successful!");
        } else {
            println!("Identity verification failed!");
        }
    }
}


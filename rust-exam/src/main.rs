extern crate serde_json;

use std::fs::File;
use std::io::Read;

use serde_json::Value;

fn main() {

    println!("rust-exam");

    let mut file = File::open("questions.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let json: Value = serde_json::from_str(&content).unwrap();
    println!("{}", json["questions"][0]["question"]);
}

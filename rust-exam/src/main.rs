extern crate serde_json;

use std::io;
use std::io::Read;

use std::fs::File;

use serde_json::value::Value;

fn main() {

    println!("rust-exam");

    let mut file = File::open("questions.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let json: Value = serde_json::from_str(&content).unwrap();

    let mut mark: u8 = 0;

    for counter in 0..7 {

        let group = &json["questions"][counter];

        println!(
            "{}. {}",
            counter,
            group["question"].as_str().unwrap()
        );

        println!(
            "{}", 
            group["code"].as_str().unwrap()
        );

        println!(
            "a) {}",
            group["a"].as_str().unwrap()
        );

        println!(
            "b) {}",
            group["b"].as_str().unwrap()
        );

        println!(
            "c) {}",
            group["c"].as_str().unwrap()
        );

        println!(
            "d) {}",
            group["d"].as_str().unwrap()
        );

        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("error reading the input");

        // required to compare the input with the correct answer
        let response: &str = response.trim();

        let answer: &str = group["answer"].as_str().unwrap();

        match response.eq(answer) {
            true => {
                println!("Correct!");
                mark += 1;
            },
            false => println!("Wrong"),
        }
    }

    println!(
        "Mark: {} / 6",
        mark
    );
}

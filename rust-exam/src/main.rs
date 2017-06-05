extern crate serde_json;
extern crate rand;

use std::io;
use std::io::Read;

use std::fs::File;

use serde_json::value::Value;

use rand::Rng;

/// Displays a dashes separator
fn display_separator() {
    println!("----------------------");
}

/// Displays the exemple code of the question
fn display_code(code: &str) {

    display_separator();

    println!(
        "{}",
        code
    );

    display_separator();
}

fn main() {

    println!("rust-exam");

    let mut file = File::open("questions.json").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let json: Value = serde_json::from_str(&content).unwrap();

    let mut mark: u8 = 0;

    for counter in 0..20 {

        /* we never check if a number has already been selected;
           in fact, the amount of questions will become high enough
           to ignore this case (the number of asked questions is still 20) */
        let random = rand::thread_rng().gen_range(0, 35);
        let group = &json["questions"][random];

        println!(
            "{}. {}",
            counter,
            group["question"].as_str().unwrap()
        );

        match group["code"].as_str() {
            Some(code) => display_code(code),
            None => {}
        }

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
            false => {
                println!(
                    "Wrong, the correct answer is : {}",
                    answer
                )
            }
        }
    }

    println!(
        "Mark: {} / 20",
        mark
    );
}

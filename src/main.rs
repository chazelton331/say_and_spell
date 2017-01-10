extern crate rand;

#[macro_use] extern crate text_io;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::process;
use std::process::Command;

use rand::Rng;

fn load_words_db(filename : &str) -> Vec<String> {
    let mut words_vec: Vec<String> = Vec::new();

    let fd = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("\nFailed to open words database, make sure file 'words.db' exists");
            process::exit(1);
        }
    };

    let buf_reader = BufReader::new(fd);

    for line in buf_reader.lines() {
        words_vec.push(line.unwrap());
    }

    words_vec
}

fn say(words : String) {
    let output = Command::new("say")
                         .arg("-v")
                         .arg("Alex")
                         .arg(words)
                         .output()
                         .expect("The 'say' command failed.");

    output.stdout;
}

fn quiz_for_random_word(words_vec : &Vec<String>, score : &mut i32) {
    let mut spell_string = "Spell the word ".to_string();
    let word = rand::thread_rng().choose(words_vec).unwrap();

    spell_string.push_str(word);
    say(spell_string);

    let user_input: String = read!("{}\n");

    if user_input == word.to_string() {
        say("Yaaaay! You correctly spelled the word!".to_string());
        *score += 1;
    } else {
        say("Sorry, you spelled it incorrectly. No points!".to_string());
    }
}

fn user_is_done() -> bool {
        println!("Press n to quit or any other key to keep going.");

        let user_input : String = read!("{}\n");

        user_input == "n"
}

fn main() {
    let filename = "words.db";
    let words_vec: Vec<String> = load_words_db(filename);
    let mut score = 0;

    loop {
        quiz_for_random_word(&words_vec, &mut score);

        if user_is_done() {
            let message = "Bye bye!";
            println!("{}", message);
            say(message.to_string());

            break;
        }
    }
}

extern crate colored;

use colored::*;
use rand::prelude::*;
use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};

const WORD_FILE: &str = "words.txt";

fn load_word_list() -> Vec<String> {
    let f = File::open(WORD_FILE).unwrap();
    let reader = BufReader::new(f);
    let mut words: Vec<String> = vec![];

    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.len() == 0 {
            break;
        }
        words.push(line);
    }
    words
}

fn choose_word(words: &Vec<String>) -> String {
    let mut rng = thread_rng();
    let index: usize = rng.gen_range(0..words.len());
    let chosen_word = &words[index];
    chosen_word.to_string()
}

fn get_next_guess(used_invalid_letters: &Vec<char>, word_list: &Vec<String>) -> String {
    if used_invalid_letters.len() > 0 {
        println!(
            "The following letters are out: {}",
            used_invalid_letters
                .into_iter()
                .map(|c| c.to_uppercase().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    println!("Enter your guess: ");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Could not read guess");
    guess = guess.trim().to_lowercase();
    if guess.len() != 5 {
        println!("Please enter a 5 letter word. Try again.");
        return get_next_guess(&used_invalid_letters, &word_list);
    }
    let excluded_letters: Vec<char> = guess
        .chars()
        .filter(|letter| used_invalid_letters.contains(letter))
        .collect();
    if excluded_letters.len() > 0 {
        let excluded_letter_string = excluded_letters
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");
        println!(
            "The following letters in your guess are invalid: {}. Try again",
            &excluded_letter_string
        );
        return get_next_guess(&used_invalid_letters, &word_list);
    }
    if !word_list.contains(&guess) {
        println!("Not in word list. Try again.");
        return get_next_guess(&used_invalid_letters, &word_list);
    }
    guess
}

fn evaluate_guess(word: &String, guess: &String, used_invalid_letters: &mut Vec<char>) -> bool {
    println!();
    println!();
    print!("\t\t");
    let target_characters: Vec<char> = word.chars().collect::<Vec<char>>();
    let guess_characters: Vec<char> = guess.chars().collect::<Vec<char>>();
    let mut is_correct = true;
    for i in 0..guess.len() {
        let letter = guess_characters[i];
        let target_letter = target_characters[i];
        if !target_characters.contains(&letter) {
            used_invalid_letters.push(letter);
            let result_letter_string: String = letter.to_uppercase().to_string();
            print!("{} ", result_letter_string.normal().bold());
            is_correct = false;
        } else if target_characters.contains(&letter) && letter != target_letter {
            let result_letter_string: String = letter.to_uppercase().to_string();
            print!("{} ", result_letter_string.yellow().bold());
            is_correct = false;
        } else {
            let result_letter_string: String = letter.to_uppercase().to_string();
            print!("{} ", result_letter_string.green().bold());
        }
    }
    println!();
    println!();
    is_correct
}

fn play(words: Vec<String>) -> () {
    println!();
    println!();
    println!("\t\tWelcome to Command-Line Wordle!");
    println!();
    println!("\t\tYou have six attempts to guess a five letter word.");
    println!("\t\tAfter each guess, any letters in the correct positions will be green.");
    println!("\t\tWhile letters that are in the word but in the wrong position will be yellow.");
    println!("\t\tGood luck!");
    println!();
    let word = choose_word(&words);
    let mut used_invalid_letters: Vec<char> = Vec::new();
    let mut has_won = false;
    for i in 0..6 {
        let guess_num = i + 1;
        println!("Guess attempt #{}", &guess_num);
        let guess = get_next_guess(&used_invalid_letters, &words);
        has_won = evaluate_guess(&word, &guess, &mut used_invalid_letters);
        if has_won {
            println!("Congratulations!!");
            println!("You won Wordle in {} guesses.", &guess_num);
            break;
        }
    }
    if !has_won {
        println!("The word was {}", &word);
        println!("I'm sorry. Better luck next time!");
    }
}

fn main() {
    let word_list: Vec<String> = load_word_list();
    play(word_list);
}

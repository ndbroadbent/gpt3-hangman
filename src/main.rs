extern crate rand;

use serde_yaml; // 0.8.7
use std::convert::TryFrom;

use rand::prelude::*;
use rand_pcg::Pcg64;

use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;

macro_rules! puts{
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

static ASCII_SORTED: [char; 26] = [
    'e', 't', 'a', 'i', 'n', 'o', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'f', 'w', 'y', 'g', 'p',
    'b', 'v', 'k', 'q', 'j', 'x', 'z',
];

fn obfuscate_input(input: &String, guesses: &Vec<char>) -> (String, u128) {
    let ascii_re: Regex = Regex::new(r"[A-Za-z]").unwrap();
    let mut missing = 0;

    let mut result = String::from("");
    for c in input.chars() {
        if ascii_re.is_match(&c.to_string()) {
            let lowercase: char = c.to_lowercase().collect::<Vec<char>>()[0];
            if guesses.contains(&lowercase) {
                result.push(c);
            } else {
                result.push('_');
                missing += 1;
            }
        } else {
            result.push(c);
        }
    }
    return (result, missing);
}

fn random_unguessed_letter_from_input(input: &String, guesses: &Vec<char>) -> char {
    let mut rng = Pcg64::seed_from_u64(2);
    let mut next_char: char;
    let input_lower = input.to_lowercase();

    let mut guess_count = 0;
    loop {
        let char_index = rng.gen_range(0..ASCII_SORTED.len());
        next_char = ASCII_SORTED[char_index];
        // puts!(char_index, next_char);
        let lowercase = next_char.to_lowercase().collect::<Vec<char>>()[0];
        if !guesses.contains(&lowercase) && input_lower.contains(&lowercase.to_string()) {
            return next_char;
        }
        guess_count += 1;
        if guess_count > 100000 {
            panic!("Whoops, infinite loop!");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = File::open("./config.yml")?;

    let config: BTreeMap<String, Vec<String>> = serde_yaml::from_reader(config_file)?;

    let mut total_answers = 0;
    for (answer_type, answers) in config {
        for answer in answers {
            total_answers += 1;

            print!("Hint: ");
            match answer_type.as_str() {
                "actors" => println!("This person is an actor."),
                "scientists" => println!("This person is a scientist."),
                "fruits" => println!("This is a fruit."),
                _ => panic!("Don't know how to handle {}!", answer_type),
            }
            println!();

            let re = Regex::new(r"[A-Za-z]").unwrap();
            let _result = re.replace_all(answer.as_str(), "_");

            // let mut rng = Pcg64::seed_from_u64(2);

            let mut guessed_letters: Vec<char> = Vec::new();

            let mut missing_count: u128 = u128::try_from(answer.len()).unwrap();
            let mut common_letter_guesses = 0;

            while missing_count > 3 {
                let (result, next_missing_count) = obfuscate_input(&answer, &guessed_letters);
                missing_count = next_missing_count;
                println!("{}", result);

                let next_letter;
                if common_letter_guesses < 4 {
                    next_letter = ASCII_SORTED[common_letter_guesses];
                    common_letter_guesses += 1;
                } else {
                    // After guessing the 5 most common letters,
                    // just pull the letters directly from the answer (in a random order)
                    next_letter = random_unguessed_letter_from_input(&answer, &guessed_letters);
                }
                guessed_letters.push(next_letter);

                println!("=> {}", next_letter);

                let (_result, after_missing_count) = obfuscate_input(&answer, &guessed_letters);
                let match_count = missing_count - after_missing_count;
                match match_count {
                    0 => println!("- No {}'s!", next_letter),
                    1 => println!("- 1 {}!", next_letter),
                    _ => println!("- {} {}'s!", match_count, next_letter),
                }

                // let guessed_letters_fmt = guessed_letters
                //     .iter()
                //     .map(|c| c.to_string())
                //     .collect::<Vec<String>>()
                //     .join(", ");

                // println!("- Guesses: {}", guessed_letters_fmt);
                // println!("- {} letters missing", missing_count);
                println!();

                missing_count = after_missing_count;
            }

            println!("Would you like to make a guess?");
            println!("Answer: {}", answer);

            println!("\n---------------------------------------------\n");
        }
    }

    puts!(total_answers);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obfuscate_input() {
        assert_eq!(
            (String::from("_____"), 5),
            obfuscate_input(&String::from("asdff"), &vec![])
        );
        assert_eq!(
            (String::from("a____"), 4),
            obfuscate_input(&String::from("asdff"), &vec!['a'])
        );
        assert_eq!(
            (String::from("a__ff"), 2),
            obfuscate_input(&String::from("asdff"), &vec!['a', 'f'])
        );
        assert_eq!(
            (String::from("asdff"), 0),
            obfuscate_input(&String::from("asdff"), &vec!['a', 's', 'd', 'f'])
        );
        assert_eq!(
            (String::from("___ ______"), 9),
            obfuscate_input(&String::from("Tom Cruise"), &vec![])
        );
        assert_eq!(
            (String::from("T__ ______"), 8),
            obfuscate_input(&String::from("Tom Cruise"), &vec!['t'])
        );
        assert_eq!(
            (String::from("T_m ______"), 7),
            obfuscate_input(&String::from("Tom Cruise"), &vec!['t', 'm'])
        );
        assert_eq!(
            (String::from("T_m C_____"), 6),
            obfuscate_input(&String::from("Tom Cruise"), &vec!['t', 'm', 'c'])
        );
        assert_eq!(
            (String::from("Tom Cr____"), 4),
            obfuscate_input(&String::from("Tom Cruise"), &vec!['t', 'o', 'm', 'c', 'r'])
        );
        assert_eq!(
            (String::from("Tom Cr____"), 4),
            obfuscate_input(
                &String::from("Tom Cruise"),
                &vec!['t', 'o', 'm', 'c', 'r', 'z']
            )
        );
        assert_eq!(
            (String::from("Tom Cruise"), 0),
            obfuscate_input(
                &String::from("Tom Cruise"),
                &vec!['t', 'o', 'm', 'c', 'r', 'u', 'i', 's', 'e']
            )
        );
    }

    #[test]
    fn test_random_unguessed_letter_from_input() {
        assert_eq!(
            'a',
            random_unguessed_letter_from_input(&String::from("asdff"), &vec![])
        );
        assert_eq!(
            'd',
            random_unguessed_letter_from_input(&String::from("asdff"), &vec!['a'])
        );
        assert_eq!(
            's',
            random_unguessed_letter_from_input(&String::from("asdff"), &vec!['a', 'd'])
        );
        assert_eq!(
            'd',
            random_unguessed_letter_from_input(&String::from("sdff"), &vec![])
        );
        assert_eq!(
            'u',
            random_unguessed_letter_from_input(&String::from("foobarbazqux"), &vec![])
        );
        assert_eq!(
            'a',
            random_unguessed_letter_from_input(&String::from("foobarbazqux"), &vec!['u'])
        );
    }

    #[test]
    #[should_panic(expected = "Whoops, infinite loop!")]
    fn test_random_unguessed_letter_from_input_panic() {
        random_unguessed_letter_from_input(&String::from("asd"), &vec!['a', 's', 'd']);
    }
}

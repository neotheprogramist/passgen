use clap::Parser;
use rand::seq::{IteratorRandom, SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short = 'l', long, default_value = "16")]
    pub length: usize,

    #[clap(short = 'd', long, default_value = "4")]
    pub digit_num: usize,

    #[clap(short = 's', long, default_value = "4")]
    pub special_char_num: usize,
}

/// Generates a password based on specified constraints for length, number of digits, and number of special characters.
pub fn generate_password(length: usize, digit_num: usize, special_char_num: usize) -> String {
    let mut rng = rand::thread_rng();

    // Character sets for the password components
    const BASE58: &str = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    const DIGITS: &str = "123456789";
    const SPECIAL: &str = "!@#$%^&*";

    // Pool of all possible characters for the password
    let mut characters = vec![];
    characters.extend(SPECIAL.chars());
    characters.extend(DIGITS.chars());
    characters.extend(BASE58.chars());

    let mut result = vec![None; length];

    // Create a list of indices to randomly assign characters positions
    let mut indices: Vec<usize> = (0..length).collect();
    indices.shuffle(&mut rng);

    // Assign special characters randomly
    for i in indices.iter().take(special_char_num) {
        result[*i] = Some(SPECIAL.chars().choose(&mut rng).unwrap());
    }

    // Assign digits randomly
    for i in indices[special_char_num..(special_char_num + digit_num)].iter() {
        result[*i] = Some(DIGITS.chars().choose(&mut rng).unwrap());
    }

    // Fill the rest of the password with a random selection from the entire character set
    for i in &mut result.iter_mut().filter(|r| r.is_none()) {
        *i = Some(*characters.choose(&mut rng).unwrap());
    }
    result.iter().map(|&c| c.unwrap()).collect()
}

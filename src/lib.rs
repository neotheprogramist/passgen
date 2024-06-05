use clap::Parser;
use rand::prelude::SliceRandom;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::iter;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short = 'l', long, default_value = "16")]
    pub length: usize,

    #[clap(short = 'd', long, default_value = "1")]
    pub digits: usize,

    #[clap(short = 's', long, default_value = "1")]
    pub specials: usize,
}

/// Generates a password based on specified constraints for length, number of digits, and number of special characters.
pub fn generate_password(length: usize, digits: usize, specials: usize) -> String {
    const BASE58: &str = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    const DIGITS: &str = "123456789";
    const SPECIAL: &str = "!@#$%^&*";
    let mut password_chars: Vec<char> =
        iter::repeat_with(|| SPECIAL.chars().choose(&mut rand::thread_rng()).unwrap())
            .take(specials)
            .chain(
                iter::repeat_with(|| DIGITS.chars().choose(&mut rand::thread_rng()).unwrap())
                    .take(digits),
            )
            .chain(
                iter::repeat_with(|| BASE58.chars().choose(&mut rand::thread_rng()).unwrap())
                    .take(length - digits - specials),
            )
            .collect();
    password_chars.shuffle(&mut rand::thread_rng());
    password_chars.into_iter().collect()
}

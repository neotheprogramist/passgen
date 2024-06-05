use clap::Parser;
use rand::{prelude::SliceRandom, seq::IteratorRandom};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short = 'l', long, default_value = "16")]
    pub length: u32,

    #[clap(short = 'd', long, default_value = "1")]
    pub digits: u32,

    #[clap(short = 's', long, default_value = "1")]
    pub specials: u32,
}

/// Generates a password based on specified constraints for length, number of digits, and number of special characters.
pub fn generate_password(length: u32, digits: u32, specials: u32) -> String {
    let mut rng = rand::thread_rng();

    const DIGITS: &str = "123456789";
    const SPECIAL: &str = "!@#$%^&*";
    const ALL: &str = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz123456789!@#$%^&*";

    let mut result = Vec::new();

    for _ in 0..digits {
        result.push(DIGITS.chars().choose(&mut rng).unwrap());
    }

    for _ in 0..specials {
        result.push(SPECIAL.chars().choose(&mut rng).unwrap());
    }

    for _ in 0..(length - digits - specials) {
        result.push(ALL.chars().choose(&mut rng).unwrap());
    }

    result.shuffle(&mut rng);

    result.into_iter().collect()
}

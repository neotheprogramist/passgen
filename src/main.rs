use clap::Parser;
use passgen::Args;
fn main() {
    let args = Args::parse();
    let password = passgen::generate_password(
        args.length as usize,
        args.digit_num as usize,
        args.special_char_num as usize,
    );
    println!("{}", password);
}

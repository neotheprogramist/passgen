use clap::Parser;
use passgen::Args;
fn main() {
    let args = Args::parse();

    if args.digit_num + args.special_char_num > args.length {
        eprintln!("Error: The sum of digits and special characters cannot exceed the total length of the password.");
        std::process::exit(1);
    }
    let password = passgen::generate_password(
        args.length as usize,
        args.digit_num as usize,
        args.special_char_num as usize,
    );
    println!("{}", password);
}

use clap::Parser;
use passgen::Args;
fn main() {
    let args = Args::parse();

    if args.digits + args.specials > args.length {
        eprintln!("Error: The sum of digits and special characters cannot exceed the total length of the password.");
        std::process::exit(1);
    }
    let password = passgen::generate_password(
        args.length as usize,
        args.digits as usize,
        args.specials as usize,
    );
    println!("{}", password);
}

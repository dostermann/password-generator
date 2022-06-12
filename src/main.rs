use rand::prelude::*;
use rand::distributions::Alphanumeric;
use std::io;
use std::io::Write;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let mut length: String = String::new();

    let gen_passwd = |length| -> String {
        let password = thread_rng().sample_iter(Alphanumeric).take(length).map(char::from).collect();
        password
    };

    println!("Welcome to password_generator {}!", VERSION);
    println!("");

    print!("Please enter the length of your new password: ");
    io::stdout().flush().expect("Failed to flush the line!");

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read the line!");

    println!("");

    let length: usize = match length.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1),
    };

    let new_password = gen_passwd(length);

    println!("Your new password is:");
    println!("");
    println!("{}", new_password);
    println!("");
}

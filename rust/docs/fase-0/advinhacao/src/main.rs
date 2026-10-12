use std::io;

fn main() {
    println!("Guess the number game");
    println!("You should guess the number between 1 and 1k");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("You failed!");

    println!("You guessed: {guess}")
}

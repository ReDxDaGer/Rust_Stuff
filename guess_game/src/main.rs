use std::io;

fn main() {
    println!("Guess the number !");
    
    println!("Please input the number that u guessed");
        
    let mut guess = String::new();
    io::stdin
        .readline(&mut guess)
        .expect("Failed to read the line!");
    println!("You guessed {guess}");
}

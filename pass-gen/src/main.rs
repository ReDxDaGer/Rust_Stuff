use rand::Rng;
use std::io;

fn generate_password(name: &str, length: usize) -> String {
    let mut rng = rand::thread_rng();

    // uses the first lettr of thename you entered and if no name takes n word as default
    let first_char = name.chars().next().unwrap_or('n');
    //Covert it to stringif
    let name_part = name.chars().take(4).collect::<String>();


    //generates random numbers
    let random_text: String = (0..(length - name_part.len() - 1))
        .map(|_| rng.gen_range('0'..='9'))
        .collect();
    format!("{}{}{}", first_char, name_part, random_text)
}


//Main function that take the name as input and uses it to generate password
fn main() {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    let password = generate_password(name, 12);
    println!("Generated Password: {}", password);
}

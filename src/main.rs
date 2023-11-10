use rand::seq::IteratorRandom;
use std::io::{self, Write}; // Import the Write trait
use std::thread;
use std::time::Duration;

fn generate_password(length: usize, use_special_chars: bool) -> String {
    let chars: Vec<char> = if use_special_chars {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:'\",.<>?/".chars().collect()
    } else {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect()
    };

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| chars.iter().choose(&mut rng).unwrap())
        .collect();

    password
}

fn print_with_delay(s: &str, delay_ms: u64) {
    for c in s.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

fn main() {
    loop {
        println!("Welcome to the Rust Password Generator!");

        // Get password length from the user
        let mut length_str = String::new();
        println!("Enter the desired password length (1-32): ");
        io::stdin().read_line(&mut length_str).expect("Failed to read input");
        let length: usize = match length_str.trim().parse() {
            Ok(num) if num >= 1 && num <= 32 => num,
            _ => {
                println!("Invalid input. Please enter a number between 1 and 32.");
                continue;
            }
        };

        // Ask if the user wants to use special characters
        let mut use_special_chars_str = String::new();
        println!("Use special characters? (y/n): ");
        io::stdin()
            .read_line(&mut use_special_chars_str)
            .expect("Failed to read input");
        let use_special_chars = match use_special_chars_str.trim() {
            "y" => true,
            "n" => false,
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                continue;
            }
        };

        // Generate and print the password with typewriter effect
        let password = generate_password(length, use_special_chars);
        print_with_delay("Generated Password: ", 30);
        print_with_delay(&password, 50);

        // Ask if the user wants to generate another password
        let mut regenerate_str = String::new();
        println!("\nDo you want to generate another password? (y/n): ");
        io::stdin()
            .read_line(&mut regenerate_str)
            .expect("Failed to read input");

        if !regenerate_str.trim().eq_ignore_ascii_case("y") {
            break;
        }

        // Add a line break for better separation between iterations
        println!();
    }
}
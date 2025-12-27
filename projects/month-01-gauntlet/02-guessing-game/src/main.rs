// As a player, I want to guess a number within a specified range so that I can challenge myself and improve my guessing skills.
// Acceptance Criteria:
// 	•	The game should prompt the player to enter a guess.
// 	•	The player should receive feedback indicating whether their guess is too high, too low, or correct.
// 	•	The game should keep track of the number of attempts the player makes.
// 	•	The player should have the option to play again after guessing correctly.
// 	•	The game should handle invalid inputs gracefully (e.g., non-numeric entries).

use rand::Rng;
use std::cmp::Ordering;

const ATTEMPTS: u32 = 3;

fn main() {
    println!("Hello Stanger, Kindly Tell Us your Name");

    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Hello {name}, Welcome To Guessing Game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=1000);
        let mut attempts_remaining = ATTEMPTS;

        println!("\n--- New Game ---");
        println!("I'm thinking of a number between 1 and 1000.");
        println!("You have {attempts_remaining} attempts.");

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ Please type a valid number!");
                    continue;
                }
            };

            println!("You guessed: {}", guess);
            attempts_remaining -= 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("🎉 You win!");
                    break;
                }
            }

            if attempts_remaining == 0 {
                println!("💀 You ran out of attempts! The secret number was: {secret_number}");
                break;
            } else {
                println!("Attempts remaining: {attempts_remaining}");
            }
        }

        println!("\nWould you like to play again? (y/n)");
        let mut play_again = String::new();
        std::io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" && play_again.trim().to_lowercase() != "yes" {
            println!("Thanks for playing, {name}! Goodbye.");
            break;
        }
    }
}

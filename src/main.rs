use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);

    let mut guess_count:u8 = 0;
    let max_guess_count:u8 = 3;

    println!("=================");
    println!("Guessing Game");
    println!("=================");
    println!(" ");

    loop {
        print!("Input your guess (1 - 10): ");
        std::io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess).expect("Failed to read input");
        
        guess_count += 1;

        // let guess_number:i32 = guess.trim().parse().expect("Please input a number: ");
        let guess_number: u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a valid number.");
                continue;
            }
        };
    
        // println!("Your input is: {guess_number}.");
        // println!("Secret number is : {secret}.");

        // println!(" You have guessed : {guess_count} times.");
    
        if guess_number > secret {
            println!("'Too high'");
            if guess_count == max_guess_count{
                println!("You've guessed {guess_count} times.");
                break;
            } else {
                println!("Remaining {} guess.", {max_guess_count - guess_count});
            }

        } else if guess_number < secret {
            println!("'Too low'");
            if guess_count == max_guess_count{
                println!("You've guessed {guess_count} times.");
                break;
            } else {
                println!("Remaining {} guess.", {max_guess_count - guess_count});
            }
        } else {
            println!("Correct, You Win!");
            break;
        }
    }

    println!("=================");
    println!("Game Over");
    println!("=================");

}

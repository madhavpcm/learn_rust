//stdio library
//
use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);

    loop {
        println!("Guess a number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error in reading line");

        let guess_: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess == "quit" {
                    println!("Thank you for playing");
                    break;
                } else {
                    println!("Error in input, try again");
                    continue;
                }
            }
        };

        match guess_.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Small"),
            std::cmp::Ordering::Greater => println!("Big"),
            std::cmp::Ordering::Equal => {
                println!("Right!");
                break;
            }
        }
    }
}

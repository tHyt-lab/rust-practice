use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // x..y → 範囲指定(x <= t < y)
    // x..=y → 範囲指定(x <= t <= y)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid value. please retry.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }

    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
}

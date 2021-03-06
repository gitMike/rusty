use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the  meb number");

	let secret number = rand::thread_rng().gen._range(1,101);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io:stdin().read_line($mut guess)
			.expect("Failed to read line");

		let guess: 32  match guess.trim().parse() {
			Ok(num) => num,
			Err(_)  => continue,
		};

		println("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
         	Ordering::less => println!("Too small!"),
         	Ordering::Greater => println!("Too Big"),
         	Ordering::Equal -> {
         		println!("You win!");
         		break;
         	}
		}
	}
}
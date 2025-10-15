use std::io;

fn main(){
	println!("lets play a guessing game");

	println!("please input your guess");

	let mut guess: String = String :: new();

	io::stdin(): Stdin

		.read_line(but:&mut guess)

		.expect(msg:failed to read line);


	println!("you guessed: {:?}", guess );
};
use std::{io, run, uint, os, path, libc ,str};
use std::rand;
use std::rand::Rng;

static LOW:int = 0;
static HIGH:int = 3;


fn load(filename: ~str) -> ~[~str] {
	let read: Result<@Reader, ~str>;
	read = io::file_reader(~path::Path(filename));

	if read.is_ok() {
	let file = read.unwrap();
	return file.read_lines();
	}
 
	println(fmt!("Error in reading the file: %?", read.unwrap_err()));
	return ~[];
}



fn main() {
        let mut rng = rand::task_rng();
	let dictionary: ~[~str] = load(~"words.txt");
	let max_size: uint = dictionary.len();
        let n: uint = rng.gen_integer_range(0u, max_size);
	let mut word: ~str = dictionary[n];
	println(fmt!("%?", n));
	println(fmt!("%?",max_size));	
	println(word);        
	let mut guesses :~[char] = "".iter().collect();
        let line = io::stdin().read_line();
	let guess_char: char = line.char_at(0);
	println(fmt!("%? guessed: ", guess_char));
	let mut has_char: bool = word.contains_char(guess_char);
	println(fmt!("%?", has_char));	
}


use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn is_palindrome(word: &String) {

    let reversed_word = word.chars().rev().collect::<String>();

    if word == &reversed_word {
	println!("{}", word);
    }
}
fn combination(mut line_to: Vec<String>, line_from: Vec<String>, mut index: u32, cap: u32) {

    index += 1;
    let mut added_words = Vec::new();
    for line1 in &line_to {
	//println!("{}", line);
	for line2 in &line_from {
	    let combination = line1.to_owned() + line2;
	    
	    is_palindrome(&combination);
	    added_words.push(combination);
	    
	}
    }
    line_to.append(&mut added_words);
    
    if index < cap {
	combination(line_to, line_from, index);
    }
}


fn main() -> io::Result<()> {
    let file = File::open("data/words")?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();
    let mut words2 = Vec::new();
    for line in reader.lines() {
	let l = line?;
	words.push(l);
    }
    words2 = words.clone();
//    for line in &words {
	//println!("{}", line);
//	for line2 in &words {
//	    let combination = line.to_owned() + line2;
//	    words2.push(combination);
	//    is_palindrome(combination);
//	}
   // }
    combination(words, words2, 0, 8);
    Ok(())
}

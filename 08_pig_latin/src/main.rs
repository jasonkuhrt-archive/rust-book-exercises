mod pig_latin;

// Convert strings to pig latin.
// Keep in mind the details about UTF-8 encoding!
//
// 1. The first consonant of each word is moved to the end of the word and “ay” is added
//    so “first” becomes “irst-fay.”
//
// 2. Words that start with a vowel have “hay” added to the end instead
//    (“apple” becomes “apple-hay”).

fn main() {
    let default_sentence = String::from("The quick brown fox jumps over the lazy dog");
    println!("Welcome to the Pig Latin Translator!");
    println!("Enter a sentence you would like transalted into Pig Latin");
    println!("Enter nothing to try with \"{}\"", default_sentence);
    let sentence = get_input().unwrap_or(default_sentence);
    let sentence_in_piglatin = pig_latin::translate_sentence(&sentence);
    println!("{}", sentence_in_piglatin);
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input == "" {
        Some(input)
    } else {
        None
    }
}

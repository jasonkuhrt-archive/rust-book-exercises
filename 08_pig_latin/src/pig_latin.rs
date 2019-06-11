// TODO can we do do piglatin with non-english languages?
// If so the efforts around utf8 in this module would be
// worthwhile.

pub enum PigLatin {
  Vowel,
  Consonant,
  Untranslatable,
}

pub fn translate_sentence(sentence: &str) -> String {
  sentence
    .split_whitespace()
    .map(translate_word)
    .collect::<Vec<String>>()
    .join(&String::from(" ").as_str())
}

pub fn translate_word(word: &str) -> String {
  match parse(&word) {
    PigLatin::Vowel => translate_vowel_word(&word),
    PigLatin::Consonant => translate_consonant_word(&word),
    PigLatin::Untranslatable => format!("{}", word),
  }
}

fn translate_vowel_word(word: &str) -> String {
  format!("{}-hay", &word)
}

fn translate_consonant_word(word: &str) -> String {
  // use word.chars to handle utf8
  let chars: Vec<char> = word.chars().collect();
  let (first, rest) = chars.split_at(1);
  let rest_s: String = rest.into_iter().collect();
  let first_s: String = first.into_iter().collect();
  format!("{}-{}ay", &rest_s, &first_s)
}

fn parse(word: &str) -> PigLatin {
  // TODO move vowels to static module constant??
  let vowels = String::from("AEIOUaeiou");
  // use word.chars to handle utf8
  let chars: String = word.chars().collect();
  if chars.len() > 2 {
    if vowels.contains(&chars[0..1]) {
      PigLatin::Vowel
    } else {
      PigLatin::Consonant
    }
  } else {
    PigLatin::Untranslatable
  }
}

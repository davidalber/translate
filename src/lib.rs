#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate toml;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    static ref PIRATE_WORD_MAP: HashMap<String, Vec<String>> = {
        let mut f = File::open("data/pirate.toml").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        let config: HashMap<String, Vec<String>> = toml::from_str(&contents).unwrap_or(HashMap::new());
        config
    };
}

#[derive(Debug, PartialEq)]
enum Casing {
    Lowercase,
    Titlecase,
    Uppercase,
}

impl Casing {
    fn classify(word: &str) -> Casing {
        match word.chars().nth(0).unwrap_or('a').is_lowercase() {
            true => Casing::Lowercase,
            false => {
                match word.chars().nth(1).unwrap_or('a').is_lowercase() {
                    true => Casing::Titlecase,
                    false => Casing::Uppercase,
                }
            }
        }
    }

    fn apply(&self, word: &str) -> String {
        match self {
            Casing::Lowercase => word.to_lowercase(),
            Casing::Titlecase => word.chars().enumerate().map(|(i, c)| {
                match i == 0 {
                    true => c.to_uppercase().to_string(),
                    false => c.to_lowercase().to_string(),
                }
            }).collect(),
            Casing::Uppercase => word.to_uppercase(),
        }
    }
}

fn split_punctuation(word: &str) -> (&str, &str) {
    let len = word.len();
    match word.chars().last().unwrap_or('a').is_alphanumeric() {
        true => (word, ""),
        false => (&word[0..len-1], &word[len-1..]),
    }
}

fn get_pirate_translation(word: &str) -> &str {
    match PIRATE_WORD_MAP.get::<str>(word) {
        Some(words) => {
            // Randomly select one of the possible words.
            let mut rng = thread_rng();
            let n: usize = rng.gen_range(0, words.len());
            &words[n]
        },
        None => word,
    }
}

fn translate_word(word: &str) -> String {
    let (word, trailing_punctucation) = split_punctuation(word);
    let case = Casing::classify(word);
    case.apply(get_pirate_translation(&word.to_lowercase())) + trailing_punctucation
}

fn translate(text: &str) -> String {
    let mut translated: Vec<String> = Vec::new();
    for word in text.split(" ") {
        translated.push(translate_word(word));
    }

    translated.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_casing_classify() {
        assert_eq!(Casing::classify("foo"), Casing::Lowercase);
        assert_eq!(Casing::classify("Foo"), Casing::Titlecase);
        assert_eq!(Casing::classify("FOo"), Casing::Uppercase);
        assert_eq!(Casing::classify("FOO"), Casing::Uppercase);
        assert_eq!(Casing::classify(""), Casing::Lowercase);
        assert_eq!(Casing::classify("a"), Casing::Lowercase);
        assert_eq!(Casing::classify("A"), Casing::Titlecase);
    }

    #[test]
    fn test_casing_apply() {
        let case = Casing::Lowercase;
        assert_eq!(case.apply("foo"), "foo");
        assert_eq!(case.apply("Foo"), "foo");
        assert_eq!(case.apply("FOo"), "foo");
        assert_eq!(case.apply("FOO"), "foo");
        assert_eq!(case.apply(""), "");

        let case = Casing::Titlecase;
        assert_eq!(case.apply("foo"), "Foo");
        assert_eq!(case.apply("Foo"), "Foo");
        assert_eq!(case.apply("FOo"), "Foo");
        assert_eq!(case.apply("FOO"), "Foo");
        assert_eq!(case.apply(""), "");

        let case = Casing::Uppercase;
        assert_eq!(case.apply("foo"), "FOO");
        assert_eq!(case.apply("Foo"), "FOO");
        assert_eq!(case.apply("FOo"), "FOO");
        assert_eq!(case.apply("FOO"), "FOO");
        assert_eq!(case.apply(""), "");
    }

    #[test]
    fn test_split_punctuation() {
        let (w, p) = split_punctuation("foo");
        assert_eq!(w, "foo");
        assert_eq!(p, "");

        let (w, p) = split_punctuation("foo,");
        assert_eq!(w, "foo");
        assert_eq!(p, ",");

        let (w, p) = split_punctuation("");
        assert_eq!(w, "");
        assert_eq!(p, "");
    }

    #[test]
    fn test_get_pirate_translation() {
        let expected = vec!["arr", "aye", "yar", "yarr"];
        for _ in 0..100 {
            let translated = get_pirate_translation("yes");
            assert!(expected.contains(&translated));
        }
    }

    #[test]
    fn test_translate() {
        println!("{:?}", translate("You are great. Yes, you are."));
    }
}

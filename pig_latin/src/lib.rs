extern crate regex;

pub mod pig_latin {
    use regex::Regex;

    pub fn translate(word: &str) -> String {
        let re = Regex::new(r"[aeiou]").unwrap();

        for c in re.find_iter(word) {
            if c.start() == 0 {
                return format!("{}-hay", word);
            } else {
                return format!("{}-{}ay", &word[c.start()..], &word[..c.start()]);
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use pig_latin;

    #[test]
    fn first_consonant() {
        assert_eq!(pig_latin::translate("first"), String::from("irst-fay"));
    }

    #[test]
    fn first_vowel() {
        assert_eq!(pig_latin::translate("apple"), String::from("apple-hay"));
    }

    #[test]
    fn empty_word() {
        assert_eq!(pig_latin::translate(""), String::new());
    }
}

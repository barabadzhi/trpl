extern crate regex;

pub mod pig_latin {
    use regex::Regex;

    pub fn translate(text: &str) -> String {
        let re = Regex::new(r"[aeiou]").unwrap();
        let mut pig_latin = Vec::with_capacity(text.len());

        for word in text.split_whitespace() {
            match re.find(word) {
                Some(mat) => {
                    pig_latin.push(if mat.start() == 0 {
                        format!("{}-hay", word)
                    } else {
                        format!("{}-{}ay", &word[mat.start()..], &word[..mat.start()])
                    });
                }
                None => {
                    pig_latin.push(String::from(word));
                }
            }
        }

        pig_latin.join(" ")
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

    #[test]
    fn multiple_words() {
        assert_eq!(
            pig_latin::translate("first apple"),
            String::from("irst-fay apple-hay")
        );
    }
}

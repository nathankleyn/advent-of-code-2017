use std::collections::HashSet;

#[allow(dead_code)]
fn day_4(passphrase: &str, allow_anagrams: bool) -> bool {
    passphrase.split_whitespace().fold(Some(HashSet::new()), |acc, word| {
        match acc {
            None => None,
            Some(seen_words) => {
                let parsed_word: String = if allow_anagrams {
                    word.to_string()
                } else {
                    let mut chars = word.chars().collect::<Vec<char>>();
                    chars.sort();
                    chars.iter().collect()
                };

                if seen_words.get(&parsed_word).is_some() {
                    return None;
                }

                let mut new_seen_words = seen_words.clone();
                new_seen_words.insert(parsed_word);
                Some(new_seen_words)
            }
        }
    }).is_some()
}

#[cfg(test)]
mod tests {
    use day_4;

    #[test]
    fn day_4_part_1_examples() {
        assert_eq!(day_4("aa bb cc dd ee", true), true);
        assert_eq!(day_4("aa bb cc dd aa", true), false);
        assert_eq!(day_4("aa bb cc dd aaa", true), true);
    }

    #[test]
    fn day_4_part_2_examples() {
        assert_eq!(day_4("abcde fghij", false), true);
        assert_eq!(day_4("abcde xyz ecdab", false), false);
        assert_eq!(day_4("a ab abc abd abf abj", false), true);
        assert_eq!(day_4("iiii oiii ooii oooi oooo", false), true);
        assert_eq!(day_4("oiii ioii iioi iiio", false), false);
    }

    #[test]
    fn day_4_part_1_test_input() {
        let input = include_str!("input");
        assert_eq!(input.lines().filter(|passphrase| day_4(passphrase, true)).count(), 451);
    }

    #[test]
    fn day_4_part_2_test_input() {
        let input = include_str!("input");
        assert_eq!(input.lines().filter(|passphrase| day_4(passphrase, false)).count(), 223);
    }
}

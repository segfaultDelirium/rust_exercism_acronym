use unicode_segmentation::UnicodeSegmentation;

fn get_first_letter_as_string(s: &str) -> String {
    s.chars().nth(0).unwrap().to_string()
}

pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");
    let words: Vec<&str> = phrase.unicode_words().into_iter().collect();

    // we need to account for case:
    // assert_eq!(acronym::abbreviate("HyperText Markup Language"), "HTML");
    // TODO
    let first_or_uppercase_chars: Vec<char> = words
        .into_iter()
        .map(|s| s.trim_matches('_').to_string())
        .map(|s| {
            if s == s.to_uppercase() {
                get_first_letter_as_string(&s)
            } else {
                s
            }
        })
        .map(|s| {
            s.chars()
                .enumerate()
                .filter(|(i, c)| *i == 0 || c.is_uppercase())
                .map(|(i, c)| c)
                .collect::<Vec<char>>()
        })
        .flatten()
        .collect();
    let res = first_or_uppercase_chars
        .into_iter()
        .map(|c| c.to_string().to_uppercase())
        .reduce(|acc, c| format!("{}{}", acc, c))
        .unwrap();
    res

    // "".to_string()
}

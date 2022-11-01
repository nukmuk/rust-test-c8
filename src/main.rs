fn main() {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

    let mut words = text.split_whitespace();

    for mut word in words {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y', 'ä', 'ö'];
        let suffix1 = "ay";
        let suffix2 = "hay";

        let mut new_word = String::from(word);

        let first_char = &word.chars().nth(0).unwrap();

        if vowels.contains(first_char) {
            new_word.push('-');
            new_word.push(*first_char);
            new_word.push_str(suffix1);
        } else {
            word = "moro";
        }
    }

    println!("{:?}", words);
}

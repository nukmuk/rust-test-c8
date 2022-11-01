fn main() {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

    let mut words = text.split_whitespace();
    let mut new_words = Vec::new();

    for word in words {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y', 'ä', 'ö'];
        let suffix1 = "ay";
        let suffix2 = "hay";

        let mut new_word = String::new();

        let first_char = &word.chars().nth(0).unwrap();

        if vowels.contains(first_char) {
            new_word.push_str(&word);
            new_word.push('-');
            new_word.push_str(suffix2)
        } else {
            new_word.push_str(&word[1..]);
            new_word.push('-');
            new_word.push(*first_char);
            new_word.push_str(suffix1);
        }

        new_words.push(new_word);
    }

    let new_string = new_words.join(" ");

    // println!("{:?}", new_words);
    println!("{:?}", new_string);
}

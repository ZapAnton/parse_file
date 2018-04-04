use std::{env, collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn read_words_from_file(data_file: &File) -> HashMap<String, i32> {
    let mut words: HashMap<String, i32> = HashMap::new();

    for line in BufReader::new(data_file).lines() {
        let chars_to_remove = ",.?!\\\"=;:+-_()\'*|~`/";

        let line: String = line.unwrap()
            .to_lowercase()
            .chars()
            .filter(|&c| !chars_to_remove.contains(c))
            .collect();

        let line_words = line.split_whitespace();

        for word in line_words {
            let word_count = words.entry(word.to_string()).or_insert(0);

            *word_count += 1;
        }
    }

    words
}

fn find_most_frequent_word(words: &HashMap<String, i32>) -> (String, i32) {
    let mut max_entry = (String::new(), 0);

    for (key, value) in words {
        if value > &max_entry.1 {
            max_entry = (key.to_string(), *value);
        }
    }

    max_entry
}

fn get_parse_file_name() -> String {
    env::args()
        .nth(1)
        .expect("You need to provide path to data file to parse!")
}

fn main() {
    let file_path = get_parse_file_name();

    let data_file = File::open(file_path).expect("Error opening data file!");

    let words = read_words_from_file(&data_file);

    let max_entry = find_most_frequent_word(&words);

    println!(
        "The most frequent word is '{}'. The data file contains it {} times",
        max_entry.0, max_entry.1
    );
}

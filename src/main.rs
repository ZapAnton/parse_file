use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() {
    let data_file = File::open("data/data2.txt").expect("Error opening data file!");

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

    let mut max_entry = ("", 0);

    for (key, value) in &words {
        if value > &max_entry.1 {
            max_entry = (key, *value);
        }
    }

    println!("The most frequent word is {}. The data file contains it {} times", max_entry.0, max_entry.1);
}

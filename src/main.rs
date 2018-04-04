use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let data_file = File::open("data/data1.txt").expect("Error opening data file!");

    for line in BufReader::new(data_file).lines() {
        println!("{}", line.unwrap());
    }
}

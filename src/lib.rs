use std::fs;


pub struct Config {
    pub file_path: String,
    pub column: usize,
    pub delimiter: char,
}

impl Config {
    pub fn new(file_path: String, delimiter: char, column: usize) -> Config {
        Config {
            file_path: file_path,
            delimiter: delimiter,
            column: column,
        }
    }
}

pub fn cut(config: Config) {
    let mut lines: Vec<String> = vec!();
    let mut words = vec!();

    let file_content: String = fs::read_to_string(config.file_path).unwrap();

    for line in file_content.split('\n') {
        lines.push(line.to_string());
    }

    for word in lines[0].split(config.delimiter) {
        words.push(word.to_string());
    }

    println!("{}", &words[config.column]);
    
}

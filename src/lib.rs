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
            column: column
        }
    }
}

pub fn cut(config: Config) {

    let mut container: Vec<Vec<&str>> = vec!();

    let mut file_content: String = fs::read_to_string(config.file_path).unwrap();
    let content_to_vector: Vec<_> = file_content.split('\n').collect();


    for s in &content_to_vector {
        let mut v: Vec<&str> = vec!();
        v.push(s);

        container.push(v);
    }
    
    for v in &container {
        //println!("{:?}", &v);
        for s in v {
            let spl: Vec<&str> = s.split(config.delimiter).collect();
            println!("{:?}", &spl[config.column]);
        }
    }
 
}

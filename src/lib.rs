use std::fs;
use std::error::Error;
//use std::env::args;

pub struct Config {
    pub file_path: String,
    pub column: usize,
    pub delimiter: char,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {

        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("cant find file path"),
        };

        let delimiter = match args.next() {
            Some(arg) => arg.chars().next().unwrap(),
            None => return Err("delimiter?"),
        };

        let column = match args.next() {
            Some(arg) => arg,
            None => return Err("column?"),
        };

        Ok(Config { file_path, delimiter:delimiter, column:column.parse::<usize>().unwrap()})

    }
}

pub fn cut(config: Config) -> Result<(), Box<dyn Error>> {

    let mut container: Vec<Vec<&str>> = vec!();

    let file_content: String = fs::read_to_string(&config.file_path).unwrap();
    let content_to_vector: Vec<_> = file_content.split('\n').collect();

    for s in &content_to_vector {
        let mut v: Vec<&str> = vec!();
        v.push(s);

        container.push(v);
    }
    
    for v in &container {
        for s in v {
            let spl: Vec<&str> = s.split(config.delimiter).collect();
            println!("{:?}", &spl[config.column]);
        }
    } 
    Ok(())
}

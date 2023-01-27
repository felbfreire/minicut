use minicut:: {Config, cut};


fn main() {
    let config: Config = Config::new(String::from("poem.txt"), ' ', 0);

    cut(config)

    //println!("cut file {}, at column {}", config.file_path, config.column);
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    get_text_from_file("hello.txt");
}

fn get_text_from_file(path: &str) {
    let file = open_file(path);
    let text = read_text_file(path, &file);

    println!("{}", &text);
}

fn open_file(path: &str) -> File {
    let path = Path::new(path);
    let display = path.display();

    match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    }
}

fn read_text_file(path: &str, mut file: &File) -> String {
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path, why),
        Ok(_) => print!("Successfully received file\n"),
    }
    s
}

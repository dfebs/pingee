use std::fs;

mod text_tools;
use text_tools::file_handler::get_text_from_file;

fn main() {
    get_text_from_file("hello.txt");
    let data: Vec<u8> = match fs::read("gpru.png") {
        Err(why) => panic!("{}", why),
        Ok(bytes) => bytes,
    };

    let first_byte = data[0].to_string();

    // How do I iterate through all the bytes and print them?
    // How do I get the size of the byte array?
    // How does that compare to the computer's interpretation of the file size?
    // What does it look like when all the things are printed?
    // Find some time to look into the png spec.

    println!("{}", first_byte);
}

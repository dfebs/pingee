use pingee::png_tools::png_tools::find_sequences;
use std::fs;

fn main() {
    let bytes: Vec<u8> = match fs::read("gpru.png") {
        Err(why) => panic!("{}", why),
        Ok(bytes) => bytes,
    };

    let sequences = find_sequences(&bytes, &vec![137, 80, 78, 71, 13, 10, 26, 10]);
    for item in sequences.iter() {
        println!("{}", item);
    }
}

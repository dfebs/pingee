use pingee::png_model::png_model::*;
use pingee::png_tools::png_tools::*;
use std::fs;

fn main() {
    let bytes: Vec<u8> = match fs::read("gpru_lean.png") {
        Err(why) => panic!("{}", why),
        Ok(bytes) => bytes,
    };

    println!("Length of bytes: {}", &bytes.len());

    let chunks = get_chunks(&bytes);
    println!("{:#?}", chunks);
    let png = Png::new(&bytes);
    println!("{:#?}", png);
}

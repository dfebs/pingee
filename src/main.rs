use pingee::png_model::png_model::*;
use pingee::png_tools::png_tools::*;
use std::fs;

fn main() {
    let bytes: Vec<u8> = match fs::read("gpru.png") {
        Err(why) => panic!("{}", why),
        Ok(bytes) => bytes,
    };

    println!("iend");
    print_sequences(&bytes, &IEND_BYTES, 4);

    println!("idat");
    print_sequences(&bytes, &IDAT_BYTES, 4);

    println!("idhr");
    // Four core bytes plus the rest of the header info
    print_sequences(&bytes, &IDHR_BYTES, 20);

    let png = Png::new(&bytes);
    println!("{:#?}", png);
}

use pingee::png_tools::png_tools::*;
use std::fs;

const IDHR_BYTES: [u8; 4] = [0x49, 0x48, 0x44, 0x52];
const IEND_BYTES: [u8; 4] = [0x49, 0x45, 0x4e, 0x44];
const IDAT_BYTES: [u8; 4] = [0x49, 0x44, 0x41, 0x54];

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
    print_sequences(&bytes, &IDHR_BYTES, 17);
}

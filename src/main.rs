use pingee::png_tools::png_tools::find_sequences;
use std::fs;

fn main() {
    let bytes: Vec<u8> = match fs::read("gpru.png") {
        Err(why) => panic!("{}", why),
        Ok(bytes) => bytes,
    };

    println!("Bytes length: {}", bytes.len());

    let beginning = find_sequences(&bytes, &vec![137, 80, 78, 71, 13, 10, 26, 10]);
    for item in beginning.iter() {
        println!("Beginning bytes located at {}", item);
    }

    let iend = find_sequences(&bytes, &vec![0x49, 0x45, 0x4e, 0x44]);
    for &item in iend.iter() {
        println!("IEND located at {}", item);
        let slice = &bytes[item..];
        println!("Last of bits starting at IEND: {:02X?}", slice)
        // TODO: What are those last 4 bytes?
    }
}

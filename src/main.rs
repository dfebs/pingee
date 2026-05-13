use pingee::png_model::png_model::*;
use std::env;
use std::fs;

// Sample minifb code
use minifb::{Key, Window, WindowOptions};

fn main() {
    let file_name: Vec<String> = env::args().collect();
    let bytes: Vec<u8> = match fs::read(&file_name[1]) {
        Err(why) => panic!("{}", why),
        Ok(bytes) => bytes,
    };

    let png = Png::new(&bytes);
    println!("decompressed data: {:#?}", png.decompressed_img_data);
    println!("reconstructed data: {:#?}", png.reconstructed_img_data);

    let image_width: usize = png.header.width;
    let image_height: usize = png.header.height;

    let window_width: usize = if image_width <= 10 {
        image_width * 100
    } else {
        image_width * 10
    };
    let window_height: usize = if image_height <= 10 {
        image_height * 100
    } else {
        image_height * 10
    };

    // this may not work with indexed yet
    let buffer: Vec<u32> = png
        .reconstructed_img_data
        .chunks(3)
        .map(|color| (color[0] as u32) << 16 | (color[1] as u32) << 8 | color[2] as u32)
        .collect();

    let mut window = Window::new(
        "Test - ESC to exit",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, image_width, image_height)
            .unwrap();
    }
}

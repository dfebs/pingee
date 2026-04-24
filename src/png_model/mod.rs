pub mod png_model {
    use crate::png_tools::png_tools::*;

    #[derive(Debug)]
    pub struct Png {
        // Found in image header
        pub width: u32,
        pub height: u32,
        pub bit_depth: u8,
        pub color_type: u8,
        pub compression_method: u8,
        pub filter_method: u8,
        pub interlace_method: u8,
    }

    impl Png {
        pub fn new(bytes: &[u8]) -> Self {
            let idhr = find_sequences(bytes, &IDHR_BYTES)[0];

            // TODO: This is probably good as a function
            let width = &bytes[idhr + 4..idhr + 8];
            let mut width_arr = [0u8; 4];
            width_arr.copy_from_slice(width);
            let width = u32::from_be_bytes(width_arr);

            let height = &bytes[idhr + 8..idhr + 12];
            let mut height_arr = [0u8; 4];
            height_arr.copy_from_slice(height);
            let height = u32::from_be_bytes(height_arr);

            let mut remaining_bytes = bytes[idhr + 12..idhr + 17].iter();
            let bit_depth = *remaining_bytes.next().unwrap();
            let color_type = *remaining_bytes.next().unwrap();
            let compression_method = *remaining_bytes.next().unwrap();
            let filter_method = *remaining_bytes.next().unwrap();
            let interlace_method = *remaining_bytes.next().unwrap();

            Png {
                width,
                height,
                bit_depth,
                color_type,
                compression_method,
                filter_method,
                interlace_method,
            }
        }
    }

    // PLTE
    struct Palette {
        red: u8,
        green: u8,
        blue: u8,
    }

    // IDAT
    struct ImageData {}
}

#[cfg(test)]
mod tests {
    use super::png_model::*;
    use std::fs;
    fn get_test_file(file_name: &str) -> Vec<u8> {
        match fs::read("gpru.png") {
            Err(why) => panic!("{}", why),
            Ok(bytes) => bytes,
        }
    }

    #[test]
    fn verify_image_header() {
        let bytes = get_test_file("grpu.png");
        let png = Png::new(&bytes);
        assert_eq!(png.width, 2);
    }
}

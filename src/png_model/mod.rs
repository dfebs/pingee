pub mod chunks;
pub mod png_model {
    use crate::{png_model::chunks::chunks::*, png_tools::png_tools::*};

    #[derive(Debug)]
    pub struct Png {
        // Found in image header
        pub header: Header,
        pub palette: Option<Vec<Color>>,
    }

    impl Png {
        pub fn new(bytes: &[u8]) -> Self {
            let chunks = get_chunks(&bytes);
            let header = Self::retrieve_headers(chunks.get("IHDR"));
            let palette = Self::retrieve_palette(chunks.get("PLTE"));

            Png { header, palette }
        }

        fn retrieve_headers(chunk: Option<&Chunk>) -> Header {
            let bytes = &chunk.unwrap().data;
            let width = extract_u32(bytes, 0);
            let height = extract_u32(bytes, 4);

            let mut remaining_bytes = bytes[8..].iter();
            let bit_depth = *remaining_bytes.next().unwrap();
            let color_type = *remaining_bytes.next().unwrap();
            let compression_method = *remaining_bytes.next().unwrap();
            let filter_method = *remaining_bytes.next().unwrap();
            let interlace_method = *remaining_bytes.next().unwrap();

            Header {
                width,
                height,
                bit_depth,
                color_type,
                compression_method,
                filter_method,
                interlace_method,
            }
        }

        fn retrieve_palette(chunk: Option<&Chunk>) -> Option<Vec<Color>> {
            let palette = &chunk?.data;
            if palette.len() % 3 != 0 {
                panic!("Palette is not divisible by 3. It may be corrupted.");
            }
            // Figure out how to chunk up the palette into groups of 3
            // let colors = palette.as_chunks(3);
            let colors: Vec<Color> = palette
                .chunks_exact(3)
                .map(|color| Color(color[0], color[1], color[2]))
                .collect();

            Some(colors)
        }

        fn retrieve_image_data() {
            todo!();
        }
    }
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
        assert_eq!(png.header.width, 2);
    }
}

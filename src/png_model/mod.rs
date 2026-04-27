pub mod chunks;
pub mod png_model {
    use crate::{png_model::chunks::chunks::*, png_tools::png_tools::*};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Png {
        // Found in image header
        pub header: Header,
        pub palette: Option<Vec<Color>>,
        pub img_data: Vec<u8>,
    }

    impl Png {
        pub fn new(bytes: &[u8]) -> Self {
            let chunks = get_chunks(&bytes);
            let header = Self::retrieve_headers(chunks.get("IHDR"));
            let palette = Self::retrieve_palette(chunks.get("PLTE"));
            let img_data = Self::retrieve_image_data(&chunks);

            Png {
                header,
                palette,
                img_data,
            }
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

            let colors: Vec<Color> = palette
                .chunks_exact(3)
                .map(|color| Color(color[0], color[1], color[2]))
                .collect();

            Some(colors)
        }

        fn retrieve_image_data(chunks: &HashMap<String, Chunk>) -> Vec<u8> {
            chunks
                .iter()
                .filter(|(key, _)| key.contains("IDAT"))
                .map(|(_, value)| value.data.to_owned())
                .flatten()
                .collect()
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

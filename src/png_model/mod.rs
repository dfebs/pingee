pub mod chunks;
pub mod png_model {
    use crate::{png_model::chunks::chunks::*, png_tools::png_tools::*};

    #[derive(Debug)]
    pub struct Png {
        // Found in image header
        pub header: Header,
    }

    impl Png {
        pub fn new(bytes: &[u8]) -> Self {
            let raw_chunks = get_chunks(&bytes);
            let header = Self::retrieve_headers(&raw_chunks.get("IHDR").unwrap().data);
            Png { header }
        }

        fn retrieve_headers(bytes: &[u8]) -> Header {
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

        fn retrieve_image_data() {
            todo!();
        }

        fn retrieve_palette() {
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

pub mod chunks;
pub mod test;
pub mod png_model {
    use crate::{png_model::chunks::chunks::*, png_tools::png_tools::*};
    use flate2::bufread::ZlibDecoder;
    use std::collections::HashMap;
    use std::io::Read;

    #[derive(Debug)]
    pub struct Png {
        // Found in image header
        pub header: Header,
        pub palette: Option<Vec<Color>>,
        pub decompressed_img_data: Vec<u8>,
        pub reconstructed_img_data: Vec<u8>,
        pub raw_img_data: Vec<u8>,
    }

    impl Png {
        pub fn new(bytes: &[u8]) -> Self {
            let chunks = get_chunks(&bytes);
            let header = Self::retrieve_headers(chunks.get("IHDR"));
            let palette = Self::retrieve_palette(chunks.get("PLTE"));

            let raw_img_data = Self::retrieve_image_data(&chunks);
            let mut decompressed_img_data = Vec::new();
            ZlibDecoder::new(&raw_img_data[..])
                .read_to_end(&mut decompressed_img_data)
                .expect("Failed to decompress image data");

            let reconstructed_img_data =
                Self::reconstruct_image_data(&decompressed_img_data, &header);

            Png {
                header,
                palette,
                raw_img_data,
                decompressed_img_data,
                reconstructed_img_data,
            }
        }

        fn retrieve_headers(chunk: Option<&Chunk>) -> Header {
            let bytes = &chunk.unwrap().data;
            let width = extract_u32(bytes, 0) as usize;
            let height = extract_u32(bytes, 4) as usize;

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

        fn reconstruct_image_data(data: &[u8], header: &Header) -> Vec<u8> {
            let mut current_filter: u8 = 0;
            let mut final_colors: Vec<u8> = Vec::new();

            for (index, &byte) in data.iter().enumerate() {
                if index % (header.scanline_length() + 1) == 0 {
                    current_filter = byte;
                } else {
                    let filtered_byte =
                        Self::filter_byte(&final_colors, byte, header, current_filter);
                    final_colors.push(filtered_byte);
                }
            }

            final_colors
        }

        fn get_a(buffer: &[u8], header: &Header) -> u8 {
            let receiving_byte_location = buffer
                .len()
                .checked_sub(header.samples_per_pixel() * header.sample_size());
            let current_pos = buffer.len() % header.scanline_length();

            match receiving_byte_location {
                None => 0,
                Some(location) => {
                    let mut output = 0;
                    if let Some(_) = (current_pos).checked_sub(location % header.scanline_length())
                    {
                        output = buffer[location];
                    }
                    output
                }
            }
        }

        fn get_b(buffer: &[u8], header: &Header) -> u8 {
            let receiving_byte_location = buffer.len().checked_sub(header.scanline_length());

            match receiving_byte_location {
                None => 0,
                Some(byte_location) => buffer[byte_location],
            }
        }

        fn get_c(buffer: &[u8], header: &Header) -> u8 {
            let receiving_byte_location = buffer.len().checked_sub(header.scanline_length());

            if receiving_byte_location == None {
                return 0;
            }

            let slice = &buffer[0..receiving_byte_location.unwrap()];
            let final_byte = Self::get_a(slice, header);
            final_byte
        }

        fn filter_byte(buffer: &[u8], byte: u8, header: &Header, filter: u8) -> u8 {
            match filter {
                // No Filter
                0 => byte,

                // Sub
                1 => {
                    let receiving_byte = Self::get_a(buffer, header);
                    byte.wrapping_add(receiving_byte)
                }

                // Up
                2 => {
                    let receiving_byte = Self::get_b(buffer, header);
                    byte.wrapping_add(receiving_byte)
                }

                // Average
                3 => {
                    let a = Self::get_a(buffer, header) as u16;
                    let b = Self::get_b(buffer, header) as u16;
                    let avg = (a + b) / 2;

                    byte.wrapping_add(avg as u8)
                }

                // Paeth
                4 => {
                    let a = Self::get_a(buffer, header) as i16;
                    let b = Self::get_b(buffer, header) as i16;
                    let c = Self::get_c(buffer, header) as i16;

                    let p = a + b - c;
                    let pa = (p - a).abs();
                    let pb = (p - b).abs();
                    let pc = (p - c).abs();

                    let result;
                    if pa <= pb && pa <= pc {
                        result = a as u8;
                    } else if pb <= pc {
                        result = b as u8;
                    } else {
                        result = c as u8;
                    }

                    byte.wrapping_add(result)
                }
                _ => panic!("Invalid filter provided"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::png_model::test::fixtures;

    use super::png_model::*;
    use std::fs;
    fn get_test_file(file_name: &str) -> Vec<u8> {
        match fs::read(file_name) {
            Err(why) => panic!("{}", why),
            Ok(bytes) => bytes,
        }
    }

    #[test]
    fn verify_image_header() {
        let bytes = get_test_file("gpru.png");
        let png = Png::new(&bytes);
        assert_eq!(png.header.width, 2);
    }

    #[test]
    fn verify_filter_1() {
        let bytes = get_test_file("filter_1_only.png");
        let png = Png::new(&bytes);
        assert_eq!(
            png.reconstructed_img_data,
            fixtures::fixtures::FILTER_1_ONLY
        );
    }

    #[test]
    fn verify_filter_2() {
        let bytes = get_test_file("filter_2_only.png");
        let png = Png::new(&bytes);
        assert_eq!(
            png.reconstructed_img_data,
            fixtures::fixtures::FILTER_2_ONLY
        );
    }

    #[test]
    fn verify_filter_3() {
        todo!();
    }

    #[test]
    fn verify_filter_4() {
        todo!();
    }
}

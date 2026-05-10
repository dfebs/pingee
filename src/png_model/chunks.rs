pub mod chunks {
    // IHDR
    #[derive(Debug)]
    pub struct Header {
        pub width: usize,
        pub height: usize,
        pub bit_depth: u8,
        pub color_type: u8,
        pub compression_method: u8,
        pub filter_method: u8,
        pub interlace_method: u8,
    }

    impl Header {
        pub fn samples_per_pixel(&self) -> usize {
            match &self.color_type {
                0 => 1, // Greyscale
                2 => 3, // Truecolor
                3 => 1, // Indexed
                4 => 2, // Greyscale alpha
                6 => 4, // Truecolor alpha
                _ => panic!("Image header has invalid color type"),
            }
        }

        pub fn sample_size(&self) -> usize {
            match self.bit_depth {
                8 => 1,
                16 => 2,
                _ => 1,
            }
        }

        pub fn scanline_length(&self) -> usize {
            self.sample_size() * self.samples_per_pixel() * self.width
        }
    }

    // PLTE
    #[derive(Debug)]
    pub struct Color(pub u8, pub u8, pub u8);

    // IDAT
    #[derive(Debug)]
    struct ImageData {}
}

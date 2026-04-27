pub mod chunks {
    // IHDR
    #[derive(Debug)]
    pub struct Header {
        pub width: u32,
        pub height: u32,
        pub bit_depth: u8,
        pub color_type: u8,
        pub compression_method: u8,
        pub filter_method: u8,
        pub interlace_method: u8,
    }

    // PLTE
    #[derive(Debug)]
    pub struct Color(pub u8, pub u8, pub u8);

    // IDAT
    #[derive(Debug)]
    struct ImageData {}
}

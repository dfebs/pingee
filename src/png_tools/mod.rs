pub mod png_tools {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Chunk {
        pub length: usize,
        pub data: Vec<u8>,
        pub crc: Vec<u8>,
    }

    pub fn get_chunks(bytes: &[u8]) -> HashMap<String, Chunk> {
        let mut current_byte: usize = 8; // First byte after signature
        let mut chunks = HashMap::new();
        let mut num_idat_chunks = 0;

        while current_byte < bytes.len() {
            let length = extract_u32(&bytes, current_byte) as usize;
            current_byte += 4;

            let chunk_type = match str::from_utf8(&bytes[current_byte..current_byte + 4]).unwrap() {
                "IDAT" => {
                    let s = format!("IDAT_{}", num_idat_chunks);
                    num_idat_chunks += 1;
                    s
                }
                s => s.to_owned(),
            };
            current_byte += 4;

            let data = bytes[current_byte..current_byte + length].to_vec();
            current_byte += length;

            let crc = bytes[current_byte..current_byte + 4].to_vec();
            current_byte += 4;

            let chunk = Chunk {
                length: length,
                data,
                crc,
            };

            chunks.insert(chunk_type, chunk);
        }

        chunks
    }

    pub fn extract_u32(bytes: &[u8], start: usize) -> u32 {
        let width = &bytes[start..start + 4];
        let mut width_arr = [0u8; 4];
        width_arr.copy_from_slice(width);
        u32::from_be_bytes(width_arr)
    }
}

#[cfg(test)]
mod tests {
    use super::png_tools::*;
}

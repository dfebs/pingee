pub mod png_tools {
    use std::{collections::HashMap, fmt::format};

    pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
    pub const IDHR_BYTES: [u8; 4] = [0x49, 0x48, 0x44, 0x52];
    pub const IEND_BYTES: [u8; 4] = [0x49, 0x45, 0x4e, 0x44];
    pub const IDAT_BYTES: [u8; 4] = [0x49, 0x44, 0x41, 0x54];
    pub const PLTE_BYTES: [u8; 4] = [0x50, 0x4c, 0x54, 0x45];

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

    #[derive(Debug)]
    pub struct Chunk {
        pub length: usize,
        pub data: Vec<u8>,
        pub crc: Vec<u8>,
    }

    pub fn print_sequences(bytes: &[u8], sequence: &[u8], offset: usize) {
        let sequences = find_sequences(bytes, sequence);
        for &item in sequences.iter() {
            let slice = &bytes[item..item + offset];
            println!("Chunk sequence: {:02X?}", slice)
        }
    }

    pub fn find_sequences(bytes: &[u8], sequence: &[u8]) -> Vec<usize> {
        if sequence.len() <= 1 || bytes.len() <= 1 {
            panic!("Sequence and bytes parameters must be at least 2 bytes long")
        }

        let mut sequence_locations: Vec<usize> = Vec::new();
        let mut sequence_progress: usize = 0;
        let mut sequence_beginning: usize = 0;

        for (i, byte) in bytes.iter().enumerate() {
            let mut expected_byte = &sequence[sequence_progress];
            if sequence_progress == sequence.len() - 1 && expected_byte == byte {
                sequence_locations.push(sequence_beginning);
                sequence_progress = 0;
            }

            if sequence_progress == 0 {
                sequence_beginning = i;
            }

            expected_byte = &sequence[sequence_progress];
            if byte == expected_byte {
                sequence_progress += 1;
            } else {
                sequence_progress = if &sequence[0] == byte { 1 } else { 0 };
                sequence_beginning = i;
            }
        }

        sequence_locations
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

    fn check_output(bytes: Vec<u8>, sequence: Vec<u8>, expected_output: Vec<usize>) {
        assert_eq!(find_sequences(&bytes, &sequence), expected_output)
    }

    // Happy path cases
    #[test]
    fn only_has_sequence() {
        check_output(vec![3, 4, 5], vec![3, 4, 5], vec![0]);
    }

    #[test]
    fn has_overlapping_sequences() {
        check_output(
            vec![3, 4, 5, 3, 5, 3, 4, 3, 4, 5, 3, 5, 3, 4, 5],
            vec![3, 4, 5],
            vec![0, 7, 12],
        );
    }

    #[test]
    fn triple_number() {
        check_output(vec![0, 0, 0, 1, 1, 1], vec![0, 0, 0], vec![0]);
    }

    #[test]
    fn repeated_sequences() {
        check_output(vec![0, 0, 0], vec![0, 0], vec![0, 1]);
    }

    // While it would be nice to account for this test case,
    // a more advanced algorithm would be needed to achieve this.
    // For more info, check out the Knuth-Morris-Pratt algorithm
    // #[test]
    // fn kmp_edge_case() {
    //     check_output(vec![0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1], vec![0, 2]);
    // }

    // Error Cases
    #[test]
    #[should_panic(expected = "Sequence and bytes parameters must be at least 2 bytes long")]
    fn single_byte_in_sequence() {
        check_output(vec![1, 2, 3], vec![0], vec![]);
    }

    #[test]
    #[should_panic(expected = "Sequence and bytes parameters must be at least 2 bytes long")]
    fn single_byte_in_bytes() {
        check_output(vec![0], vec![0, 0, 0], vec![0, 1, 2]);
    }
}

#[cfg(test)]
mod test_data {}

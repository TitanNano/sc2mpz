use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Context;
/**
 * Parses 4 bytes into a big endian unsigned integer.
 * Args:
 *      unparsed_bytes (bytes): 4 bytes representing the int.
 * Returns:
 *      Integer representation.
 */
pub fn parse_uint32(unparsed_bytes: &[u8; 4]) -> u32 {
    assert!(unparsed_bytes.len() == 4);

    u32::from_be_bytes(*unparsed_bytes)
}

/** Parses 2 bytes into a big endian unsigned integer.
 * Args:
 *      unparsed_bytes (bytes): 2 bytes representing the int.
 * Returns:
 *      Integer representation.
 */
pub fn parse_uint16(unparsed_bytes: &[u8; 2]) -> u16 {
    assert!(unparsed_bytes.len() == 2);

    let result = u16::from_be_bytes(*unparsed_bytes);

    assert!(result > 0);

    result
}

/** Parses 1 byte into a big endian signed integer.
 * Args:
 *      unparsed_bytes (bytes): 1 bytes representing the int.
 * Returns:
 *      Integer representation.
 */
pub fn parse_uint8(unparsed_bytes: &[u8; 1]) -> u8 {
    assert!(unparsed_bytes.len() == 1);

    unparsed_bytes[0]
}

/** Parses 4 bytes into a big endian signed integer.
 * Args:
 *      unparsed_bytes (bytes):  4 bytes representing the int.
 * Returns:
 *      Integer representation.
 */
pub fn parse_int32(unparsed_bytes: &[u8; 4]) -> i32 {
    i32::from_be_bytes(*unparsed_bytes)
}

/** Converts an int into its binary representation as a string of 0s and 1s.
 * Optionally takes an argument for the length to pad to.
 * Args:
 *      int_input (int): integer to convert.
 *      pad (int): Pad with 0s to this length.
 * Returns:
 *      String representation of the input integer in binary.
 */
pub fn int_to_bitstring(int: u32, pad: usize) -> String {
    let mut result = String::from("");
    let mut int_input = int;

    while int_input > 0 {
        let bit = int_input & 1;

        result = bit.to_string() + &result;
        int_input >>= 1;
    }

    while result.len() < pad {
        result = String::from("0") + &result;
    }

    result
}

pub fn parse_bitstring(bit_string: &str) -> u32 {
    let mut result: u32 = 0;

    for bit in bit_string.chars() {
        if result == 0 && bit == '0' {
            continue;
        }

        result = (result << 1)
            | u32::from_str_radix(&bit.to_string(), 2)
                .expect("bit_string can only contain 0 and 1");
    }

    result
}

/** Convenience function that opens a file and returns its binary contents.
 * Args:
 *      input_file (path): full path of a file to open.
 * Returns:
 *      Raw binary contents of the input file.
 */
pub fn open_file(input_file: &Path) -> Vec<u8> {
    let mut f = File::open(input_file)
        .with_context(|| {
            format!(
                "file path {} does not exists or can't be opened!",
                input_file.to_str().unwrap_or_default()
            )
        })
        .unwrap_or_else(|err| panic!("{:?}", err));

    let mut buffer = vec![];

    f.read_to_end(&mut buffer)
        .expect("failed to read file contents");

    buffer
}

/** Turns a sequence of bytes (must be a multiple of 4) into a list of integers.
 * Args:
 *      input_bytes (bytes): bytes to convert
 * Returns:
 *      List of converted signed integers.
 */
pub fn bytes_to_int32s(input_bytes: &[u8]) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    assert!(input_bytes.len() % 4 == 0); // ensure the byte array contains a full set of int32s

    for offset in (0..input_bytes.len()).step_by(4) {
        res.push(parse_int32(
            &input_bytes[offset..offset + 4]
                .try_into()
                .expect("there should be a slice of 4 bytes"),
        ));
    }

    res
}

/**
 * Variable length convenience function to convert some number of bytes to an int.
 * Args:
 *     input_bytes (bytes): Input bytes to convert.
 * Returns:
 *     Integer.
 */
pub fn bytes_to_uint(input_bytes: &[u8]) -> u32 {
    let num_bytes = input_bytes.len();

    match num_bytes {
        1 => parse_uint8(input_bytes.try_into().expect("should be 1 byte")) as u32,
        2 => parse_uint16(input_bytes.try_into().expect("should be 2 bytes")) as u32,
        4 => parse_uint32(input_bytes.try_into().expect("should be 4 bytes")),
        _ => panic!("unable to handle more than 4 bytes"),
    }
}

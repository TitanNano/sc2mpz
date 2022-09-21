use super::sc_util::{int_to_bitstring, parse_bitstring};
use serde::Serialize;

fn bit_as_bool(chars: &str, index: usize) -> bool {
    chars
        .chars()
        .nth(index)
        .expect("should have bit 0")
        .to_digit(2)
        .expect("digit")
        != 0
}

/// Stores the bit flags and implements str() and int().
#[derive(Debug, Serialize)]
pub struct BitFlags {
    powerable: bool,
    powered: bool,
    piped: bool,
    watered: bool,
    xval: bool,
    water: bool,
    rotate: bool,
    salt: bool,
}

/// Returns the integer corresponding to the flags.
impl BitFlags {
    fn to_u32(&self) -> u32 {
        parse_bitstring(&self.to_string())
    }

    /**
     * Converts this bitflags to a bytes.
     * Returns:
     *      A single, big endian byte representation of the bitflags.
     */
    fn to_byte(&self) {
        unimplemented!("to_byte is not implemented!");
    }
}

impl From<u8> for BitFlags {
    fn from(flags: u8) -> Self {
        let _flags_string = int_to_bitstring(flags as u32, 8);
        let _flags = _flags_string.as_str();

        let powerable = bit_as_bool(_flags, 0); // Is this a tile that needs power?
        let powered = bit_as_bool(_flags, 1); // Is this tile recieving power?
        let piped = bit_as_bool(_flags, 2); // Does this tile have pipes underneath it?
        let watered = bit_as_bool(_flags, 3); // Is this tile recieving water?
        let xval = bit_as_bool(_flags, 4); // Land value of this tile
        let water = bit_as_bool(_flags, 5); // Is this tile covered in water?
        let rotate = bit_as_bool(_flags, 6); // Should this tile be rotated?
        let salt = bit_as_bool(_flags, 7); // Is this tile salt water?

        Self {
            powerable,
            powered,
            piped,
            watered,
            xval,
            water,
            rotate,
            salt,
        }
    }
}

/// Returns a binary string representing the bitflags.
impl ToString for BitFlags {
    fn to_string(&self) -> String {
        let attrs = [
            self.powerable,
            self.powered,
            self.piped,
            self.watered,
            self.xval,
            self.water,
            self.rotate,
            self.salt,
        ];

        let bit_string: String = attrs.iter().map(|x| if *x { '1' } else { '0' }).collect();

        bit_string
    }
}

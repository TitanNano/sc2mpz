use super::sc_util;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[allow(dead_code)]
struct Sc2FileHeader {
    type_id: String,
    data_size: usize,
    file_type: String,
    city_name: Option<Vec<u8>>,
}

#[derive(Default)]
pub struct ChunkList {
    text: Vec<Vec<u8>>,
    cnam: Vec<u8>,
    altm: Vec<u8>,
    scen: Vec<u8>,
    pict: Vec<u8>,
    tile: Vec<u8>,

    misc: Vec<u8>,
    xter: Vec<u8>,
    xbld: Vec<u8>,
    xtrf: Vec<u8>,
    xplt: Vec<u8>,
    xval: Vec<u8>,
    xcrm: Vec<u8>,
    xzon: Vec<u8>,
    xund: Vec<u8>,
    xtxt: Vec<u8>,
    xbit: Vec<u8>,
    xplc: Vec<u8>,
    xfir: Vec<u8>,
    xpop: Vec<u8>,
    xrog: Vec<u8>,
    xlab: Vec<u8>,
    xmic: Vec<u8>,
    xthg: Vec<u8>,
    xgrp: Vec<u8>,

    unknown: HashMap<String, Vec<u8>>,
}

impl ChunkList {
    fn set(&mut self, id: &str, value: &[u8]) {
        match id {
            "TEXT" => self.text = vec![value.to_vec()],
            "CNAM" => self.cnam = value.into(),
            "ALTM" => self.altm = value.into(),
            "PICT" => self.pict = value.into(),
            "TILE" => self.tile = value.into(),

            "MISC" => self.misc = value.into(),
            "XTER" => self.xter = value.into(),
            "XBLD" => self.xbld = value.into(),
            "XTRF" => self.xtrf = value.into(),
            "XPLT" => self.xplt = value.into(),
            "XVAL" => self.xval = value.into(),
            "XCRM" => self.xcrm = value.into(),
            "XZON" => self.xzon = value.into(),
            "XUND" => self.xund = value.into(),
            "XTXT" => self.xtxt = value.into(),
            "XBIT" => self.xbit = value.into(),
            "XPLC" => self.xplc = value.into(),
            "XFIR" => self.xfir = value.into(),
            "XPOP" => self.xpop = value.into(),
            "XROG" => self.xrog = value.into(),
            "XLAB" => self.xlab = value.into(),
            "XMIC" => self.xmic = value.into(),
            "XTHG" => self.xthg = value.into(),
            "XGRP" => self.xgrp = value.into(),

            _ => {
                self.unknown.insert(id.into(), value.to_vec());
            }
        };
    }

    fn iter_compressed(&self) -> CompressedIterator {
        CompressedIterator {
            list: self,
            cursor: 0,
        }
    }

    pub fn text(&self) -> &[Vec<u8>] {
        &self.text
    }

    pub fn cnam(&self) -> &[u8] {
        &self.cnam
    }

    pub fn altm(&self) -> &[u8] {
        &self.altm
    }

    pub fn scen(&self) -> &[u8] {
        &self.scen
    }

    pub fn pict(&self) -> &[u8] {
        &self.pict
    }

    #[allow(dead_code)]
    pub fn tile(&self) -> &[u8] {
        &self.tile
    }

    pub fn misc(&self) -> &[u8] {
        &self.misc
    }

    pub fn xter(&self) -> &[u8] {
        &self.xter
    }

    pub fn xbld(&self) -> &[u8] {
        &self.xbld
    }

    pub fn xtrf(&self) -> &[u8] {
        &self.xtrf
    }

    pub fn xplt(&self) -> &[u8] {
        &self.xplt
    }

    pub fn xval(&self) -> &[u8] {
        &self.xval
    }

    pub fn xcrm(&self) -> &[u8] {
        &self.xcrm
    }

    pub fn xzon(&self) -> &[u8] {
        &self.xzon
    }

    pub fn xund(&self) -> &[u8] {
        &self.xund
    }

    pub fn xtxt(&self) -> &[u8] {
        &self.xtxt
    }

    pub fn xbit(&self) -> &[u8] {
        &self.xbit
    }

    pub fn xplc(&self) -> &[u8] {
        &self.xplc
    }

    pub fn xfir(&self) -> &[u8] {
        &self.xfir
    }

    pub fn xpop(&self) -> &[u8] {
        &self.xpop
    }

    pub fn xrog(&self) -> &[u8] {
        &self.xrog
    }

    pub fn xlab(&self) -> &[u8] {
        &self.xlab
    }

    pub fn xmic(&self) -> &[u8] {
        &self.xmic
    }

    pub fn xthg(&self) -> &[u8] {
        &self.xthg
    }

    pub fn xgrp(&self) -> &[u8] {
        &self.xgrp
    }
}

struct CompressedIterator<'a> {
    list: &'a ChunkList,
    cursor: usize,
}

impl<'a> Iterator for CompressedIterator<'a> {
    type Item = (String, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        let offset = 19;

        let next = match self.cursor {
            0 => Some((String::from("MISC"), &self.list.misc)),
            1 => Some((String::from("XTER"), &self.list.xter)),
            2 => Some((String::from("XBLD"), &self.list.xbld)),
            3 => Some((String::from("XTRF"), &self.list.xtrf)),
            4 => Some((String::from("XPLT"), &self.list.xplt)),
            5 => Some((String::from("XVAL"), &self.list.xval)),
            6 => Some((String::from("XCRM"), &self.list.xcrm)),
            7 => Some((String::from("XZON"), &self.list.xzon)),
            8 => Some((String::from("XUND"), &self.list.xund)),
            9 => Some((String::from("XTXT"), &self.list.xtxt)),
            10 => Some((String::from("XBIT"), &self.list.xbit)),
            11 => Some((String::from("XPLC"), &self.list.xplc)),
            12 => Some((String::from("XFIR"), &self.list.xfir)),
            13 => Some((String::from("XPOP"), &self.list.xpop)),
            14 => Some((String::from("XROG"), &self.list.xrog)),
            15 => Some((String::from("XLAB"), &self.list.xlab)),
            16 => Some((String::from("XMIC"), &self.list.xmic)),
            17 => Some((String::from("XTHG"), &self.list.xthg)),
            18 => Some((String::from("XGRP"), &self.list.xgrp)),

            i if (i - offset) < self.list.unknown.len() => self
                .list
                .unknown
                .iter()
                .nth(i - offset)
                .map(|(key, value)| (key.to_owned(), value)),
            _ => None,
        };

        self.cursor += 1;

        next.map(|(key, value)| (key, value.to_owned()))
    }
}

/**
 * Get's the city's name, if it exists. Sometimes CNAM contains garbage, so this also cleans that up.
 * Args:
 *      dirty_name (bytes): City's name, possible with garbage in it.
 * Returns:
 *      A string of the name, with garbage removed.
 */
pub fn clean_city_name(dirty_name: &[u8]) -> String {
    dirty_name[1..32]
        .iter()
        .take_while(|x| **x != 0x00)
        .map(|x| char::from(x.to_owned()))
        .collect()
}

// Functions to handle decompression and compression of the actual city information.

/**
 * Takes already uncompressed city data and converts it into chunks.
 * Args:
 *      input_file (bytes): raw uncompressed city data.
 *      input_type (str): type of the input file we're opening.
 * Returns:
 *      A dictionary of {chunk id: chunk data} form, one entry per chunk.
 * Raises:
 *      SC2Parse: re-raised errors from check_file()
 */
pub fn chunk_input_serial(input_file: &[u8], input_type: &str) -> Result<ChunkList> {
    let mut output_dict = ChunkList::default();
    let (header, input_file) = check_file(input_file, input_type)?;

    let file_length = header.data_size;

    // -12B for the header
    let mut remaining_length = file_length - 12;

    if output_dict.cnam.is_empty() {
        if let Some(city_name) = header.city_name {
            output_dict.cnam = city_name;
        }
    }

    while remaining_length > 0 {
        let offset = file_length - remaining_length;
        let (chunk_id, chunk_size, chunk_data) = get_chunk_from_offset(input_file, offset);

        if chunk_id == "TEXT" {
            output_dict.text.push(chunk_data.to_vec());
        } else {
            output_dict.set(chunk_id.as_str(), chunk_data);
        }

        // How much of the file still needs to be scanned? Subtract the size of the chunk's data and header from it.
        remaining_length -= chunk_size + 8;
    }

    Ok(output_dict)
}

/**
 * Does some basic checks of the file to make sure it's valid and includes special handling for the Mac version of the game. The IFF standard is from 1985, so it's not super robust...
 * Untested with some of the weirder versions of SC2k, such as Amiga, PocketPC/Windows Mobile, etc.
 * Currently only supports parsing for FORM and MIFF files.
 * Args:
 *      input_data (bytes): bytes containing the entirety of the city.
 *      input_type (str): type of input file, supported are 'mif' for .mif tileset/MIFF file and 'sc2' for .sc2 city file.
 * Returns:
 *      A tuple containing a dictionary and the input.
 *      The dictionary looks like {'type_id': header, 'data_size': reported_size, 'file_type': file_type} where the header is the opening 4 bytes of input as a bytestring, reported_size is an int of the size the file claims to be and file_type is one of b"SC2K" (tileset) of b"SCDH" (city).
 * Raises:
 *      SC2Parse: an error relating to parsing .sc2 files. Could be caused by the file being a SimCity classic city (currently an unsupported format), not a city file at all, or being corrupted.
 *      MIFFParse: an error relating to parsing .mif files. Could be caused by file corruption of not actually being a tileset file.
 */
fn check_file<'a>(input_data: &'a [u8], input_type: &str) -> Result<(Sc2FileHeader, &'a [u8])> {
    let mut city_name: Option<Vec<u8>> = None;
    let mut input_data = input_data;

    // Check and convert if this is a Mac city file.
    if mac_check(input_data) {
        let (fixed_input_data, fixed_city_name) = mac_fix(input_data);

        input_data = fixed_input_data;
        city_name = Some(fixed_city_name.to_vec());
    }

    // This should be "FORM" for .sc2
    let header: &[u8; 4] = &input_data[0..4].try_into().expect("should be 4 bytes");

    // The reported size saved in the .sc2, we don't count the first 8 bytes though, so we need to add them back.
    let reported_size =
        sc_util::parse_int32(input_data[4..8].try_into().expect("should be 4 bytes")) + 8;

    // This should be "SCDH"
    let file_type = input_data[8..12].try_into().expect("should be 4 bytes");

    // Actual size of our input file
    let actual_size: i32 = input_data
        .len()
        .try_into()
        .expect("actual_size should fit in i32");

    if reported_size != actual_size {
        return Err(anyhow!(
            "File reports being: {}B, but is actually {}B long.",
            reported_size,
            actual_size
        ));
    }

    let header_string = String::from_utf8_lossy(header);
    let file_type_string = String::from_utf8_lossy(file_type);

    // Check and see if this is a Simcity Classic city.
    match input_type {
        "sc2" => {
            if header_string != "FORM" {
                let data_match =
                    input_data[0x41..0x49] == [0x43, 0x49, 0x54, 0x59, 0x4D, 0x43, 0x52, 0x50];
                let header_match = header[0..2] == [0x00, 0x0d];

                // Check and see if this is a Simcity Classic city.
                let error = if data_match && header_match {
                    String::from("Simcity Classic city files are not supported.")
                } else {
                    format!("Not a FORM type IFF file, claiming: {}", header_string)
                };

                return Err(anyhow!(error));
            }

            if file_type_string != "SCDH" {
                return Err(anyhow!(
                    "File type is not SCDH, claiming: {}",
                    file_type_string
                ));
            }
        }

        "mif" => {
            if header_string != "MIFF" {
                return Err(anyhow!(
                    "Not a MIFF type IFF file, claiming: {}",
                    header_string
                ));
            }

            if file_type_string != "SC2K" {
                return Err(anyhow!(
                    "File type is not SC2K, claiming: {}",
                    file_type_string
                ));
            }
        }

        _ => panic!("unknown input type: {}", input_type),
    }

    Ok((
        Sc2FileHeader {
            type_id: header_string.into_owned(),
            data_size: reported_size as usize,
            file_type: file_type_string.into_owned(),
            city_name,
        },
        input_data,
    ))
}

/**
 * Parses an IFF chunk by reading the header and using the size to determine which bytes belong to it.
 * An IFF chunk has an 8 byte header, of which the first 4 bytes is the type and the second 4 bytes is the size (exclusive of the header).
 * Args:
 *      input_data (bytes): raw city information.
 *      offset (int): starting offset in input to start parsing at.
 * Returns:
 *      A list containing the id of the chunk (a 4 byte ascii value), an int length of the chunk of finally bytes of the chunk data.
 */
fn get_chunk_from_offset(input_data: &[u8], offset: usize) -> (String, usize, &[u8]) {
    let location_index = offset;
    let chunk_id =
        String::from_utf8_lossy(&input_data[location_index..location_index + 4]).into_owned();

    // Maximum 32b/4B, so 2^32 in length.
    let chunk_size = sc_util::parse_uint32(
        input_data[(location_index + 4)..(location_index + 8)]
            .try_into()
            .expect("should be 4 bytes"),
    ) as usize;

    let chunk_data = &input_data[(location_index + 8)..(location_index + 8 + chunk_size)];

    (chunk_id, chunk_size, chunk_data)
}

/**
 * Checks if this is a Mac .sc2 file.
 * Args:
 *      input_data (bytes): raw city information.
 * Returns:
 *      True if this is a Mac formatted file, False if it isn't.
 */
fn mac_check(input_data: &[u8]) -> bool {
    let header = &input_data[0..4];
    let mac_form = &input_data[0x80..0x84];

    String::from_utf8_lossy(header) != "FORM" && String::from_utf8_lossy(mac_form) == "FORM"
}

/**
 * Makes a Mac city file compatible with the Win95 version of the game.
 * Basically, we don't need the first 0x80 bytes from the Mac file, something about a resource fork. Also, some of the files have garbage data at the end, which is also trimmed.
 * Args:
 *      input_data (bytes): raw city information.
 * Returns:
 *      Bytes comprising a compatible SC2k Win95 city file from the Mac file, and the name of the city from the start of the file.
 */
fn mac_fix(input_data: &[u8]) -> (&[u8], &[u8]) {
    let reported_size = (sc_util::parse_int32(
        input_data[0x84..0x88]
            .try_into()
            .expect("should be 4 bytes"),
    ) + 8) as usize;

    let name_len = input_data[1] as usize;
    let city_name = &input_data[1..2 + name_len];

    (&input_data[0x80..(0x80 + reported_size)], city_name)
}

/**
 * Uncompresses a compressed .mif or .sc2 file.
 * For a .sc2 file, doesn't uncompress chunks with id of CNAM or ALTM and for .mif, soesn't uncompress TILE chunks.
 * Args:
 *      input_file (bytes): compressed city data.
 *      input_type (str): type of the input file we're opening.
 * Returns:
 *      A dictionary of uncompressed {chunk id: chunk data} form, one entry per chunk.
 */
pub fn sc2_uncompress_input(input_file: ChunkList, input_type: &str) -> ChunkList {
    let mut uncompressed_chunk_list = ChunkList::default();

    log::debug!("cnam: {}", input_file.cnam.len());
    log::debug!("misc: {}", input_file.misc.len());
    log::debug!("altm: {}", input_file.altm.len());
    log::debug!("xter: {}", input_file.xter.len());
    log::debug!("xbld: {}", input_file.xbld.len());

    log::debug!("uncompressing file data...");

    match input_type {
        "sc2" => {
            input_file
                .iter_compressed()
                .map(|(key, slice)| {
                    log::debug!("decompressing {}...", key);

                    (key, uncompress_rle(&slice))
                })
                .for_each(|(key, value)| uncompressed_chunk_list.set(&key, &value));

            uncompressed_chunk_list.text = input_file.text;
            uncompressed_chunk_list.cnam = input_file.cnam;
            uncompressed_chunk_list.altm = input_file.altm;
            uncompressed_chunk_list.scen = input_file.scen;
            uncompressed_chunk_list.pict = input_file.pict;
        }

        "mif" => {
            input_file
                .iter_compressed()
                .map(|(key, slice)| (key, uncompress_rle(&slice)))
                .for_each(|(key, value)| uncompressed_chunk_list.set(&key, &value));

            uncompressed_chunk_list.tile = input_file.tile;
        }

        _ => panic!("unknown input type: {}", input_type),
    }

    log::debug!("cnam: {}", uncompressed_chunk_list.cnam.len(),);
    log::debug!("misc: {}", uncompressed_chunk_list.misc.len(),);
    log::debug!("altm: {}", uncompressed_chunk_list.altm.len(),);
    log::debug!("xter: {}", uncompressed_chunk_list.xter.len(),);
    log::debug!("xbld: {}", uncompressed_chunk_list.xbld.len(),);

    assert!(uncompressed_chunk_list.cnam.len() == 32);
    assert!(uncompressed_chunk_list.misc.len() == 4800);
    assert!(uncompressed_chunk_list.altm.len() == 32768);
    assert!(uncompressed_chunk_list.xter.len() == 16384);
    assert!(uncompressed_chunk_list.xbld.len() == 16384);

    uncompressed_chunk_list
}

/**
 * Uncompresses the RLE compressed city data. For more information, consult the .sc2 file format specification documents at https://github.com/dfloer/SC2k-docs
 * Args:
 *      encoded_data (bytes): raw city information.
 * Returns:
 *      Uncompressed bytes.
 */
fn uncompress_rle(encoded_data: &[u8]) -> Vec<u8> {
    let mut decoded_data = vec![];
    let mut next_byte_repeat = false;
    let mut byte_count = 0u8;

    // Data is stored in two forms: 0x01..0x7F and 0x81..0xFF
    for byte in encoded_data {
        match (byte, byte_count, next_byte_repeat) {
            (0..=0x7F, 0, _) => {
                // In this case, byte is a count of the number of data bytes that follow.
                byte.clone_into(&mut byte_count);
                next_byte_repeat = false;
                log::debug!("new byte count: {}", byte_count);
            }
            (0x80.., 0, _) => {
                // In this case, byte-127=count of how many times the very next byte repeats.
                byte_count = byte - 0x7f;
                next_byte_repeat = true;
                log::debug!("new byte repeat: {}", byte_count);
            }
            (0.., 1.., true) => {
                let size = byte_count as usize;
                let mut repeated = Vec::with_capacity(size);

                repeated.resize(size, *byte);
                decoded_data.append(&mut repeated);
                log::debug!(
                    "repeated byte {:#04x} x {}. New Length {}",
                    byte,
                    byte_count,
                    decoded_data.len()
                );
                byte_count = 0;
            }
            (0.., 1.., false) => {
                decoded_data.push(*byte);
                byte_count -= 1;
                log::debug!(
                    "appending byte {:#04x}, remaining bytes {}, New Length {}",
                    byte,
                    byte_count,
                    decoded_data.len()
                );
            }
        }
    }

    decoded_data
}

use serde::Serialize;
use std::collections::HashMap;

// Couldn't think of a better name, but this stores minimap info/simulation variables stores in:
// XTRF, XPLT, XVAL, XCRM, XPLC, XFIR, XPOP, XROG.

const X64: [&str; 4] = ["XTRF", "XPLT", "XVAL", "XCRM"];
const X32: [&str; 4] = ["XPLC", "XFIR", "XPOP", "XROG"];

#[derive(Clone, Debug, Serialize)]
pub struct Minimap {
    name: String,
    // #[serde(serialize_with = "serialize_cord_hash_map")]
    data: HashMap<(usize, usize), u8>,
    size: usize,
}

impl Minimap {
    pub fn new(name: String, size: usize) -> Self {
        let data = Default::default();

        Self { name, size, data }
    }

    fn convert_xy(&self, key: (usize, usize)) -> (usize, usize) {
        let (x, y) = key;
        let d = if self.size == 64 { 2 } else { 4 };

        (x / d, y / d)
    }

    pub fn get_scaled(&self, key: (usize, usize)) -> &u8 {
        let new_key = self.convert_xy(key);

        &self.data[&new_key]
    }

    pub fn set_scaled(&mut self, key: (usize, usize), item: u8) {
        let new_key = self.convert_xy(key);

        self.data.insert(new_key, item);
    }

    pub fn set_item(&mut self, key: (usize, usize), value: u8) {
        self.data.insert(key, value);
    }

    fn get_item(&self, key: (usize, usize)) -> &u8 {
        &self.data[&key]
    }
}

impl ToString for Minimap {
    fn to_string(&self) -> String {
        [self.name.clone()]
            .into_iter()
            .chain((0..self.size).map(|_| {
                let items: Vec<_> = (0..self.size).into_iter().map(|y| y.to_string()).collect();

                items.join("")
            }))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

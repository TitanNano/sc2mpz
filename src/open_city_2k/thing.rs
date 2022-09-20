use serde::Serialize;

/// Class to represent a thing stored in the XTHG segment.
#[derive(Debug, Serialize)]
pub struct Thing {
    thing_id: usize,
    rotation_1: usize,
    rotation_2: usize,
    x: usize,
    y: usize,
    data: [u8; 7],
}

impl Thing {
    /**
     * Parses raw bytes into a thing.
     * Args:
     *      raw_thing (bytes):  12 bytes representing the thing.
     */
    pub fn parse_thing(raw_thing: &[u8; 12]) -> Self {
        let thing_id = raw_thing[0] as usize;
        let rotation_1 = raw_thing[1] as usize;
        let rotation_2 = raw_thing[2] as usize;
        let x = raw_thing[3] as usize;
        let y = raw_thing[4] as usize;
        let data = raw_thing[5..12]
            .try_into()
            .expect("slice should be of length 7");

        Self {
            thing_id,
            rotation_1,
            rotation_2,
            x,
            y,
            data,
        }
    }
}

impl ToString for Thing {
    fn to_string(&self) -> String {
        format!(
            "Thing with ID: {} at ({}, {}), rotations: {}, {}, data: {:?}",
            self.thing_id, self.x, self.y, self.rotation_1, self.rotation_2, self.data
        )
    }
}

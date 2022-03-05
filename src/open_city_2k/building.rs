use super::buildings;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Building {
    pub building_id: u8,
    tile_coords: (usize, usize),
    name: &'static str,
    size: usize,
}

impl Building {
    pub fn new(building_id: u8, coords: (usize, usize)) -> Self {
        let name = buildings::get_name(&building_id).expect(&format!(
            "trying to create bulding with invalid id {:#04x}",
            building_id
        ));

        let size = buildings::get_size(&building_id).expect(&format!(
            "trying to create bulding with invalid id {:#04x}",
            building_id
        ));

        let tile_coords = coords;

        Self {
            building_id,
            tile_coords,
            name,
            size,
        }
    }
}

impl ToString for Building {
    fn to_string(&self) -> String {
        format!(
            "Building: {} {:#04x} at {}, {}",
            self.name, self.building_id, self.tile_coords.0, self.tile_coords.1
        )
    }
}

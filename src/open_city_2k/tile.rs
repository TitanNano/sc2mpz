// Stores all the information related to a tile.

use super::bit_flags::BitFlags;
use super::building::Building;
use super::minimap::Minimap;
use super::sc_util::int_to_bitstring;
use serde::Serialize;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct Tile {
    coordinates: (usize, usize),
    altitude_tunnel: u32,
    is_water: bool,
    altitude_unknown: u32,
    altitude: u32,
    terrain: u8,
    building: Option<Arc<Building>>,
    zone_corners: String,
    zone: u32,
    underground: u8,
    _label: Vec<String>,
    text_pointer: i32,
    bit_flags: Option<BitFlags>,
    #[serde(skip_serializing)]
    _traffic_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _pollution_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _value_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _crime_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _police_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _fire_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _density_minimap: Arc<Minimap>,
    #[serde(skip_serializing)]
    _growth_minimap: Arc<Minimap>,
}

impl Tile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        traffic: Arc<Minimap>,
        pollution: Arc<Minimap>,
        value: Arc<Minimap>,
        crime: Arc<Minimap>,
        police: Arc<Minimap>,
        fire: Arc<Minimap>,
        density: Arc<Minimap>,
        growth: Arc<Minimap>,
        label: Vec<String>,
    ) -> Self {
        let coordinates = (0, 0);
        // Altitude map related values.
        let altitude_tunnel = 0;
        let is_water = false;
        let altitude_unknown = 0;
        let altitude = 0;
        // Terrain
        let terrain = 0;
        // City stuff
        let building = None;
        let zone_corners = String::from("");
        let zone = 0;
        let underground = 0;
        let _label = label;
        // text/signs
        let text_pointer = -1;
        // bit flags
        let bit_flags = None;

        // minimaps/simulation stuff
        Self {
            coordinates,
            altitude_tunnel,
            is_water,
            altitude_unknown,
            altitude,
            terrain,
            building,
            zone_corners,
            zone,
            underground,
            _label,
            text_pointer,
            bit_flags,
            _traffic_minimap: traffic,
            _pollution_minimap: pollution,
            _value_minimap: value,
            _crime_minimap: crime,
            _police_minimap: police,
            _fire_minimap: fire,
            _density_minimap: density,
            _growth_minimap: growth,
        }
    }

    pub fn set_coordinates(&mut self, value: (usize, usize)) {
        self.coordinates = value;
    }

    pub fn set_altitude(&mut self, value: u32) {
        self.altitude = value;
    }

    pub fn set_altitude_unknown(&mut self, value: u32) {
        self.altitude_unknown = value;
    }

    pub fn set_altitude_tunnel(&mut self, value: u32) {
        self.altitude_tunnel = value;
    }

    pub fn set_is_water(&mut self, value: bool) {
        self.is_water = value;
    }

    pub fn terrain(&self) -> &u8 {
        &self.terrain
    }

    pub fn set_terrain(&mut self, value: u8) {
        self.terrain = value;
    }

    pub fn zone_corners(&self) -> &str {
        &self.zone_corners
    }

    pub fn set_zone_corners(&mut self, value: String) {
        self.zone_corners = value;
    }

    pub fn zone(&self) -> &u32 {
        &self.zone
    }

    pub fn set_zone(&mut self, value: u32) {
        self.zone = value;
    }

    pub fn underground(&self) -> &u8 {
        &self.underground
    }

    pub fn set_underground(&mut self, value: u8) {
        self.underground = value;
    }

    pub fn text_pointer(&self) -> &i32 {
        &self.text_pointer
    }

    pub fn set_text_pointer(&mut self, value: u8) {
        self.text_pointer = value as i32;
    }

    pub fn set_bit_flags(&mut self, value: BitFlags) {
        self.bit_flags = Some(value);
    }

    pub fn bit_flags(&self) -> &Option<BitFlags> {
        &self.bit_flags
    }

    fn get_traffic(&self) -> &u8 {
        self._traffic_minimap.get_scaled(self.coordinates)
    }

    fn get_pollution(&self) -> &u8 {
        self._pollution_minimap.get_scaled(self.coordinates)
    }

    fn get_value(&self) -> &u8 {
        self._value_minimap.get_scaled(self.coordinates)
    }

    fn get_crime(&self) -> &u8 {
        self._crime_minimap.get_scaled(self.coordinates)
    }

    fn get_police(&self) -> &u8 {
        self._police_minimap.get_scaled(self.coordinates)
    }

    fn get_fire(&self) -> &u8 {
        self._fire_minimap.get_scaled(self.coordinates)
    }

    fn get_density(&self) -> &u8 {
        self._density_minimap.get_scaled(self.coordinates)
    }

    fn get_growth(&self) -> &u8 {
        self._growth_minimap.get_scaled(self.coordinates)
    }

    fn get_text(&self) -> &str {
        &self._label[self.text_pointer as usize]
    }

    #[allow(dead_code)]
    pub fn building(&self) -> &Option<Arc<Building>> {
        &self.building
    }

    pub fn set_building(&mut self, value: Arc<Building>) {
        self.building = Some(value);
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terr = int_to_bitstring(self.terrain as u32, 0);
        let b_id = if let Some(building) = &self.building {
            format!("{:#04x}", building.building_id)
        } else {
            String::from("null")
        };

        let sign_text = if self.text_pointer > -1 {
            format!(", Sign: {:?}", self.get_text())
        } else {
            String::from("")
        };

        write!(
            f,
            r#"Tile at {:?}
Altitude:
    tunnel: {}, water: {}, unknown: {}, altitude: {}
Terrain: {}
Buildings:
    id: {}, corners {}, zone: {}, underground: {}
Text pointer: {}{}
Flags: {:?}
Minimap:
    Traffic: {:?}, pollution: {:?}, value: {:?}, crime: {:?}, police: {:?}, fire: {:?}, density: {:?}, growth: {:?}
"#,
            self.coordinates,
            self.altitude_tunnel,
            self.is_water,
            self.altitude_unknown,
            self.altitude,
            terr,
            b_id,
            self.zone_corners,
            self.zone,
            self.underground,
            self.text_pointer,
            sign_text,
            self.bit_flags,
            self.get_traffic(),
            self.get_pollution(),
            self.get_value(),
            self.get_crime(),
            self.get_police(),
            self.get_fire(),
            self.get_density(),
            self.get_growth()
        )
    }
}

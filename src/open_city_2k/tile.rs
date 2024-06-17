// Stores all the information related to a tile.

use super::bit_flags::BitFlags;
use super::building::Building;
use super::City;
use serde::Serialize;
use std::ops::Deref;
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
    label: Vec<String>,
    text_pointer: i32,
    bit_flags: Option<BitFlags>,
}

impl Tile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(label: Vec<String>) -> Self {
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
            label,
            text_pointer,
            bit_flags,
        }
    }

    pub fn coordinates(&self) -> (usize, usize) {
        self.coordinates
    }

    pub fn set_coordinates(&mut self, value: (usize, usize)) {
        self.coordinates = value;
    }

    pub fn altitude(&self) -> u32 {
        self.altitude
    }

    pub fn set_altitude(&mut self, value: u32) {
        self.altitude = value;
    }

    pub fn altitude_unknown(&self) -> u32 {
        self.altitude_unknown
    }

    pub fn set_altitude_unknown(&mut self, value: u32) {
        self.altitude_unknown = value;
    }

    pub fn altitude_tunnel(&self) -> u32 {
        self.altitude_tunnel
    }

    pub fn set_altitude_tunnel(&mut self, value: u32) {
        self.altitude_tunnel = value;
    }

    pub fn is_water(&self) -> bool {
        self.is_water
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

    pub fn text_pointer(&self) -> i32 {
        self.text_pointer
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

    pub fn traffic(&self, city: &City) -> u8 {
        *city.traffic.get_scaled(self.coordinates)
    }

    pub fn pollution(&self, city: &City) -> u8 {
        *city.pollution.get_scaled(self.coordinates)
    }

    pub fn value(&self, city: &City) -> u8 {
        *city.value.get_scaled(self.coordinates)
    }

    pub fn crime(&self, city: &City) -> u8 {
        *city.crime.get_scaled(self.coordinates)
    }

    pub fn police(&self, city: &City) -> u8 {
        *city.police.get_scaled(self.coordinates)
    }

    pub fn fire(&self, city: &City) -> u8 {
        *city.fire.get_scaled(self.coordinates)
    }

    pub fn density(&self, city: &City) -> u8 {
        *city.density.get_scaled(self.coordinates)
    }

    pub fn growth(&self, city: &City) -> u8 {
        *city.growth.get_scaled(self.coordinates)
    }

    pub fn text(&self) -> Option<&str> {
        self.label.get(self.text_pointer as usize).map(Deref::deref)
    }

    #[allow(dead_code)]
    pub fn building(&self) -> &Option<Arc<Building>> {
        &self.building
    }

    pub fn set_building(&mut self, value: Arc<Building>) {
        self.building = Some(value);
    }
}

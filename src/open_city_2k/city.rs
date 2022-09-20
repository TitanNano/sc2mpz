use std::cmp::min;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use log::{debug, error, info, warn};
use phf::phf_map;
use serde::Serialize;

use super::bit_flags::BitFlags;
use super::budget::Budget;
use super::building::Building;
use super::buildings;
use super::buildings::GROUNDCOVER_IDS;
use super::buildings::HIGHWAY_2X2_IDS;
use super::buildings::NETWORK_IDS;
use super::graph::Graph;
use super::indexed_write::IndexedWrite;
use super::minimap::Minimap;
use super::sc2_iff_parse as sc2p;
use super::sc2_iff_parse::ChunkList;
use super::sc_util;
use super::thing::Thing;
use super::tile::Tile;

// constants
const GRAPH_WINDOW_GRAPHS: [&str; 16] = [
    "City Size",
    "Residents",
    "Commerce",
    "Industry",
    "Traffic",
    "Pollution",
    "Value",
    "Crime",
    "Power %",
    "Water %",
    "Health",
    "Education",
    "Unemployment",
    "GNP",
    "Nat'n Pop.",
    "Fed Rate",
];
const POPULATION_GRAPH_NAMES: [&str; 3] = ["population_percent", "health_le", "education_eq"];
const INDUSTRY_GRAPH_NAMES: [&str; 3] = [
    "industrial_ratios",
    "industrial_tax_rate",
    "industrial_demand",
];
const SIMULATOR_SETTINGS_NAMES: [&str; 9] = [
    "YearEnd",
    "GlobalSeaLevel",
    "terCoast",
    "terRiver",
    "Military",
    "Zoom",
    "Compass",
    "CityCentX",
    "CityCentY",
];
const GAME_SETTING_NAME: [&str; 6] = [
    "GameSpeed",
    "AutoBudget",
    "AutoGo",
    "UserSoundOn",
    "UserMusicOn",
    "NoDisasters",
];
const INVENTION_NAMES: [&str; 17] = [
    "gas_power",
    "nuclear_power",
    "solar_power",
    "wind_power",
    "microwave_power",
    "fusion_power",
    "airport",
    "highways",
    "buses",
    "subways",
    "water_treatment",
    "desalinisation",
    "plymouth",
    "forest",
    "darco",
    "launch",
    "highway_2",
];

const MISC_PARSE_ORDER: phf::Map<u16, &'static str> = phf_map! {
    0x0000u16 => "FirstEntry",  // nominally the same in every city.
    0x0004u16 => "GameMode",
    0x0008u16 => "Compass",  // rotation
    0x000cu16 => "baseYear",
    0x0010u16 => "simCycle",
    0x0014u16 => "TotalFunds",
    0x0018u16 => "TotalBonds",
    0x001cu16 => "GameLevel",
    0x0020u16 => "CityStatus",
    0x0024u16 => "CityValue",
    0x0028u16 => "LandValue",
    0x002cu16 => "CrimeCount",
    0x0030u16 => "TrafficCount",
    0x0034u16 => "Pollution",
    0x0038u16 => "CityFame",
    0x003cu16 => "Advertising",
    0x0040u16 => "Garbage",
    0x0044u16 => "WorkerPercent",
    0x0048u16 => "WorkerHealth",
    0x004cu16 => "WorkerEducate",
    0x0050u16 => "NationalPop",
    0x0054u16 => "NationalValue",
    0x0058u16 => "NationalTax",
    0x005cu16 => "NationalTrend",
    0x0060u16 => "heat",
    0x0064u16 => "wind",
    0x0068u16 => "humid",
    0x006cu16 => "weatherTrend",
    0x0070u16 => "NewDisaster",
    0x0074u16 => "oldResPop",
    0x0078u16 => "Rewards",
    0x007cu16 => "Population Graphs",
    0x016cu16 => "Industry Graphs",
    0x01f0u16 => "Tile Counts",
    0x05f0u16 => "ZonePop|0",
    0x05f4u16 => "ZonePop|1",
    0x05f8u16 => "ZonePop|2",
    0x05fcu16 => "ZonePop|3",
    0x0600u16 => "ZonePop|4",
    0x0604u16 => "ZonePop|5",
    0x0608u16 => "ZonePop|6",
    0x060cu16 => "ZonePop|7",
    0x0610u16 => "Bonds",
    0x06d8u16 => "Neighbours",
    0x0718u16 => "Valve?|0",  // reverse engineered from the game, may be a typo in original.
    0x071cu16 => "Valve?|1",
    0x0720u16 => "Valve?|2",
    0x0724u16 => "Valve?|3",
    0x0728u16 => "Valve?|4",
    0x072cu16 => "Valve?|5",
    0x0730u16 => "Valve?|6",
    0x0734u16 => "Valve?|7",
    0x0738u16 => "gas_power",
    0x073cu16 => "nuclear_power",
    0x0740u16 => "solar_power",
    0x0744u16 => "wind_power",
    0x0748u16 => "microwave_power",
    0x074cu16 => "fusion_power",
    0x0750u16 => "airport",
    0x0754u16 => "highways",
    0x0758u16 => "buses",
    0x075cu16 => "subways",
    0x0760u16 => "water_treatment",
    0x0764u16 => "desalinisation",
    0x0768u16 => "plymouth",
    0x076cu16 => "forest",
    0x0770u16 => "darco",
    0x0774u16 => "launch",
    0x0778u16 => "highway_2",
    0x077cu16 => "Budget",
    0x0e3cu16 => "YearEnd",
    0x0e40u16 => "GlobalSeaLevel",
    0x0e44u16 => "terCoast",
    0x0e48u16 => "terRiver",
    0x0e4cu16 => "Military",
    0x0e50u16 => "Paper List",
    0x0ec8u16 => "News List",
    0x0fa0u16 => "Ordinances",
    0x0fa4u16 => "unemployed",
    0x0fa8u16 => "Military Count",
    0x0fe8u16 => "SubwayCnt",
    0x0fecu16 => "GameSpeed",
    0x0ff0u16 => "AutoBudget",
    0x0ff4u16 => "AutoGo",
    0x0ff8u16 => "UserSoundOn",
    0x0ffcu16 => "UserMusicOn",
    0x1000u16 => "NoDisasters",
    0x1004u16 => "PaperDeliver",
    0x1008u16 => "PaperExtra",
    0x100cu16 => "PaperChoice",
    0x1010u16 => "unknown128",
    0x1014u16 => "Zoom",
    0x1018u16 => "CityCentX",
    0x101cu16 => "CityCentY",
    0x1020u16 => "GlobalArcoPop",
    0x1024u16 => "ConnectTiles",
    0x1028u16 => "TeamsActive",
    0x102cu16 => "TotalPop",
    0x1030u16 => "IndustryBonus",
    0x1034u16 => "PolluteBonus",
    0x1038u16 => "oldArrest",
    0x103cu16 => "PoliceBonus",
    0x1040u16 => "DisasterObject",
    0x1044u16 => "CurrentDisaster",
    0x1048u16 => "GoDisaster",
    0x104cu16 => "SewerBonus",
    0x1050u16 => "Extra",
};

#[derive(Default, Debug, Serialize)]
struct Neighbour {
    name: Vec<i32>,
    population: Vec<i32>,
    value: Vec<i32>,
    fame: Vec<i32>,
}

impl IndexedWrite<i32> for Neighbour {
    fn field_len() -> usize {
        4
    }

    fn write(&mut self, index: usize, value: i32) {
        match index {
            0 => self.name.push(value),

            1 => self.population.push(value),

            2 => self.value.push(value),

            3 => self.fame.push(value),

            _ => panic!("index out of bounds"),
        }
    }
}

/// Class to store all of a city information, including buildings and all other tile contents, MISC city data, minimaps, etc.
/// Also handles serializing a city back out to a complaint .sc2 (or .scn file).
#[derive(Debug, Serialize)]
pub struct City {
    city_name: String,
    labels: Vec<String>,
    microsim_state: Vec<Box<[u8]>>,
    tilelist: HashMap<(usize, usize), Tile>,
    buildings: HashMap<(usize, usize), Arc<Building>>,
    networks: HashMap<(usize, usize), Arc<Building>>,
    groundcover: HashMap<(usize, usize), Arc<Building>>,
    things: Vec<Thing>,
    city_size: usize,
    graphs: HashMap<String, Graph>,
    city_attributes: HashMap<String, i32>,
    budget: Option<Budget>,
    neighbor_info: Vec<Neighbour>,
    building_count: Vec<i32>,
    game_settings: HashMap<String, i32>,
    inventions: HashMap<String, i32>,
    population_graphs: HashMap<String, Vec<i32>>,
    industry_graphs: HashMap<String, Vec<i32>>,
    simulator_settings: HashMap<String, i32>,
    traffic: Arc<Minimap>,
    pollution: Arc<Minimap>,
    value: Arc<Minimap>,
    crime: Arc<Minimap>,
    police: Arc<Minimap>,
    fire: Arc<Minimap>,
    density: Arc<Minimap>,
    growth: Arc<Minimap>,
    is_scenario: bool,
    scenario_text: String,
    scenario_descriptive_text: String,
    scenario_condition: HashMap<String, u32>,
    scenario_pict: Vec<u8>,
    original_filename: String,
}

impl City {
    fn new() -> Self {
        Self {
            city_name: String::from(""),
            labels: vec![],
            microsim_state: vec![],
            //graph_data: HashMap::new(),
            tilelist: HashMap::new(),
            buildings: HashMap::new(), // Note that this stores *only* buildings.
            networks: HashMap::new(), // Stores roads, rails, powerlines and other things that are above ground networks.
            groundcover: HashMap::new(), // Stores trees, rubble and radioactivity.
            things: vec![],
            city_size: 128,
            graphs: HashMap::new(),

            // Stuff from Misc
            city_attributes: HashMap::new(),
            budget: None, // original was null
            neighbor_info: vec![],
            building_count: Vec::with_capacity(256),
            simulator_settings: HashMap::new(),
            inventions: HashMap::new(),
            population_graphs: HashMap::new(),
            industry_graphs: HashMap::new(),

            game_settings: HashMap::new(),

            // Minimaps
            traffic: Arc::new(Minimap::new(String::from("traffic"), 64)),
            pollution: Arc::new(Minimap::new(String::from("pollution"), 64)),
            value: Arc::new(Minimap::new(String::from("value"), 64)),
            crime: Arc::new(Minimap::new(String::from("crime"), 64)),
            police: Arc::new(Minimap::new(String::from("police"), 32)),
            fire: Arc::new(Minimap::new(String::from("fire"), 32)),
            density: Arc::new(Minimap::new(String::from("density"), 32)),
            growth: Arc::new(Minimap::new(String::from("growth"), 32)),

            // Optional Scenario stuff
            is_scenario: false,
            scenario_text: String::from(""),
            scenario_descriptive_text: String::from(""),
            scenario_condition: HashMap::new(),
            scenario_pict: vec![],

            original_filename: String::from(""),
        }
    }

    /**
     * Creates the 8 minimaps.
     * Args:
     *      raw_sc2_data (bytes): Uncompressed .sc2 file.
     */
    fn create_minimaps(&mut self, raw_sc2_data: &ChunkList) {
        info!("parsing minimaps...");

        // Minimaps that map 4 tiles to 1.
        let map_size = self.city_size / 2;

        for x in 0..map_size {
            for y in 0..map_size {
                let tile_idx = x * map_size + y;
                let tile_key = (x, y);

                let xtrf = &raw_sc2_data.xtrf()[tile_idx..tile_idx + 1]
                    .try_into()
                    .expect("should be 1 byte");
                let xplt = &raw_sc2_data.xplt()[tile_idx..tile_idx + 1]
                    .try_into()
                    .expect("should be 1 byte");
                let xval = &raw_sc2_data.xval()[tile_idx..tile_idx + 1]
                    .try_into()
                    .expect("should be 1 byte");
                let xcrm = &raw_sc2_data.xcrm()[tile_idx..tile_idx + 1]
                    .try_into()
                    .expect("should be 1 byte");

                self.traffic = {
                    let mut traffic = (*self.traffic).clone();

                    traffic.set_item(tile_key, sc_util::parse_uint8(xtrf));

                    Arc::new(traffic)
                };

                self.pollution = {
                    let mut pollution = (*self.traffic).clone();

                    pollution.set_item(tile_key, sc_util::parse_uint8(xplt));

                    Arc::new(pollution)
                };

                self.value = {
                    let mut value = (*self.value).clone();

                    value.set_item(tile_key, sc_util::parse_uint8(xval));

                    Arc::new(value)
                };

                self.crime = {
                    let mut crime = (*self.crime).clone();

                    crime.set_item(tile_key, sc_util::parse_uint8(xcrm));

                    Arc::new(crime)
                };

                debug!(
                    "{:?}: traffic: {}, pollution: {}, land value: {}, crime: {}",
                    tile_key,
                    sc_util::parse_uint8(xtrf),
                    sc_util::parse_uint8(xplt),
                    sc_util::parse_uint8(xval),
                    sc_util::parse_uint8(xcrm)
                );
            }
        }

        // Minimaps that map 16 tiles to 1.
        // warning-ignore:integer_division
        let map_size_small = self.city_size / 4;

        for x in 0..map_size_small {
            for y in 0..map_size_small {
                let tile_idx = x * map_size_small + y;
                let tile_key = (x, y);

                let byte_range = tile_idx..(tile_idx + 1);

                let xplc = raw_sc2_data.xplc()[byte_range.clone()]
                    .try_into()
                    .expect("should be 1 byte");
                let xfir = raw_sc2_data.xfir()[byte_range.clone()]
                    .try_into()
                    .expect("should be 1 byte");
                let xpop = raw_sc2_data.xpop()[byte_range.clone()]
                    .try_into()
                    .expect("should be 1 byte");
                let xrog = raw_sc2_data.xrog()[byte_range.clone()]
                    .try_into()
                    .expect("should be 1 byte");

                self.police = {
                    let mut police = (*self.police).clone();

                    police.set_item(tile_key, sc_util::parse_uint8(xplc));

                    Arc::new(police)
                };

                self.fire = {
                    let mut fire = (*self.fire).clone();

                    fire.set_item(tile_key, sc_util::parse_uint8(xfir));

                    Arc::new(fire)
                };

                self.density = {
                    let mut density = (*self.density).clone();

                    density.set_item(tile_key, sc_util::parse_uint8(xpop));

                    Arc::new(density)
                };

                self.growth = {
                    let mut growth = (*self.growth).clone();

                    growth.set_item(tile_key, sc_util::parse_uint8(xrog));

                    Arc::new(growth)
                };

                debug!(
                    "{:?}: police: {}, fire: {}, densitye: {}, growth: {}",
                    tile_key,
                    sc_util::parse_uint8(xplc),
                    sc_util::parse_uint8(xfir),
                    sc_util::parse_uint8(xpop),
                    sc_util::parse_uint8(xrog)
                );
            }
        }
    }

    /**
     * Stores information about a tile.
     * Args:
     *      raw_sc2_data (bytes): Uncompressed .sc2 file.
     */
    fn create_tilelist(&mut self, raw_sc2_data: &ChunkList) {
        info!("parsing city terrain tiles...");

        for row in 0..self.city_size {
            for col in 0..self.city_size {
                let mut tile = Tile::new(
                    self.traffic.clone(),
                    self.pollution.clone(),
                    self.value.clone(),
                    self.crime.clone(),
                    self.police.clone(),
                    self.fire.clone(),
                    self.density.clone(),
                    self.growth.clone(),
                    self.labels.clone(),
                );
                let tile_idx = row * self.city_size + col;
                let tile_coords = (row, col);

                tile.set_coordinates(tile_coords);

                debug!("index: {} at {:?}", tile_idx, tile_coords);

                // First start with parsing the terrain related features.
                let altm = &raw_sc2_data.altm()[(tile_idx * 2)..(tile_idx * 2 + 2)]
                    .try_into()
                    .expect("should be 2 bytes");
                let xter = &raw_sc2_data.xter()[tile_idx..(tile_idx + 1)]
                    .try_into()
                    .expect("should be 1 byte");
                //			let altm_bits = sc_util::parse_uint16(altm);
                let altm_bits = sc_util::int_to_bitstring(sc_util::parse_uint16(altm) as u32, 16);
                //			tile.altitude_tunnel = altm_bits & 0xFF
                tile.set_altitude_tunnel(sc_util::parse_bitstring(&altm_bits[0..8]));
                //			tile.is_water = bool(altm_bits & 0x100)
                tile.set_is_water(sc_util::parse_bitstring(&altm_bits[8..9]) > 0);
                //			tile.altitude_unknown = altm_bits & 0x600
                tile.set_altitude_unknown(sc_util::parse_bitstring(&altm_bits[9..11]));
                //			tile.altitude = altm_bits & 0xF800
                tile.set_altitude(sc_util::parse_bitstring(&altm_bits[11..]));
                tile.set_terrain(sc_util::parse_uint8(xter));

                debug!("altm: {}, xter: {}", altm_bits, tile.terrain());

                // Next parse city stuff.
                // skip self.building for now, it's handled specially.
                let xzon = &raw_sc2_data.xzon()[tile_idx..(tile_idx + 1)]
                    .try_into()
                    .expect("should be 1 byte");
                let xzon_bits = sc_util::int_to_bitstring(sc_util::parse_uint8(xzon) as u32, 8);
                tile.set_zone_corners(xzon_bits[0..4].to_string());
                tile.set_zone(sc_util::parse_bitstring(&xzon_bits[4..]));
                let xund = &raw_sc2_data.xund()[tile_idx..(tile_idx + 1)]
                    .try_into()
                    .expect("should be 1 byte");
                tile.set_underground(sc_util::parse_uint8(xund));

                debug!(
                    "zone: {}, corners: {}, underground: {}",
                    tile.zone(),
                    tile.zone_corners(),
                    tile.underground()
                );

                // text/signs
                let xtxt = raw_sc2_data.xtxt()[tile_idx..(tile_idx + 1)]
                    .try_into()
                    .expect("should be 1 byte");
                tile.set_text_pointer(sc_util::parse_uint8(xtxt));

                // bit flags
                let xbit = raw_sc2_data.xbit()[tile_idx..(tile_idx + 1)]
                    .try_into()
                    .expect("should be 1 byte");
                tile.set_bit_flags(BitFlags::from(sc_util::parse_uint8(xbit)));

                debug!(
                    "text pointer: {:?}, bit flags: {:?}",
                    tile.text_pointer(),
                    tile.bit_flags()
                );

                // Add the new tile to the tilelist
                self.tilelist.insert((row, col), tile);
            }
        }
    }

    /**
     * Parses the label data.
     * Todo: Make handling of "special" labels easier.
     * Args:
     *      xlab_segment (bytes): XLAB sgement of the raw .sc2 file.
     */
    fn parse_labels(&mut self, xlab_segment: &[u8]) {
        info!("parsing labels...");

        for x in (0..xlab_segment.len()).step_by(25) {
            let label_id = x / 25;
            let raw_label = &xlab_segment[x..(x + 25)];
            let label_len =
                sc_util::parse_uint8(raw_label[0..1].try_into().expect("should be 1 byte"));

            if label_len == 0 {
                continue;
            }

            let label = String::from_utf8_lossy(&raw_label[1..(1 + label_len as usize)]);

            self.labels.push(label.clone().into_owned());

            debug!("Label: {label_id}: '{label}'");
        }
    }

    /**
     * Parses the label data.
     * Note that this is incomplete and contains the raw bytes presently.
     * Args:
     *      xmic_segment (bytes): XMIC sgement of the raw .sc2 file.
     */
    fn parse_microsim(&mut self, xmic_segment: &[u8]) {
        info!("parsing micro simulation data...");

        for x in (0..xmic_segment.len()).step_by(8) {
            let microsim_id = x / 8;
            let microsim = &xmic_segment[x..(x + 8)];

            self.microsim_state.push(Box::from(microsim));

            debug!("Raw Microsim: {microsim_id}: {microsim:?}");
        }
    }

    /**
     * Parses the XTHG segment.
     * Note: incompolete as XTHG segment spec not fully known.
     * Args:
     *      xthg_segments (bytes): Raw bytes representing the segment.
     */
    fn parse_things(&mut self, xthg_segments: &[u8]) {
        info!("parsing things...");

        for idx in (0..xthg_segments.len()).step_by(12) {
            let thing_data = xthg_segments[idx..(idx + 12)]
                .try_into()
                .expect("should be 12 bytes");
            let thing_index = idx / 12;
            let thing = Thing::parse_thing(thing_data);

            debug!("Index: {thing_index}, {}", thing.to_string());

            self.things.push(thing);
        }
    }

    /**
     * Parses the various graphs.
     * Args:
     *      xgrp_segment (bytes): Raw graph data to parse
     */
    fn parse_graphs(&mut self, xgrp_segment: &[u8]) {
        info!("parsing graphs...");

        let segment_len = 52 * 4;

        for (idx, graph_name) in GRAPH_WINDOW_GRAPHS.iter().enumerate() {
            let graph_start = idx * segment_len;
            let graph = Graph::parse_graph(
                xgrp_segment[graph_start..(graph_start + segment_len)]
                    .try_into()
                    .expect("should be 212 bytes"),
            );

            debug!("Graph: {graph_name}\n{}", graph.to_string());

            self.graphs.insert(graph_name.to_string(), graph);
        }
    }

    /**
     * Parses the scenario information.
     * Args:
     *     raw_city_data (bytes): Raw data to parse scenario information out of.
     */
    fn parse_scenario(&mut self, raw_city_data: &ChunkList) {
        info!("parsing city scenario...");

        self.is_scenario = true;

        let raw_text = raw_city_data.text();
        let raw_scenario = raw_city_data.scen();
        let picture = raw_city_data.pict();

        for entry in raw_text {
            let string_id = u32::from_be_bytes(entry[0..4].try_into().expect("should be 4 bytes"));
            let raw_string = String::from_utf8_lossy(&entry[4..entry.len()]).replace('\r', "\n");

            if string_id == 0x80000000 {
                self.scenario_text = raw_string
            } else if string_id == 0x81000000 {
                self.scenario_descriptive_text = raw_string
            } else {
                warn!("Found unknown TEXT block in input file.\nid: {string_id}, contents: \"{raw_string}\"");
            }
        }

        debug!(
            "Scenario:\nShort text: {}\nDescriptive Text: {}",
            self.scenario_text, self.scenario_descriptive_text
        );

        let mut conditions = HashMap::<String, u32>::new();
        let mut offset = 4;

        let contents = [
            ("disaster_type", 2),
            ("distater_x_location", 1),
            ("disaster_y_location", 1),
            ("time_limit_months", 2),
            ("city_size_goal", 4),
            ("residential_goal", 4),
            ("commercial_goal", 4),
            ("industrial_goal", 4),
            ("cash_flow_goal-bonds", 4),
            ("land_value_goal", 4),
            ("pollution_limit", 4),
            ("traffic_limit", 4),
            ("crime_limit", 4),
            ("build_item_one", 1),
            ("build_item_two", 1),
            ("item_one_tiles", 2),
            ("item_two_tiles", 2),
        ];

        for (k, v) in contents {
            conditions.insert(
                k.to_string(),
                sc_util::bytes_to_uint(&raw_scenario[offset..(offset + v)]),
            );

            offset += v;

            debug!("Conditions: {conditions:?}");
        }

        self.scenario_condition = conditions;

        let header = &picture[0..4];

        if header != [0x80, 0x00, 0x00, 0x00] {
            error!("Scenario PICT parsing failed. {header:?}"); //# todo: exception?
        }

        // Why is the endianness different here? It just is.
        let mut row_length: usize = 0;
        let mut row_count: usize = 0;

        for (idx, byte) in (&picture[4..6]).iter().enumerate() {
            // x dimension of image.
            row_length |= (*byte as usize) << idx;
        }

        for (idx, byte) in picture[6..8].iter().enumerate() {
            // y dimension of image.
            row_count |= (*byte as usize) << idx;
        }

        let mut image_data = vec![];
        let picture_data = &picture[8..picture.len()];

        debug!("Scenario PICT, {row_length}x{row_count} pixels:");

        for row_idx in 0..row_count {
            let row_start = row_idx * (row_length + 1);
            let mut row = vec![];

            for x in &picture_data[row_start..(row_start + row_length + 1)] {
                row.push(x.to_owned());
            }

            if *row.last().expect("should have at least one") != 255 {
                row = Vec::with_capacity(row_length);
            } else {
                row = row[0..(row.len() - 1)].to_vec();
            }

            image_data.append(&mut row);
        }

        for (idx, r) in image_data.iter().enumerate() {
            debug!("{idx}:\n{r}");
        }

        self.scenario_pict = image_data;
    }

    /**
     * Finds all of the buildings in a city file and creates a dict populated with Building objects with the keys being the x, y coordinates of the left corner.
     * Building generation algorighm:
     *      Scan for buildings by looking for their left corner. Why do it this way, which is obviously fragile? Because that's the way the original game did it, and this is attempting to replicate how the original game behaves.
     *      Once a building is found, look it up in XBLD to determine its size.
     *      Look for holes (from the magic eraser or other bugs in this building).
     *      Todo: find building either missing the left corner (rotation) or otherwise "broken" but still supported by the game.
     *      Buildings are stored as a dictionary, where a tile's xy coordinates are the key. Each tile of a building will point back to the same builiding object. This handles holes in the building.
     * Args:
     *      raw_sc2_data: Raw data for the city.
     */
    fn find_buildings(&mut self, raw_sc2_data: &ChunkList) -> Result<()> {
        info!("parsing city buildings...");

        // If the city has been rotated, then what is considered the left corrner changes.
        let city_rotation = self.simulator_settings["Compass"];
        let corner = [0b1000, 0b0001, 0b0010, 0b0100];
        let left_corner = corner[city_rotation as usize];

        debug!("City has rotation {}.", city_rotation);

        let raw_xbld = raw_sc2_data.xbld();

        for row in 0..self.city_size {
            for col in 0..self.city_size {
                // Find left corner.
                let zone_mask = self.tilelist[&(row, col)].zone_corners();
                let tile_idx = row * self.city_size + col;
                let building_id = raw_xbld[tile_idx];

                debug!("Checking building at: ({row}, {col})");
                debug!("Zone Mask: {}, {:04b}", zone_mask, left_corner);

                match zone_mask {
                    mask if sc_util::parse_bitstring(mask) & left_corner != 0 => {
                        let new_building = Arc::new(Building::new(building_id, (row, col)));

                        // Certain highway pieces are 2x2 buildings, but should only be in networks.
                        match building_id {
                            building_id if NETWORK_IDS.contains(&building_id) => {
                                self.networks.insert((row, col), new_building.clone());
                            }

                            _ => {
                                self.buildings.insert((row, col), new_building.clone());
                            }
                        }

                        match self.tilelist.get_mut(&(row, col)) {
                            Some(tile) => tile.set_building(new_building.clone()),
                            None => warn!("WARNING: no tile at ({row}, {col})"),
                        }

                        let building_size = buildings::get_size(&building_id)?;

                        debug!("Found Building: {building_id} with size: {building_size} at ({row}, {col})");

                        // Now we need to find the rest of the building.
                        if building_size == 1 {
                            continue;
                        }

                        // The clamping to 127 is to deal with certain industrial 3x3 buildings that glitch out on the edge of the map.
                        for building_x in row..min(row + building_size, 127) {
                            debug!("col: {}, building: {}", col, building_size);

                            for building_y in ((col - (building_size - 1))..=col).rev() {
                                let next_tile_idx = building_x * self.city_size + building_y;
                                let new_building_id = raw_xbld[next_tile_idx];

                                match new_building_id {
                                    new_building_id if new_building_id == building_id => {
                                        // Certain highway pieces are 2x2 buildings, but should only be in networks.
                                        if NETWORK_IDS.contains(&building_id) {
                                            self.networks.insert(
                                                (building_x, building_y),
                                                new_building.clone(),
                                            );
                                        } else {
                                            match self.tilelist.get_mut(&(building_x, building_y)) {
                                                Some(tile) => {
                                                    tile.set_building(new_building.clone())
                                                }
                                                None => warn!("WARNING: no tile at ({row}, {col})"),
                                            }
                                        }

                                        debug!(
                                            "Added Building: {} at ({}, {})",
                                            new_building_id, building_x, building_y
                                        );
                                    }

                                    _ => {
                                        warn!("Found hole at: ({}, {})", building_x, building_y);
                                        // This should probably be handled, but not yet.
                                    }
                                }
                            }
                        }
                    }

                    // Why are groundcover and networks treated differently?
                    // Because it seems (seemed?) to add flexibility.
                    _ if GROUNDCOVER_IDS.contains(&building_id) => {
                        let new_building = Arc::new(Building::new(building_id, (row, col)));

                        self.groundcover.insert((row, col), new_building.clone());

                        match self.tilelist.get_mut(&(row, col)) {
                            Some(tile) => tile.set_building(new_building),
                            None => warn!("WARNING: no tile at ({row}, {col})"),
                        }

                        debug!("Found groundcover: {} at ({}, {})", building_id, row, col);
                    }

                    // We've already added the highways to the network.
                    _ if NETWORK_IDS.contains(&building_id)
                        && !HIGHWAY_2X2_IDS.contains(&building_id) =>
                    {
                        let new_building = Arc::new(Building::new(building_id, (row, col)));

                        self.networks.insert((row, col), new_building.clone());

                        match self.tilelist.get_mut(&(row, col)) {
                            Some(tile) => tile.set_building(new_building),
                            None => warn!("WARNING: no tile at ({row}, {col})"),
                        }

                        debug!("Found network: {} at ({}, {})", building_id, row, col);
                    }

                    _ => {
                        debug!(
                            "Tile parsing fallthrough at ({}, {}) with id: {}",
                            row, col, building_id
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /**
     * Populates a city object from a .sc2 file.
     * Args:
     *      city_path: Path
     * Returns:
     *      Nothing, used to populate a city object from a file.
     */
    pub fn create_city_from_file(city_path: &Path) -> Result<Self> {
        let uncompressed_city = Self::open_and_uncompress_sc2_file(city_path)?;
        let mut city = Self::new();

        city.original_filename = city_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        city.name_city(&uncompressed_city);
        city.create_minimaps(&uncompressed_city);
        city.create_tilelist(&uncompressed_city);
        city.parse_misc(uncompressed_city.misc());
        city.find_buildings(&uncompressed_city)?;
        city.parse_labels(uncompressed_city.xlab());
        city.parse_microsim(uncompressed_city.xmic());
        city.parse_things(uncompressed_city.xthg());
        city.parse_graphs(uncompressed_city.xgrp());

        // Check for scenario.
        if uncompressed_city.text().is_empty()
            || uncompressed_city.scen().is_empty()
            || uncompressed_city.pict().is_empty()
        {
            return Ok(city);
        }

        city.parse_scenario(&uncompressed_city);
        Ok(city)
    }

    /**
     * Attempts to name the city based on the filename even if we can't parse a name for the city.
     * Truncates to 31 characters and coverts to all caps (as per original SC2k behaviour).
     * If the file is old enough and is missing a CNAM section, generates one using the same method.
     * Args:
     *      uncompressed_data: Uncompressed city data.
     */
    fn name_city(&mut self, uncompressed_data: &ChunkList) {
        let mut city_name = sc2p::clean_city_name(uncompressed_data.cnam());

        if city_name.is_empty() {
            let mut file_name: Vec<&str> = self.original_filename.split('.').collect();

            file_name.truncate(file_name.len() - 1);

            city_name = file_name.join(".").to_uppercase();
        }

        self.city_name = city_name;
        self.city_name.truncate(31);
    }

    /**
     * Handles opening and decompression of a city file.
     * Args:
     *      city_file_path: Path to the city file to be opened.
     * Returns:
     *      Uncompressed city data ready for parsing into something more usable.
     *      This takes the form of a dictionary with the keys being the 4-letter chunk headers from the sc2 IFF file, and the values being the uncompressed raw binary data in bytearray from.
     */
    fn open_and_uncompress_sc2_file(city_file_path: &Path) -> Result<ChunkList> {
        info!("reading file from {}...", city_file_path.to_string_lossy());
        let raw_sc2_file = sc_util::open_file(city_file_path);
        info!("reading city data chunks...");
        let compressed_data = sc2p::chunk_input_serial(&raw_sc2_file, "sc2")?;
        info!("decompressing city data chunks...");
        let uncompressed_data = sc2p::sc2_uncompress_input(compressed_data, "sc2");

        Ok(uncompressed_data)
    }

    /*
     * Parses the MISC section of the .sc2 file and populates the City object with its values.
     * See .sc2 file spec docs for more, at:
     * Args:
     *      misc_data (bytes): MISC segment of the raw data from the .sc2 file.
     */
    fn parse_misc(&mut self, misc_data: &[u8]) {
        info!("parsing city meta data, game settings and simulation settings...");
        // This is the offset of the section that's being parsed from MISC.
        let parse_order = &MISC_PARSE_ORDER;

        let mut handle_special = ([
            "Population Graphs",
            "Industry Graphs",
            "Tile Counts",
            "Bonds",
            "Neighbours",
            "Budget",
            "Military Count",
            "Paper List",
            "News List",
            "Extra",
            "Ordinances",
        ])
        .into_iter()
        .chain(SIMULATOR_SETTINGS_NAMES)
        .chain(GAME_SETTING_NAME)
        .chain(INVENTION_NAMES);

        // Parse misc and generate city attributes.
        for (k, v) in parse_order {
            let offset = *k as usize;

            match *v {
                "Population Graphs" => {
                    let length = 240;
                    self.population_graphs = Self::misc_uninterleave_data(
                        &POPULATION_GRAPH_NAMES,
                        offset,
                        length,
                        misc_data,
                    );
                }

                "Industry Graphs" => {
                    let length = 132;
                    self.industry_graphs = Self::misc_uninterleave_data(
                        &INDUSTRY_GRAPH_NAMES,
                        offset,
                        length,
                        misc_data,
                    );
                }

                "Tile Counts" => {
                    for x in 0..self.building_count.len() {
                        let offset = offset + x * 4;
                        self.building_count[x] = sc_util::parse_int32(
                            misc_data[offset..(offset + 4)]
                                .try_into()
                                .expect("should be 4 bytes"),
                        );
                    }
                }

                "Bonds" | "Ordinances" => {
                    // Handled along with the budget.
                }

                "Neighbours" => {
                    // Calculate their offsets. 64 = 4 neighbours at 4 x 4B entries each

                    for start_offset in (offset..(offset + 64)).step_by(16) {
                        // 16 = 4 entries x 4B per entry.
                        let mut neighbour = Neighbour::default();

                        for x in (start_offset..(start_offset + 16)).step_by(4) {
                            let index = ((x + 8) % 16) / 4;

                            neighbour.write(
                                index,
                                sc_util::parse_int32(
                                    misc_data[x..(x + 4)].try_into().expect("should be 4 bytes"),
                                ),
                            );
                        }

                        self.neighbor_info.push(neighbour);
                    }
                }

                "Budget" => {
                    let budget = Budget::from_misc_data(misc_data);

                    self.budget = Some(budget);
                }

                "Military Count" => {
                    let num_items = 16;

                    for (idx, x) in (offset..(offset + num_items * 4)).step_by(4).enumerate() {
                        let key = format!("{v}|{idx}");

                        self.city_attributes.insert(
                            key,
                            sc_util::parse_int32(
                                misc_data[x..(x + 4)].try_into().expect("should be 4 bytes"),
                            ),
                        );
                    }
                }

                "Paper List" => {
                    let num_items = 6 * 5;

                    for (idx, x) in (offset..(offset + num_items * 4)).step_by(4).enumerate() {
                        let key = format!("{v}|{idx}");

                        self.city_attributes.insert(
                            key,
                            sc_util::parse_int32(
                                misc_data[x..(x + 4)].try_into().expect("should be 4 bytes"),
                            ),
                        );
                    }
                }

                "News List" => {
                    let num_items = 9 * 6;

                    for (idx, x) in (offset..(offset + num_items * 4)).step_by(4).enumerate() {
                        let key = format!("{v}|{idx}");

                        self.city_attributes.insert(
                            key,
                            sc_util::parse_int32(
                                misc_data[x..(x + 4)].try_into().expect("should be 4 bytes"),
                            ),
                        );
                    }
                }

                "Extra" => {
                    for (idx, x) in (offset..4800).step_by(4).enumerate() {
                        let key = format!("{v}|{idx}");

                        self.city_attributes.insert(
                            key,
                            sc_util::parse_int32(
                                misc_data[x..(x + 4)].try_into().expect("should be 4 bytes"),
                            ),
                        );
                    }
                }

                value if SIMULATOR_SETTINGS_NAMES.contains(&value) => {
                    self.simulator_settings.insert(
                        v.to_string(),
                        sc_util::parse_int32(
                            misc_data[offset..(offset + 4)]
                                .try_into()
                                .expect("should be 4 bytes"),
                        ),
                    );
                }

                value if GAME_SETTING_NAME.contains(&value) => {
                    self.game_settings.insert(
                        v.to_string(),
                        sc_util::parse_int32(
                            misc_data[offset..(offset + 4)]
                                .try_into()
                                .expect("should be 4 bytes"),
                        ),
                    );
                }

                value if INVENTION_NAMES.contains(&value) => {
                    self.inventions.insert(
                        v.to_string(),
                        sc_util::parse_int32(
                            misc_data[offset..(offset + 4)]
                                .try_into()
                                .expect("should be 4 bytes"),
                        ),
                    );
                }

                v if !handle_special.any(|item| v == item) => {
                    let value = sc_util::parse_int32(
                        misc_data[offset..(offset + 4)]
                            .try_into()
                            .expect("should be 4 bytes"),
                    );

                    self.city_attributes.insert(v.to_string(), value);
                }

                _ => {
                    // Fallthrough, this should never, ever, be hit.
                    error!("MISC is missing something! k: {}, v: {}", k, v);
                }
            }
        }
    }

    /**
     * Args:
     *      keys (): list of keys? representing the data we want to parse.
     *      offset (int): Offset into MISC where the segment we want to uninterleave starts.
     *      length (int): Total length of the section.
     *      misc_data: Data from the MISC section that needs to be uninterlaved
     * Returns:
     *      A dictionary with the key being .
     */
    fn misc_uninterleave_data(
        keys: &[&str],
        offset: usize,
        length: usize,
        misc_data: &[u8],
    ) -> HashMap<String, Vec<i32>> {
        let num_keys = keys.len();
        let mut values: Vec<Vec<i32>> = vec![];

        for _ in 0..num_keys {
            values.push(vec![]);
        }

        for (idx, local_offset) in (offset..(offset + length)).step_by(4).enumerate() {
            let data = sc_util::parse_int32(
                misc_data[local_offset..(local_offset + 4)]
                    .try_into()
                    .expect("should be 4 bytes"),
            );
            values[idx % num_keys].push(data);
        }

        let mut output = HashMap::<String, Vec<i32>>::new();

        for (idx, key_name) in keys.iter().enumerate() {
            output.insert(key_name.to_string(), values[idx].clone());
        }

        output
    }
}

use super::indexed_write::{IndexedWrite, IndexedWriter};
use super::sc_util::{bytes_to_int32s, int_to_bitstring, parse_bitstring, parse_uint32};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default, Debug, Serialize)]
struct SubBudget {
    current_count: usize,
    current_funding: usize,
    unknown: usize,
    jan_count: usize,
    jan_funding: usize,
    feb_count: usize,
    feb_funding: usize,
    mar_count: usize,
    mar_funding: usize,
    apr_count: usize,
    apr_funding: usize,
    may_count: usize,
    may_funding: usize,
    jun_count: usize,
    jun_funding: usize,
    jul_count: usize,
    jul_funding: usize,
    aug_count: usize,
    aug_funding: usize,
    sep_count: usize,
    sep_funding: usize,
    oct_count: usize,
    oct_funding: usize,
    nov_count: usize,
    nov_funding: usize,
    dec_count: usize,
    dec_funding: usize,
}

impl IndexedWrite<usize> for SubBudget {
    fn write(&mut self, index: usize, value: usize) {
        match index {
            0 => self.current_count = value,
            1 => self.current_funding = value,
            2 => self.unknown = value,
            3 => self.jan_count = value,
            4 => self.jan_funding = value,
            5 => self.feb_count = value,
            6 => self.feb_funding = value,
            7 => self.mar_count = value,
            8 => self.mar_funding = value,
            9 => self.apr_count = value,
            10 => self.apr_funding = value,
            11 => self.may_count = value,
            12 => self.may_funding = value,
            13 => self.jun_count = value,
            14 => self.jun_funding = value,
            15 => self.jul_count = value,
            16 => self.jul_funding = value,
            17 => self.aug_count = value,
            18 => self.aug_funding = value,
            19 => self.sep_count = value,
            20 => self.sep_funding = value,
            21 => self.oct_count = value,
            22 => self.oct_funding = value,
            23 => self.nov_count = value,
            24 => self.nov_funding = value,
            25 => self.dec_count = value,
            26 => self.dec_funding = value,

            _ => panic!("index out of bounds"),
        }
    }

    fn field_len() -> usize {
        27
    }
}

/// Ordinances isn't really handled properly yet here, but it's here for now.
#[derive(EnumIter, Copy, Clone)]
enum SubBudgetIndices {
    Residential,
    Commercial,
    Industrial,
    Ordinances,
    Bonds,
    Police,
    Fire,
    Health,
    Schools,
    Colleges,
    Road,
    Hiway,
    Bridge,
    Rail,
    Subway,
    Tunnel,
}

impl Into<u16> for SubBudgetIndices {
    fn into(self) -> u16 {
        match self {
            Self::Residential => 0x077C,
            Self::Commercial => 0x07E8,
            Self::Industrial => 0x0854,
            Self::Ordinances => 0x08C0,
            Self::Bonds => 0x092C,
            Self::Police => 0x0998,
            Self::Fire => 0x0A04,
            Self::Health => 0x0A70,
            Self::Schools => 0x0ADC,
            Self::Colleges => 0x0B48,
            Self::Road => 0x0BB4,
            Self::Hiway => 0x0C20,
            Self::Bridge => 0x0C8C,
            Self::Rail => 0x0CF8,
            Self::Subway => 0x0D64,
            Self::Tunnel => 0x0DD0,
        }
    }
}

#[derive(Default, Debug, Serialize)]
struct BudgetItems {
    residential: SubBudget,
    commercial: SubBudget,
    industrial: SubBudget,
    ordinances: SubBudget,
    bonds: SubBudget,
    police: SubBudget,
    fire: SubBudget,
    health: SubBudget,
    schools: SubBudget,
    colleges: SubBudget,
    road: SubBudget,
    hiway: SubBudget,
    bridge: SubBudget,
    rail: SubBudget,
    subway: SubBudget,
    tunnel: SubBudget,
}

trait EnumMap<K, V> {
    fn get(&self, key: K) -> &V;
    fn get_mut(&mut self, key: K) -> &mut V;
}

impl EnumMap<SubBudgetIndices, SubBudget> for BudgetItems {
    fn get(&self, key: SubBudgetIndices) -> &SubBudget {
        match key {
            SubBudgetIndices::Residential => &self.residential,
            SubBudgetIndices::Commercial => &self.commercial,
            SubBudgetIndices::Industrial => &self.industrial,
            SubBudgetIndices::Ordinances => &self.ordinances,
            SubBudgetIndices::Bonds => &self.bonds,
            SubBudgetIndices::Police => &self.police,
            SubBudgetIndices::Fire => &self.fire,
            SubBudgetIndices::Health => &self.health,
            SubBudgetIndices::Schools => &self.schools,
            SubBudgetIndices::Colleges => &self.colleges,
            SubBudgetIndices::Road => &self.road,
            SubBudgetIndices::Hiway => &self.hiway,
            SubBudgetIndices::Bridge => &self.bridge,
            SubBudgetIndices::Rail => &self.rail,
            SubBudgetIndices::Subway => &self.subway,
            SubBudgetIndices::Tunnel => &self.tunnel,
        }
    }

    fn get_mut(&mut self, key: SubBudgetIndices) -> &mut SubBudget {
        match key {
            SubBudgetIndices::Residential => &mut self.residential,
            SubBudgetIndices::Commercial => &mut self.commercial,
            SubBudgetIndices::Industrial => &mut self.industrial,
            SubBudgetIndices::Ordinances => &mut self.ordinances,
            SubBudgetIndices::Bonds => &mut self.bonds,
            SubBudgetIndices::Police => &mut self.police,
            SubBudgetIndices::Fire => &mut self.fire,
            SubBudgetIndices::Health => &mut self.health,
            SubBudgetIndices::Schools => &mut self.schools,
            SubBudgetIndices::Colleges => &mut self.colleges,
            SubBudgetIndices::Road => &mut self.road,
            SubBudgetIndices::Hiway => &mut self.hiway,
            SubBudgetIndices::Bridge => &mut self.bridge,
            SubBudgetIndices::Rail => &mut self.rail,
            SubBudgetIndices::Subway => &mut self.subway,
            SubBudgetIndices::Tunnel => &mut self.tunnel,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Budget {
    budget_items: BudgetItems,
    #[serde(serialize_with = "serialize_array")]
    bonds: [i32; 50],
    ordinance_flags: [u32; 20],
}

impl Budget {
    /**
     * Parses the budget data segment from MISC into budget data.
     * Args:
     * 		raw_misc_data (bytes): Raw segment from Misc
     */
    pub fn from_misc_data(raw_misc_data: &[u8]) -> Self {
        // Ordinances
        let ordinance_raw: &[u8; 4] = &raw_misc_data[0x0FA0..(0x0FA0 + 4)]
            .try_into()
            .expect("we should have 4 bytes");

        let mut ordinance_flags = [u32::default(); 20];

        for (index, char) in int_to_bitstring(parse_uint32(ordinance_raw), 0)
            .chars()
            .enumerate()
        {
            ordinance_flags[index] = parse_bitstring(&char.to_string());
        }

        // bonds
        let start_offset = 0x0610;
        let bonds_len = 50 * 4;
        let bonds_end = start_offset + bonds_len;
        let mut bonds = [i32::default(); 50];
        let mut parsed_bonds = bytes_to_int32s(&raw_misc_data[start_offset..bonds_end]);

        bonds.swap_with_slice(parsed_bonds.as_mut_slice());

        let mut budget_items = BudgetItems::default();

        // various sub-budgets
        let sub_len = 27 * 4;
        let sub_end = sub_len;

        for name in SubBudgetIndices::iter() {
            let start_offset: u16 = name.into();

            let chunk =
                &raw_misc_data[(start_offset as usize)..((start_offset + sub_end) as usize)];
            let sub_budget = budget_items.get_mut(name);
            let chunk_data = bytes_to_int32s(chunk);
            let mut sub_budget_writer = IndexedWriter::new(sub_budget);

            for idx in 0..SubBudget::field_len() {
                sub_budget_writer.set_next(chunk_data[idx] as usize);
            }
        }

        Self {
            budget_items,
            bonds,
            ordinance_flags,
        }
    }
}

fn serialize_array<S: Serializer, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: IntoIterator,
    T::Item: Serialize,
{
    let collection: Vec<T::Item> = value.into_iter().collect();

    let mut sequence = serializer.serialize_seq(Some(collection.len()))?;

    for item in collection {
        sequence.serialize_element(&item)?;
    }

    sequence.end()
}

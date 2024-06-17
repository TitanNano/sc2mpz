use std::fmt::Display;

use super::sc_util::parse_int32;
use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct Graph {
    one_year: [i32; 12],
    ten_years: [i32; 20],
    hundred_years: [i32; 20],
}

impl Graph {
    pub fn parse_graph(raw_graphs: &[u8]) -> Self {
        let mut start = 0;
        let mut one_year: [i32; 12] = Default::default();
        let mut ten_years: [i32; 20] = Default::default();
        let mut hundred_years: [i32; 20] = Default::default();

        for x in (start..(start + 12 * 4)).step_by(4) {
            let idx = x / 4;

            one_year[idx] = parse_int32(
                &raw_graphs[x..(x + 4)]
                    .try_into()
                    .expect("should be 4 bytes"),
            );
        }

        start += 12 * 4;

        for x in (start..(start + 20 * 4)).step_by(4) {
            let idx = (x - start) / 4;

            ten_years[idx] = parse_int32(
                &raw_graphs[x..(x + 4)]
                    .try_into()
                    .expect("should be 4 bytes"),
            );
        }

        start += 20 * 4;

        for x in (start..(start + 20 * 4)).step_by(4) {
            let idx = (x - start) / 4;

            hundred_years[idx] =
                parse_int32(raw_graphs[x..x + 4].try_into().expect("should be 4 bytes"));
        }

        Self {
            one_year,
            ten_years,
            hundred_years,
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Year:\n\t{:?}.\n10 Years:\n\t{:?}.\n100 Years:\n\t{:?}.\n",
            self.one_year, self.ten_years, self.hundred_years
        )
    }
}

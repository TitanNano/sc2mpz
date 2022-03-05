mod open_city_2k;

use anyhow::Result;
use clap::Parser;
use open_city_2k::City;
use rmp_serde::encode;
// use serde_json::to_writer;
use gzp::{
    deflate::Gzip,
    par::compress::{ParCompress, ParCompressBuilder},
};
use log::info;
use simplelog::{
    ColorChoice as LoggerColorChoice, Config as LoggerConfig, LevelFilter, TermLogger, TerminalMode,
};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    #[clap(required = true)]
    sc2_file: Vec<PathBuf>,

    #[clap(short, long)]
    output: Option<PathBuf>,

    /// enables debug mode and output
    #[clap(short)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let log_level = if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        log_level,
        LoggerConfig::default(),
        TerminalMode::Mixed,
        LoggerColorChoice::Auto,
    )?;

    for path in args.sc2_file {
        process_file(&path, args.output.as_ref())?;
    }

    Ok(())
}

fn get_target_filename(path: &Path) -> String {
    format!(
        "{}.mpz",
        path.file_name()
            .unwrap_or(OsStr::new("city.sc2"))
            .to_string_lossy()
    )
}

fn process_file(input: &PathBuf, output: Option<&PathBuf>) -> Result<()> {
    let input = fs::canonicalize(input)?;
    let output = match output {
        Some(path) => {
            let mut out = path.to_owned();

            if out.file_name().is_none() {
                out.set_file_name(get_target_filename(&input));
            }

            out
        }

        None => {
            let mut path = input.clone();

            path.set_file_name(get_target_filename(&input));
            path
        }
    };

    let city = City::create_city_from_file(&input)?;
    let out_file = fs::File::create(&output)?;

    let mut compress: ParCompress<Gzip> = ParCompressBuilder::new().from_writer(out_file);

    info!("writing city to {}...", output.to_string_lossy());
    //to_writer(out_file, &city)?;
    encode::write_named(&mut compress, &city)?;

    info!("done!");
    Ok(())
}

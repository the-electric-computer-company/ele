use log;
use env_logger;
use failure::Error;

use std::path::PathBuf;

pub const ELE_DIR: &str = ".ele";

#[derive(Debug)]
pub struct Config {
  pub address: String,
  pub port: u16,
  pub log_level: log::Level,
  pub base_path: PathBuf,
}


pub fn run(config: Config) -> Result<(), Error> {
  env_logger::Builder::from_default_env()
    .filter_level(config.log_level.to_level_filter())
    .init();

  Ok(())
}

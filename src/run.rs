use failure::Error;
use std::path::PathBuf;
use std::thread;

#[derive(Debug)]
pub struct Config {
  pub address: String,
  pub port: u16,
  pub library_path: PathBuf,
}

pub fn run(_config: Config) -> Result<(), Error> {
  loop {
    thread::park();
  }
}

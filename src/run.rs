use log;
use env_logger;
use failure::Error;

use std::path::PathBuf;

use server;
use ::node_grpc::{NodeServer};
use grpc;
use std::thread;
use tls_api_native_tls;

pub const ELE_DIR: &str = ".ele";

#[derive(Debug)]
pub struct Config {
  pub address: String,
  pub port: u16,
  pub log_level: log::Level,
  pub base_path: PathBuf,
}

// TODO: secrets generation
// TODO: tls setup

pub fn run(config: Config) -> Result<(), Error> {
  env_logger::Builder::from_default_env()
    .filter_level(config.log_level.to_level_filter())
    .init();

  let mut server = grpc::ServerBuilder::<tls_api_native_tls::TlsAcceptor>::new();
  server.http.set_port(config.port);
  server.add_service(NodeServer::new_service_def(server::NodeImpl));
  server.http.set_cpu_pool_threads(4);
  let _server = server.build()?;

  loop {
    thread::park();
  }
}

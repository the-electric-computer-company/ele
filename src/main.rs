extern crate grpc;
extern crate protobuf;
extern crate tls_api;

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate clap;

use structopt::clap::AppSettings;
use structopt::StructOpt;
use failure::Error;

use std::env;

mod node;
mod node_grpc;
mod run;

arg_enum! {
  #[derive(Debug)]
  enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
  }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "ele", about = "Gateway to the world's content",
            raw(setting = "AppSettings::InferSubcommands"))]
enum Opt {
  #[structopt(name = "run")]
  Run {
    #[structopt(short = "l", long = "log-level", help = "Set log level", default_value = "error",
                raw(possible_values = "&LogLevel::variants()", case_insensitive = "true"))]
    log_level: LogLevel,


    #[structopt(short = "a", long = "address", help = "Set listen ip address", default_value = "127.0.0.1")]
    address: String,

    #[structopt(short = "p", long = "port", help = "Set listen port", default_value = "1337")]
    port: u16,
  }
}

fn main() -> Result<(), Error> {
  let opt = Opt::from_args();

  match opt {
    Opt::Run{log_level: opt_log_level, address, port} => {
      let log_level = match opt_log_level {
        LogLevel::Error => log::Level::Error,
        LogLevel::Warn => log::Level::Warn,
        LogLevel::Info => log::Level::Info,
        LogLevel::Debug => log::Level::Debug,
        LogLevel::Trace => log::Level::Trace,
      };

      let home_dir = env::home_dir().ok_or_else(|| format_err!("no home directory set"))?;
      let base_path = home_dir.join(run::ELE_DIR);

      let config = run::Config{
        log_level,
        address,
        port,
        base_path,
      };

      run::run(config)
    },
  }
}

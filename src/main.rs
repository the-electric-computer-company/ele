#[macro_use]
extern crate structopt;

extern crate env_logger;
extern crate failure;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;
extern crate tls_api_native_tls;
extern crate uuid;

mod node;
mod node_grpc;
mod run;

use failure::Error;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "ele",
  about = "💾 Share and enjoy",
  raw(setting = "AppSettings::InferSubcommands")
)]
enum Opt {
  #[structopt(name = "node")]
  Node {
    #[structopt(long = "address", help = "Set listen ip address", default_value = "0.0.0.0")]
    address: String,

    #[structopt(long = "port", help = "Set listen port", default_value = "2018")]
    port: u16,

    #[structopt(long = "library-path", help = "Set library path", parse(from_os_str))]
    library_path: Option<PathBuf>,
  },
}

fn main() -> Result<(), Error> {
  env_logger::Builder::from_default_env().init();

  let opt = Opt::from_args();

  match opt {
    Opt::Node {
      address,
      port,
      library_path,
    } => {
      let library_path = library_path.unwrap_or_else(|| {
        let mut path = PathBuf::new();
        path.push("/tmp");
        path.push("library.db");
        path
      });
      let config = run::Config {
        address,
        port,
        library_path,
      };

      run::run(config)
    }
  }
}

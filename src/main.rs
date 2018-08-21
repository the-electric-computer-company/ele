#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;

#[cfg(test)]
mod testing;

#[cfg(test)]
extern crate assert_fs;

#[cfg(test)]
extern crate predicates;

extern crate env_logger;
extern crate grpc;
extern crate protobuf;
extern crate rand;
extern crate rusqlite;
extern crate sha2;
extern crate tls_api;
extern crate tls_api_native_tls;
extern crate uuid;

#[macro_use]
mod show;

#[macro_use]
pub mod api;
mod common;
mod default;
mod error;
mod library;
mod node;
mod platform;
pub mod pubkey;
mod svc;

use common::*;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "ele", about = "💾 Share and enjoy", raw(setting = "AppSettings::InferSubcommands")
)]
enum Opt {
  #[structopt(name = "node")]
  Node {
    #[structopt(long = "library-path", help = "Set library path", parse(from_os_str))]
    library_path: Option<PathBuf>,
  },
}

fn main() -> Result<(), Error> {
  env_logger::Builder::from_default_env().init();

  let opt = Opt::from_args();

  match opt {
    Opt::Node { library_path } => {
      let library_path = if let Some(library_path) = library_path {
        library_path
      } else {
        Library::default_path()
      };

      let library = Library::with_path(library_path)?;

      let node = Node::new(library);

      node.run()
    }
  }
}

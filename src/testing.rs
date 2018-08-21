use common::*;

use env_logger;

/// `true` if running on Appveyor CI
pub fn running_on_appveyor() -> bool {
  env::var_os("APPVEYOR").is_some()
}

static FLAG: Once = ONCE_INIT;

pub fn test_init() {
  FLAG.call_once(|| env_logger::init());
}

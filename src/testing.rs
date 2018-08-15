use common::*;

/// `true` if running on Appveyor CI
pub fn running_on_appveyor() -> bool {
  env::var_os("APPVEYOR").is_some()
}

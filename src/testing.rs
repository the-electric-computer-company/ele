use common::*;

pub fn running_on_appveyor() -> bool {
  env::var_os("APPVEYOR").is_some()
}

use common::*;

use std::env;

pub struct Platform;

pub trait PlatformInterface {
  fn app_directory_base_path() -> PathBuf;
}

#[cfg(target_os = "windows")]
impl PlatformInterface for Platform {
  fn app_directory_base_path() -> PathBuf {
    if let Some(value) = env::var_os("LOCALAPPDATA") {
      PathBuf::from(value)
    } else {
      panic!("%LOCALAPPDATA% environment variable was unset")
    }
  }
}

#[cfg(target_os = "macos")]
impl PlatformInterface for Platform {
  fn app_directory_base_path() -> PathBuf {
    if let Some(value) = env::var_os("HOME") {
      let mut path = PathBuf::from(value);
      path.push("Library");
      path.push("Application Support");
      path
    } else {
      panic!("$HOME environment variable was unset")
    }
  }
}

#[cfg(target_os = "linux")]
impl PlatformInterface for Platform {
  fn app_directory_base_path() -> PathBuf {
    if let Some(value) = env::var_os("XDG_DATA_HOME") {
      PathBuf::from(value)
    } else if let Some(value) = env::var_os("HOME") {
      let mut path = PathBuf::from(value);
      path.push(".local");
      path.push("share");
      path
    } else {
      panic!("Both $XDG_DATA_HOME and $HOME environment variables were unset")
    }
  }
}

use common::*;

use rusqlite;

#[derive(Debug)]
pub enum Error {
  /// IO error occured accessing app directory or app directory ancestor
  AppDirectoryIo { io_error: io::Error, path: PathBuf },
  /// Application directory was not a directory
  AppDirectoryNotDirectory { path: PathBuf },
  /// Library file path had extension other than `.db`
  LibraryPathExtension { library_path: PathBuf },
  /// Error accessing the library database
  LibraryDatabase {
    sqlite_error: rusqlite::Error,
    library_path: PathBuf,
  },
}

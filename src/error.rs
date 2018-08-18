use common::*;

use rusqlite;

#[derive(Debug)]
pub enum Error {
  /// IO error occured accessing app directory or app directory ancestor
  AppDirectoryIo { io_error: io::Error, path: PathBuf },
  /// Application directory was not a directory
  AppDirectoryNotDirectory { path: PathBuf },
  /// Library file path had extension other than `.db`
  LibraryPathExtension { database_path: PathBuf },
  /// Error accessing the library database
  LibrarySqlite {
    sqlite_error: rusqlite::Error,
    statement: Option<String>,
    database_path: PathBuf,
  },
  /// Pre-existing library database had an invalid application id
  LibraryApplicationId {
    application_id: i32,
    database_path: PathBuf,
  },
  /// Node ID stored in library database was invalid
  LibraryStoredNodeId { pubkey_error: pubkey::Error },

  /// Collection ID stored in library database was invalid
  LibraryStoredCollectionId { pubkey_error: pubkey::Error },
}

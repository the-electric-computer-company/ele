use common::*;

use rusqlite::{self, types::FromSql, Connection, Row};

const SQLITE_DATABASE_APPLICATION_ID: i32 = 0x1337_0000;

#[derive(Debug)]
pub struct Library {
  connection: Mutex<Connection>,
  database_path: PathBuf,
}

impl Library {
  pub fn default_path() -> PathBuf {
    let mut path = Platform::app_directory_base_path();
    path.push("ele");
    path.push("library.db");
    path
  }

  pub fn with_path(database_path: impl AsRef<Path>) -> Result<Library, Error> {
    let database_path = database_path.as_ref().to_path_buf();

    if database_path.extension() != Some(OsStr::new("db")) {
      return Err(Error::LibraryPathExtension {
        database_path: database_path.clone(),
      });
    }

    {
      // This unwrap is safe, since we assert above that the
      // path must have an extension, which means it must
      // have at least one component
      let app_directory_path = database_path.parent().unwrap();

      let mut ancestors = app_directory_path.ancestors().collect::<Vec<&Path>>();
      ancestors.reverse();

      for ancestor in ancestors {
        if ancestor == Path::new("") {
          continue;
        }

        match ancestor.metadata() {
          Err(ref io_error) if io_error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir(ancestor).map_err(|io_error| Error::AppDirectoryIo {
              io_error,
              path: ancestor.to_path_buf(),
            })?;
          }

          Err(io_error) => {
            return Err(Error::AppDirectoryIo {
              io_error,
              path: ancestor.to_path_buf(),
            })
          }

          Ok(metadata) => if !metadata.is_dir() {
            return Err(Error::AppDirectoryNotDirectory {
              path: ancestor.to_path_buf(),
            });
          },
        }
      }
    }

    let pre_existing = database_path.exists();

    if pre_existing {
      info!(
        "Opening pre-existing library database at `{}`...",
        database_path.display()
      );
    } else {
      info!(
        "Creating new library database at `{}`...",
        database_path.display()
      );
    }

    let connection =
      Connection::open(&database_path).map_err(|sqlite_error| Error::LibrarySqlite {
        sqlite_error,
        statement: None,
        database_path: database_path.clone(),
      })?;

    let library = Library {
      connection: Mutex::new(connection),
      database_path,
    };

    if pre_existing {
      library.check_application_id()?;
    } else {
      library.set_application_id()?;
      library.set_journal_mode()?;
    }

    Ok(library)
  }

  fn application_id(&self) -> Result<i32, Error> {
    let statement = "PRAGMA application_id;";

    self.query_scalar::<i32>(statement)
  }

  fn set_application_id(&self) -> Result<(), Error> {
    let statement = format!(
      "PRAGMA application_id = {};",
      SQLITE_DATABASE_APPLICATION_ID
    );

    self.execute(&statement)
  }

  fn set_journal_mode(&self) -> Result<(), Error> {
    self.query_scalar::<String>("PRAGMA journal_mode = WAL;")?;
    Ok(())
  }

  fn execute(&self, statement: &str) -> Result<(), Error> {
    self
      .connection
      .lock()
      .expect("library connection lock poisoned")
      .execute(statement, &[])
      .embellish(self, statement)
      .map(|_| ())
  }

  fn query_scalar<T: FromSql>(&self, statement: &str) -> Result<T, Error> {
    fn get<T: FromSql>(row: &Row) -> T {
      row.get(0)
    }

    self
      .connection
      .lock()
      .expect("library connection lock poisoned")
      .query_row(statement, &[], get::<T>)
      .embellish(self, statement)
  }

  fn check_application_id(&self) -> Result<(), Error> {
    let application_id = self.application_id()?;

    if application_id != SQLITE_DATABASE_APPLICATION_ID {
      Err(Error::LibraryApplicationId {
        application_id,
        database_path: self.database_path.clone(),
      })
    } else {
      Ok(())
    }
  }
}

/// Extension trait for Result<T, rusqlite::Error>
trait SqliteResultExt<T> {
  /// Convert a `rusqlite::Error` to an `Error`, which additionally includes
  /// the path to the library database, as well as the statement that caused
  /// the error.
  fn embellish(self, library: &Library, statement: &str) -> Result<T, Error>;
}

impl<T> SqliteResultExt<T> for Result<T, rusqlite::Error> {
  fn embellish(self, library: &Library, statement: &str) -> Result<T, Error> {
    self.map_err(|sqlite_error| Error::LibrarySqlite {
      sqlite_error,
      statement: Some(statement.to_string()),
      database_path: library.database_path.clone(),
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use assert_fs::prelude::*;
  use assert_fs::TempDir;
  use predicates::prelude::*;

  #[test]
  fn base_path_correct_suffix() {
    let default_path = Library::default_path();

    let components = default_path.components().collect::<Vec<_>>();

    use std::path::Component::*;

    if cfg!(windows) {
      assert_eq!(components[1], RootDir, "path not absolute");
    } else {
      assert_eq!(components[0], RootDir, "path not absolute");
    }

    assert_eq!(components[components.len() - 2], Normal(OsStr::new("ele")),);

    assert_eq!(
      components[components.len() - 1],
      Normal(OsStr::new("library.db")),
    );
  }

  #[test]
  fn db_extension_required() {
    match Library::with_path("foo.bar") {
      Err(Error::LibraryPathExtension { database_path }) => {
        assert_eq!(database_path, PathBuf::from("foo.bar"))
      }
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }
  }

  #[test]
  fn creation() {
    let tempdir = TempDir::new().unwrap();

    let db = tempdir.child("hello.db");

    let library = Library::with_path(db.path()).unwrap();

    db.assert(predicate::path::is_file());

    assert_eq!(
      library
        .query_scalar::<String>("PRAGMA journal_mode;")
        .unwrap(),
      "wal"
    );
  }

  #[test]
  fn library_with_non_directory_path() {
    let tempdir = TempDir::new().unwrap();

    let foo = tempdir.child("foo");
    foo.touch().unwrap();

    let db = tempdir.child("foo/library.db");

    match Library::with_path(db.path()) {
      Err(Error::AppDirectoryNotDirectory { path }) => {
        assert_eq!(path, foo.path(), "unexpected path")
      }
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }
  }

  #[test]
  fn intermediate_directory_creation() {
    let tempdir = TempDir::new().unwrap();

    let foo = tempdir.child("foo");

    let bar = tempdir.child("foo/bar");

    let db = tempdir.child("foo/bar/library.db");

    Library::with_path(db.path()).unwrap();

    foo.assert(predicate::path::is_dir());
    bar.assert(predicate::path::is_dir());
  }

  #[test]
  fn no_permission() {
    // Appveyor tests run as the superuser, who ignores the
    // read-only permission, which causes this test to fail.
    if running_on_appveyor() {
      return;
    }

    use rusqlite::Error::SqliteFailure;

    let tempdir = TempDir::new().unwrap();

    let sub = tempdir.child("sub");
    let path = sub.path();

    fs::create_dir(path).unwrap();

    let mut permissions = path.metadata().unwrap().permissions();

    permissions.set_readonly(true);

    fs::set_permissions(path, permissions).unwrap();

    let db = tempdir.child("sub/library.db");

    match Library::with_path(db.path()) {
      Err(Error::LibrarySqlite {
        sqlite_error: SqliteFailure(_, Some(message)),
        statement: None,
        database_path,
      }) => {
        assert_eq!(message, "unable to open database file");
        assert_eq!(database_path, db.path().to_path_buf());
      }
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }
  }

  #[test]
  fn app_directory_io_error() {
    // Appveyor tests run as the superuser, who ignores the
    // read-only permission, which causes this test to fail.
    if running_on_appveyor() {
      return;
    }

    let tempdir = TempDir::new().unwrap();

    let sub = tempdir.child("sub");
    let path = sub.path();

    fs::create_dir(path).unwrap();

    let mut permissions = path.metadata().unwrap().permissions();

    permissions.set_readonly(true);

    let foo = tempdir.child("sub/foo");

    fs::set_permissions(path, permissions).unwrap();

    let db = tempdir.child("sub/foo/library.db");

    match Library::with_path(db.path()) {
      Err(Error::AppDirectoryIo { path, .. }) => assert_eq!(path, foo.path()),
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }
  }

  #[test]
  fn bad_app_id() {
    {
      let tempdir = TempDir::new().unwrap();

      let db = tempdir.child("library.db");

      Connection::open(db.path()).unwrap();

      match Library::with_path(db.path()) {
        Err(Error::LibraryApplicationId {
          database_path,
          application_id,
        }) => {
          assert_eq!(application_id, 0);
          assert_eq!(database_path, db.path());
        }
        otherwise => panic!("unexpected result: {:?}", otherwise),
      }
    }

    {
      let tempdir = TempDir::new().unwrap();

      let db = tempdir.child("library.db");

      Connection::open(db.path())
        .unwrap()
        .execute("PRAGMA application_id = 10;", &[])
        .unwrap();

      match Library::with_path(db.path()) {
        Err(Error::LibraryApplicationId {
          database_path,
          application_id,
        }) => {
          assert_eq!(application_id, 10);
          assert_eq!(database_path, db.path());
        }
        otherwise => panic!("unexpected result: {:?}", otherwise),
      }
    }
  }
}

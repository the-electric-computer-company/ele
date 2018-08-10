use common::*;

use rusqlite::Connection;

#[derive(Debug)]
pub struct Library {
  _connection: Connection,
}

impl Library {
  pub fn default_path() -> PathBuf {
    let mut path = Platform::app_directory_base_path();
    path.push("ele");
    path.push("library.db");
    path
  }

  pub fn with_path(library_path: impl AsRef<Path>) -> Result<Library, Error> {
    let library_path = library_path.as_ref().to_path_buf();

    if library_path.extension() != Some(OsStr::new("db")) {
      return Err(Error::LibraryPathExtension {
        library_path: library_path.clone(),
      });
    }

    {
      // This unwrap is safe, since we assert above that the
      // path must have an extension, which means it must
      // have at least one component
      let app_directory_path = library_path.parent().unwrap();

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

    let pre_existing = library_path.exists();

    if pre_existing {
      info!(
        "Opening pre-existing library database at `{}`...",
        library_path.display()
      );
    } else {
      info!(
        "Creating new library database at `{}`...",
        library_path.display()
      );
    }

    let connection =
      Connection::open(&library_path).map_err(|sqlite_error| Error::LibraryDatabase {
        sqlite_error,
        library_path,
      })?;

    Ok(Library {
      _connection: connection,
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
  fn base_path_correct_suffix() -> Result<(), Error> {
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

    Ok(())
  }

  #[test]
  fn db_extension_required() {
    match Library::with_path("foo.bar") {
      Err(Error::LibraryPathExtension { library_path }) => {
        assert_eq!(library_path, PathBuf::from("foo.bar"))
      }
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }
  }

  #[test]
  fn library_with_path() -> io::Result<()> {
    let tempdir = TempDir::new()?;

    let db = tempdir.child("hello.db");

    Library::with_path(db.path()).unwrap();

    db.assert(predicate::path::is_file());

    Ok(())
  }

  #[test]
  fn library_with_non_directory_path() -> io::Result<()> {
    let tempdir = TempDir::new()?;

    let foo = tempdir.child("foo");
    foo.touch()?;

    let db = tempdir.child("foo/library.db");

    match Library::with_path(db.path()) {
      Err(Error::AppDirectoryNotDirectory { path }) => {
        assert_eq!(path, foo.path(), "unexpected path")
      }
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }

    Ok(())
  }

  #[test]
  fn intermediate_directory_creation() -> io::Result<()> {
    let tempdir = TempDir::new()?;

    let foo = tempdir.child("foo");

    let bar = tempdir.child("foo/bar");

    let db = tempdir.child("foo/bar/library.db");

    Library::with_path(db.path()).unwrap();

    foo.assert(predicate::path::is_dir());
    bar.assert(predicate::path::is_dir());

    Ok(())
  }

  #[test]
  fn no_permission() -> io::Result<()> {
    // Appveyor tests run as the superuser, who ignores the
    // read-only permission, which causes this test to fail.
    if env::var_os("APPVEYOR").is_some() {
      return Ok(());
    }

    use rusqlite::Error::SqliteFailure;

    let tempdir = TempDir::new()?;

    let sub = tempdir.child("sub");
    let path = sub.path();

    fs::create_dir(path)?;

    let mut permissions = path.metadata()?.permissions();

    permissions.set_readonly(true);

    fs::set_permissions(path, permissions)?;

    let db = tempdir.child("sub/library.db");

    match Library::with_path(db.path()) {
      Err(Error::LibraryDatabase {
        sqlite_error: SqliteFailure(_, Some(message)),
        library_path,
      }) => {
        assert_eq!(message, "unable to open database file");
        assert_eq!(library_path, db.path().to_path_buf());
      }
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }

    Ok(())
  }

  #[test]
  fn app_directory_io_error() -> io::Result<()> {
    let tempdir = TempDir::new()?;

    let sub = tempdir.child("sub");
    let path = sub.path();

    fs::create_dir(path)?;

    let mut permissions = path.metadata()?.permissions();

    permissions.set_readonly(true);

    let foo = tempdir.child("sub/foo");

    fs::set_permissions(path, permissions)?;

    let db = tempdir.child("sub/foo/library.db");

    match Library::with_path(db.path()) {
      Err(Error::AppDirectoryIo { path, .. }) => assert_eq!(path, foo.path()),
      otherwise => panic!("unexpected result: {:?}", otherwise),
    }

    Ok(())
  }
}

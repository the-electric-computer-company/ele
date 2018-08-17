use super::*;

const ERROR_PARSE: u32 = 1;

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
  pub message: String,
  pub kind: ErrorKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorKind {
  Unknown { code: u32 },
  Parse,
}

impl ErrorKind {
  pub fn into_error(self, message: impl AsRef<str>) -> Error {
    Error {
      message: message.as_ref().to_string(),
      kind: self,
    }
  }
}

impl Error {
  pub fn code(&self) -> u32 {
    use self::ErrorKind::*;
    match self.kind {
      Unknown { code } => code,
      Parse => ERROR_PARSE,
    }
  }

  pub fn into_protobuf(self) -> svc::Error {
    let mut pb_error = svc::Error::new();
    pb_error.set_code(self.code());
    pb_error.set_message(self.message);
    pb_error
  }

  pub fn from_protobuf(mut pb_error: svc::Error) -> Error {
    let code = pb_error.get_code();
    let message = pb_error.take_message();

    use self::ErrorKind::*;
    let kind = match code {
      ERROR_PARSE => Parse,
      _ => Unknown { code },
    };
    Error { message, kind }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn error() {
    let first_error = ErrorKind::Parse.into_error("foo");
    let pb_err = first_error.clone().into_protobuf();
    let second_error = Error::from_protobuf(pb_err);
    assert_eq!(second_error, first_error);
  }

}

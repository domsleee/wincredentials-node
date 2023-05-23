use napi::{Error, Status};
use napi_derive::napi;
use wincredentials::credential::Credential;
#[napi]
/// Writes a credential.
pub fn write_credential(target: String, username: String, secret: String) -> Result<(), Error> {
  match wincredentials::write_credential(&target, Credential { username, secret }) {
    Ok(v) => Ok(v),
    Err(e) => Err(Error::new(
      Status::InvalidArg,
      format!("unable to write_credential: {e}"),
    )),
  }
}

#[napi]
/// Reads a credential. Throws Error with code: 'InvalidArg' if the credential doesn't exist
pub fn read_credential(target: String) -> Result<NapiCredentials, Error> {
  let credential = match wincredentials::read_credential(&target) {
    Ok(v) => v,
    Err(e) => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("unable to read_credential: {e}"),
      ))
    }
  };
  Ok(NapiCredentials::from_credential(credential))
}

#[napi]
/// Deletes a credential. Throws Error with code: 'InvalidArg' if the credential doesn't exist
pub fn delete_credential(target: String) -> Result<(), Error> {
  match wincredentials::delete_credential(&target) {
    Ok(v) => Ok(v),
    Err(e) => Err(Error::new(
      Status::InvalidArg,
      format!("unable to delete_credential: {e}"),
    )),
  }
}

#[napi(object)]
pub struct NapiCredentials {
  pub username: String,
  pub secret: String,
}

impl NapiCredentials {
  pub fn from_credential(credential: Credential) -> Self {
    Self {
      username: credential.username,
      secret: credential.secret,
    }
  }
}

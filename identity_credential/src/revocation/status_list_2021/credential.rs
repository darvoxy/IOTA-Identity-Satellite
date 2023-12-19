use std::borrow::Cow;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use identity_core::common::Context;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

const CREDENTIAL_TYPE: &str = "StatusList2021Credential";
const CREDENTIAL_SUBJECT_TYPE: &str = "StatusList2021";

#[derive(Clone, Debug, Error)]
/// [Error](std::error::Error) type that represents the possible errors that can be
/// encountered when dealing with [`StatusList2021Credential`]s
pub enum StatusList2021CredentialError {
  #[error("A StatusList2021Credential may only have one credentialSubject")]
  /// The provided [`Credential`] has more than one `credentialSubject`
  MultipleCredentialSubject,
  #[error("Invalid property {0}")]
  /// The provided [`Credential`] has an invalid property
  InvalidProperty(String),
}

use crate::credential::Credential;
use crate::credential::CredentialBuilder;
use crate::credential::Issuer;
use crate::credential::Proof;

use super::status_list::InvalidEncodedStatusList;
use super::StatusList2021;

#[derive(Debug, Eq, PartialEq, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
/// A parsed [StatusList2021Credential](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021credential)
pub struct StatusList2021Credential(Credential);

impl Deref for StatusList2021Credential {
  type Target = Credential;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<Credential> for StatusList2021Credential {
  type Error = StatusList2021CredentialError;
  fn try_from(credential: Credential) -> Result<Self, Self::Error> {
    let has_right_credential_type = credential.types.contains(&CREDENTIAL_TYPE.to_owned());
    let _subject = StatusList2021CredentialSubject::try_from_credential(&credential)?;

    if has_right_credential_type {
      Ok(Self(credential))
    } else {
      Err(StatusList2021CredentialError::InvalidProperty("type".to_owned()))
    }
  }
}

impl StatusList2021Credential {
  /// Returns the inner "raw" [`Credential`]
  pub fn into_inner(self) -> Credential {
    self.0
  }
  /// Returns the purpose of this status list
  pub fn purpose(&self) -> StatusPurpose {
    let subject = StatusList2021CredentialSubject::try_from_credential(&self.0).unwrap(); // Safety: `Self` has already been validated as a valid StatusList2021Credential
    subject.status_purpose
  }
  /// Updates the [`StatusList2021`] stored in this credential, applying `f` to it
  pub fn update_status_list<F>(&mut self, f: F) -> Result<(), InvalidEncodedStatusList>
  where
    F: FnOnce(&mut StatusList2021),
  {
    use identity_core::common::Value;

    let new_encoded_status_list = {
      let status_list_credential = StatusList2021CredentialSubject::try_from_credential(&self.0).unwrap();
      let mut status_list = StatusList2021::try_from_encoded_str(&status_list_credential.encoded_list)?;
      f(&mut status_list);
      status_list.into_encoded_str()
    };
    let OneOrMany::One(subject) = &mut self.0.credential_subject else {
      unreachable!("already validated");
    };
    subject
      .properties
      .entry("encodedList".to_owned())
      .and_modify(|value| *value = Value::String(new_encoded_status_list));

    Ok(())
  }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// [`StatusList2021Credential`]'s purpose
pub enum StatusPurpose {
  #[default]
  /// Used for revocation
  Revocation,
  /// Used for suspension
  Suspension,
}

impl Display for StatusPurpose {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Revocation => "revocation",
      Self::Suspension => "suspension",
    };
    write!(f, "{s}")
  }
}

impl FromStr for StatusPurpose {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "revocation" => Ok(Self::Revocation),
      "suspension" => Ok(Self::Suspension),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatusList2021CredentialSubject<'a> {
  status_purpose: StatusPurpose,
  encoded_list: Cow<'a, str>,
  id: Option<Cow<'a, Url>>,
}

impl<'c> StatusList2021CredentialSubject<'c> {
  fn try_from_credential(credential: &'c Credential) -> Result<Self, StatusList2021CredentialError> {
    let OneOrMany::One(subject) = &credential.credential_subject else {
      return Err(StatusList2021CredentialError::MultipleCredentialSubject);
    };
    if !subject
      .properties
      .get("type")
      .is_some_and(|value| value.as_str().is_some_and(|t| t == CREDENTIAL_SUBJECT_TYPE))
    {
      return Err(StatusList2021CredentialError::InvalidProperty(
        "credentialSubject.type".to_owned(),
      ));
    };
    let id = subject.id.as_ref().map(Cow::Borrowed);
    let Some(status_purpose) = subject
      .properties
      .get("statusPurpose")
      .and_then(|value| value.as_str())
      .and_then(|purpose| StatusPurpose::from_str(purpose).ok())
    else {
      return Err(StatusList2021CredentialError::InvalidProperty(
        "credentialSubject.statusPurpose".to_owned(),
      ));
    };
    let Some(encoded_list) = subject
      .properties
      .get("encodedList")
      .and_then(|value| value.as_str())
      .map(Cow::Borrowed)
    else {
      return Err(StatusList2021CredentialError::InvalidProperty(
        "credentialSubject.encodedList".to_owned(),
      ));
    };
    Ok(StatusList2021CredentialSubject {
      status_purpose,
      encoded_list,
      id,
    })
  }
}

#[derive(Debug, Default)]
/// Builder type for [`StatusList2021Credential`]
pub struct StatusList2021CredentialBuilder {
  inner_builder: CredentialBuilder,
  id: Option<Url>,
  purpose: StatusPurpose,
  encoded_list: String,
}

impl StatusList2021CredentialBuilder {
  /// Creates a new [`StatusList2021CredentialBuilder`] from a [`StatusList2021`]
  pub fn new(status_list: StatusList2021) -> Self {
    Self {
      encoded_list: status_list.into_encoded_str(),
      ..Default::default()
    }
  }
  /// Sets `credentialSubject.statusPurpose`
  pub const fn purpose(mut self, purpose: StatusPurpose) -> Self {
    self.purpose = purpose;
    self
  }
  /// Sets `credentialSubject.id`
  pub fn subject_id(mut self, id: Url) -> Self {
    self.id = Some(id);
    self
  }
  /// Sets `issuanceDate`
  pub const fn issuance_date(mut self, time: Timestamp) -> Self {
    self.inner_builder.issuance_date = Some(time);
    self
  }
  /// Sets `expirationDate`
  pub const fn expiration_date(mut self, time: Timestamp) -> Self {
    self.inner_builder.expiration_date = Some(time);
    self
  }
  /// Sets `issuer`
  pub fn issuer(mut self, issuer: Issuer) -> Self {
    self.inner_builder.issuer = Some(issuer);
    self
  }
  /// Sets `@context`
  pub fn context(mut self, ctx: Context) -> Self {
    self.inner_builder.context.push(ctx);
    self
  }
  /// Adds a `type` entry
  pub fn add_type(mut self, r#type: String) -> Self {
    self.inner_builder.types.push(r#type);
    self
  }
  /// Adds a credential proof
  pub fn proof(mut self, proof: Proof) -> Self {
    self.inner_builder.proof = Some(proof);
    self
  }
  /// Consumes this [`StatusList2021CredentialBuilder`] into a [`StatusList2021Credential`]
  pub fn build(self) -> Result<StatusList2021Credential, crate::Error> {
    let subject = {
      use crate::credential::Subject;
      use identity_core::common::Value;

      let properties = [
        ("statusPurpose".to_owned(), Value::String(self.purpose.to_string())),
        ("type".to_owned(), Value::String(CREDENTIAL_SUBJECT_TYPE.to_owned())),
        ("encodedList".to_owned(), Value::String(self.encoded_list)),
      ]
      .into_iter()
      .collect();
      if self.id.is_some() {
        Subject::with_id_and_properties(self.id.unwrap(), properties)
      } else {
        Subject::with_properties(properties)
      }
    };
    self
      .inner_builder
      .type_(CREDENTIAL_TYPE)
      .subject(subject)
      .build()
      .map(StatusList2021Credential)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn status_purpose_serialization_works() {
    assert_eq!(
      serde_json::to_string(&StatusPurpose::Revocation).ok(),
      Some(format!("\"{}\"", StatusPurpose::Revocation))
    );
  }
  #[test]
  fn status_purpose_deserialization_works() {
    assert_eq!(
      serde_json::from_str::<StatusPurpose>("\"suspension\"").ok(),
      Some(StatusPurpose::Suspension),
    )
  }
}
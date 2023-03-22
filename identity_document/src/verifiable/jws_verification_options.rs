// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_jose::jws::Decoder;
use identity_verification::MethodScope;

/// Holds additional options for verifying a JWS with
/// [`CoreDocument::verify_jws`](crate::document::CoreDocument::verify_jws()).
#[derive(Default, Debug)]
pub struct JwsVerificationOptions {
  pub(crate) decoder: Decoder,
  pub(crate) nonce: Option<String>,
  pub(crate) method_scope: Option<MethodScope>,
}

impl JwsVerificationOptions {
  /// Append values to the list of permitted extension parameters.
  pub fn critical(self, value: impl Into<String>) -> Self {
    let Self {
      decoder,
      nonce,
      method_scope,
    } = self;
    let decoder = decoder.critical(value);
    Self {
      decoder,
      nonce,
      method_scope,
    }
  }

  /// Set the expected value for the `nonce` parameter of the protected header.
  pub fn nonce(mut self, value: impl Into<String>) -> Self {
    self.nonce = Some(value.into());
    self
  }

  /// Set the scope of the verification methods that may be used to verify the given JWS.
  pub fn method_scope(mut self, value: MethodScope) -> Self {
    self.method_scope = Some(value);
    self
  }
}

// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::core::Url;
use identity_iota::storage::JwsSignatureOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JwsSignatureOptions, inspectable)]
pub struct WasmJwsSignatureOptions(pub(crate) JwsSignatureOptions);

#[wasm_bindgen(js_class = JwsSignatureOptions)]
impl WasmJwsSignatureOptions {
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<IJwsSignatureOptions>) -> Result<WasmJwsSignatureOptions> {
    if let Some(options) = options {
      let options: JwsSignatureOptions = options.into_serde().wasm_result()?;
      Ok(WasmJwsSignatureOptions(options))
    } else {
      Ok(WasmJwsSignatureOptions(Default::default()))
    }
  }

  /// Whether to attach the public key in the corresponding method
  /// to the JWS header.
  #[wasm_bindgen(js_name = setAttachJwk)]
  pub fn set_attach_jwk(&mut self, value: bool) {
    self.0.attach_jwk = value;
  }

  /// Set whether the payload should be Base64url encoded.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7797#section-3)
  #[wasm_bindgen(js_name = setB64)]
  pub fn set_b64(&mut self, value: bool) {
    self.0.b64 = Some(value);
  }

  /// Set the Type value to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
  #[wasm_bindgen(js_name = setTyp)]
  pub fn set_typ(&mut self, value: String) {
    self.0.typ = Some(value);
  }

  /// Set the Content Type to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.10)
  #[wasm_bindgen(js_name = setCty)]
  pub fn set_cty(&mut self, value: String) {
    self.0.cty = Some(value);
  }

  /// Append a value to the list of permitted extension parameters
  /// to be attached to the protected header.
  ///
  ///[More Info](https://tools.ietf.org/html/rfc7515#section-4.1.11)
  #[wasm_bindgen(js_name = addCrit)]
  pub fn add_crit(&mut self, value: String) {
    self.0.crit.get_or_insert(Vec::new()).push(value);
  }

  /// The URL to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.4.1)
  #[wasm_bindgen(js_name = serUrl)]
  pub fn set_url(&mut self, value: String) -> Result<()> {
    self.0.url = Some(Url::parse(value).wasm_result()?);
    Ok(())
  }

  /// Set the nonce to be placed in the protected header.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8555#section-6.5.2)
  #[wasm_bindgen(js_name = setNonce)]
  pub fn set_nonce(&mut self, value: String) {
    self.0.nonce = Some(value);
  }

  /// Whether the JWS signature should have a detached payload.
  #[wasm_bindgen(js_name = setDetachedPayload)]
  pub fn set_detached_payload(&mut self, value: bool) {
    self.0.detached_payload = value;
  }
}

impl_wasm_json!(WasmJwsSignatureOptions, JwsSignatureOptions);
impl_wasm_clone!(WasmJwsSignatureOptions, JwsSignatureOptions);

/// Duck-typed interface to allow creating `JwsSignatureOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwsSignatureOptions")]
  pub type IJwsSignatureOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_JWS_SIGNATURE_OPTIONS: &'static str = r#"
/** Holds options to create `JwsSignatureOptions`. */
interface IJwsSignatureOptions {
    /** Attach the publicKeyJwk value to the protected header
    *
    * Default: false
    */
    readonly attachJwk?: boolean;

    /** 
    *
    * Default: false
    */
    readonly b64?: boolean;

    /**  */
    readonly typ?: string;

    /**  */
    readonly cty?: string;

    /** 
    * 
    */
    readonly crit?: [string];

    /** 
    *
    * 
    */
    readonly url?: string;

    /**
     * 
     */
    readonly nonce?: string;

    /** Whether the JWS signature should have a detached payload.*/
    readonly detachedPayload?: boolean
}"#;

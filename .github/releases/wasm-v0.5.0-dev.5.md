This release introduces a breaking change to the proof field of DID Documents created by versions `v0.5.0-dev.1` through `v0.5.0-dev.4`, making all prior documents incompatible. The main feature of this release is the introduction of WebAssembly (Wasm) bindings for the high-level `Account` API for Javascript/Typescript in both Node.js and the browser. This includes Stronghold storage support but only for Node.js, as it was determined that compiling Stronghold to Wasm for private key storage in the browser would not be sufficiently secure.
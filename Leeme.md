![banner](https://github.com/iotaledger/identity.rs/raw/HEAD/.github/banner_identity.svg)

<p align="center">
  <a href="https://iota.stackexchange.com/" style="text-decoration:none;"><img src="https://img.shields.io/badge/StackExchange-9cf.svg?logo=stackexchange" alt="StackExchange"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/badge/Discord-9cf.svg?logo=discord" alt="Discord"></a>
  <a href="https://discord.iota.org/" style="text-decoration:none;"><img src="https://img.shields.io/discord/397872799483428865" alt="Discord"></a>
  <a href="https://github.com/iotaledger/identity.rs/blob/HEAD/LICENSE" style="text-decoration:none;"><img src="https://img.shields.io/github/license/iotaledger/identity.rs.svg" alt="Apache 2.0 license"></a>
  <img src="https://deps.rs/repo/github/iotaledger/identity.rs/status.svg" alt="Dependencies">
  <a href='https://coveralls.io/github/iotaledger/identity.rs?branch=main'><img src='https://coveralls.io/repos/github/iotaledger/identity.rs/badge.svg?branch=main' alt='Coverage Status' /></a>

</p>

<p align="center">
  <a href="#introduction">Introduction</a> ◈
  <a href="#bindings">Bindings</a> ◈
  <a href="#documentation-and-resources">Documentation & Resources</a> ◈
  <a href="#getting-started">Getting Started</a> ◈
  <a href="#example-creating-an-identity">Example</a> ◈
  <a href="#roadmap-and-milestones">Roadmap</a> ◈
  <a href="#contributing">Contributing</a>
</p>

---

## Introduction

IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized digital identity, also known as Self-Sovereign Identity (SSI). It implements the W3C [Decentralized Identifiers (DID)](https://www.w3.org/TR/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) specifications. This library can be used to create, resolve and authenticate digital identities and to create verifiable credentials and presentations in order to share information in a verifiable manner and establish trust in the digital world. It does so while supporting secure storage of cryptographic keys, which can be implemented for your preferred key management system. Many of the individual libraries (Rust crates) are agnostic over the concrete DID method, with the exception of some libraries dedicated to implement the [IOTA DID method](https://wiki.iota.org/shimmer/identity.rs/specs/did/iota_did_method_spec/), which is an implementation of decentralized digital identity on the IOTA and Shimmer networks. Written in stable Rust, IOTA Identity has strong guarantees of memory safety and process integrity while maintaining exceptional performance.

## Bindings

[Foreign Function Interface (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface) Bindings of this [Rust](https://www.rust-lang.org/) library to other programming languages:

- [Web Assembly](https://github.com/iotaledger/identity.rs/blob/HEAD/bindings/wasm/) (JavaScript/TypeScript)

## Documentation and Resources

- API References:
  - [Rust API Reference](https://docs.rs/identity_iota/latest/identity_iota/): Package documentation (cargo docs).
  - [Wasm API Reference](https://wiki.iota.org/shimmer/identity.rs/libraries/wasm/api_reference/): Wasm Package documentation.
- [Identity Documentation Pages](https://wiki.iota.org/shimmer/identity.rs/introduction): Supplementing documentation with context around identity and simple examples on library usage.
- [Examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples): Practical code snippets to get you started with the library.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.65)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.65)

## Getting Started

If you want to include IOTA Identity in your project, simply add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
identity_iota = { version = "1.0.0" }
```

To try out the [examples](https://github.com/iotaledger/identity.rs/blob/HEAD/examples), you can also do this:

1. Clone the repository, e.g. through `git clone https://github.com/iotaledger/identity.rs`
2. Start a private Tangle as described in the [next section](#example-creating-an-identity)
3. Run the example to create a DID using `cargo run --release --example 0_create_did`

## Example: Creating an Identity

The following code creates and publishes a new IOTA DID Document to a locally running private network.
See the [instructions](https://github.com/iotaledger/hornet/tree/develop/private_tangle) on running your own private network.

_Cargo.toml_

```toml
[package]
name = "iota_identity_example"
version = "1.0.0"
edition = "2021"

[dependencies]
identity_iota = {version = "1.0.0", features = ["memstore"]}
iota-sdk = { version = "1.0.2", default-features = true, features = ["tls", "client", "stronghold"] }
tokio = { version = "1", features = ["full"] }
```



## Roadmap and Milestones

For detailed development progress, see the IOTA Identity development [kanban board](https://github.com/orgs/iotaledger/projects/8/views/5).

## Contributing

We would love to have you help us with the development of IOTA Identity. Each and every contribution is greatly valued!

Please review the [contribution](https://wiki.iota.org/shimmer/identity.rs/contribute) and [workflow](https://wiki.iota.org/shimmer/identity.rs/workflow) sections in the [IOTA Wiki](https://wiki.iota.org/).

To contribute directly to the repository, simply fork the project, push your changes to your fork and create a pull request to get them included!

The best place to get involved in discussions about this library or to look for support at is the `#identity` channel on the [IOTA Discord](https://discord.iota.org). You can also ask questions on our [Stack Exchange](https://iota.stackexchange.com/).

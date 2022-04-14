# OTP generator
> 'otp_generator' is simple one time password generator.

[![Rust](https://github.com/Maheshkumar-Kakade/rust-otp-generator/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Maheshkumar-Kakade/rust-otp-generator/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/otp_generator.svg)](https://crates.io/crates/otp_generator)
[![API](https://docs.rs/otp_generator/badge.svg)](https://docs.rs/otp_generator)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.36+-lightgray.svg)](https://github.com/Maheshkumar-Kakade/rust-otp-generator#rust-version-requirements)

## Index
* [Install](#install)
* [Usage](#usage)
* [License](#license)

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
otp_generator = "0.1.0"
```

## Usage

```rust
let flags = otp_generator::Flags {digits : true, ..Default::default()};
println!("6 digit Otp = {}", otp_generator::generate(6,&flags).unwrap());
```

## License
[MIT][license-url]

[license-image]: http://img.shields.io/badge/license-MIT-blue.svg?style=flat
[license-url]: LICENSE

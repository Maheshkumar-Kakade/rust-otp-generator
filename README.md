# OTP generator
> 'otp_generator' is simple one time password generator.

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

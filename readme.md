# SnowRustler

SnowRustler is a Rust library designed to simplify interfacing with Snowflake, providing Rust developers with a robust
set of tools to interact with Snowflake databases. This library enables executing SQL queries, managing connections, and
handling responses with the efficiency and safety of Rust.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Testing](#testing)
- [Contributing](#contributing)
- [Versioning](#versioning)
- [Authors](#authors)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)
- [Changelog](#changelog)

## Features

- Execute SQL queries: SnowRustler allows you to execute SQL queries directly from your Rust code.
- Manage connections: It provides a simple and efficient way to manage your connections to Snowflake databases.
- Handle responses: It allows you to handle responses from Snowflake databases in a Rust-friendly way.

## Requirements

- Rust 1.54.0 or later
- Cargo 1.54.0 or later
- Access to a Snowflake database

## Getting Started

1. Install Rust and Cargo.
2. Clone the SnowRustler repository.
3. Run `cargo build` to build the project.
4. Run `cargo test` to run the tests.

## Quick Start

```rust
use snowrustler::SnowRustler;

let sr = SnowRustler::new("your_snowflake_url", "your_username", "your_password");
let result = sr.execute_query("SELECT * FROM your_table");
println!("{:?}", result);
``` 

## API Reference

## Testing

...

## Contributing

...

## Versioning

...

## Authors

Paul Wade - Initial work - [GitHub](https://github.com/paul-wade)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENCE) file for details

## Contact

Project Link: [https://github.com/paul-wade/snow_rustler](https://github.com/paul-wade/snow_rustler)

## Acknowledgments

This project wouldn't be possible without these wonderful resources:

- [reqwest](https://github.com/seanmonstar/reqwest) - For making HTTP requests in Rust.
- [tokio](https://github.com/tokio-rs/tokio) - A runtime for writing reliable, asynchronous, and slim applications with
  the Rust programming language.
- [serde](https://github.com/serde-rs/serde) - A framework for serializing and deserializing Rust data structures
  efficiently and generically.
- [mockall](https://github.com/asomers/mockall) - A powerful, flexible mocking library for Rust.
- [serde_json](https://github.com/serde-rs/json) - A JSON library for Rust, built with serde.
- [httpmock](https://github.com/alexliesenfeld/httpmock) - A library for mocking HTTP servers in Rust.
 
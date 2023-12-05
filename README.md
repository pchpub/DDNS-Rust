# ddns-rust

This is the README file for the `DDNS-Rust` project.

## Introduction

The `DDNS-Rust` project is a Rust-based dynamic DNS (DDNS) client. It allows users to update their DNS records dynamically, enabling them to host services on their own machines with changing IP addresses.

## Installation

To install `DDNS-Rust`, follow these steps:

1. Clone the repository: `git clone https://github.com/pchpub/DDNS-Rust.git`
2. Change into the project directory: `cd DDNS-Rust`
3. Build the project: `cargo build --profile=fast`
4. Run the project: `cargo run --profile=fast`

## Usage

To use `DDNS-Rust`, you need to provide the necessary configuration. This includes the DNS provider, domain name, and authentication credentials. Once the configuration is set up, you can run the client to update your DNS records.

## Contributing

Contributions are welcome! If you would like to contribute to `DDNS-Rust`, please follow the guidelines in the [CONTRIBUTING.md](./CONTRIBUTING.md) file.

## License

This project is licensed under the [GNU General Public License v3.0](./LICENSE).

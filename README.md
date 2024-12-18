# pass-vault

`pass-vault` is a simple password manager written in Rust. It allows you to add, list, and search for password entries stored in a JSON file.

## Features

- Add new password entries
- List all password entries
- Search for a specific password entry by service name

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/pass-vault.git
   cd pass-vault
   ```

2. Build the project using Cargo:
   ```sh
   cargo build
   ```

## Usage

Run the application:

```sh
cargo run
```

## File Structure

- src/main.rs: Contains the main function and the menu logic.
- src/passentry.rs: Contains the ServiceInfo struct and related functions for handling password entries.
- password.json: The JSON file where password entries are stored.

## Dependencies

- serde: Serialization and deserialization library for Rust.
- serde_json: JSON support for serde.

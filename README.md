# Zola

Zola is a CLI tool that fetches and displays version information of npm packages from either the public npm registry or a private Artifactory repository.

## Features

- Fetches version details of an npm package
- Supports both public npm registry and private Artifactory repository
- Prints the latest, next, beta, and rc versions

## Installation

Please ensure that you have Rust and cargo installed in your system. You can download it from the [official website](https://www.rust-lang.org/tools/install).

Clone this repository:

```sh
git clone https://github.com/orieken/zola.git
```

Then build the project:
```shell
cd zola
cargo build --release
```
This will create an executable in the ./target/release directory.

## Usage
You can run Zola with the package name you want to fetch version details for:
```shell
./target/release/zola -p [package-name]
```
## Configuration
For private Artifactory repositories, you'll need a settings.toml file in your project directory with the following format:

```shell

url = "artifactory-url"
token = "artifactory-auth-token"
```
Replace artifactory-url and artifactory-auth-token with your actual Artifactory details.

## Tests
Zola has both unit tests and integration tests. You can run the tests with:

```sh
cargo test
```

## Contributing
Contributions are welcome! Please submit a pull request or create an issue to get started.

## License
This project is licensed under the MIT License. See LICENSE for details.

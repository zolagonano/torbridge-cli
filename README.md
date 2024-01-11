# torbridge-cli

![b](https://img.shields.io/crates/l/torbridge-cli)
![b](https://img.shields.io/crates/d/torbridge-cli)
![b](https://img.shields.io/crates/v/torbridge-cli)

`torbridge-cli` is a command-line interface (CLI) tool written in [Rust](https://rustlang.org) that allows you to fetch Tor bridges from the BridgeDB and display them in the terminal. It simplifies the process of obtaining Tor bridges for your Tor client configuration.

## Installation

Make sure you have Rust and Cargo installed on your system. Then, follow these steps to build and install `torbridge-cli`:

```bash
cargo install torbridge-cli
```

Or you can clone this Repository and build from there:

1. Clone the repository:

```bash
git clone https://github.com/zolagonano/torbridge-cli.git
```

2. Change into the project directory:

```bash
cd torbridge-cli
```

3. Build the project:

```bash
cargo build --release
```

4. Optionally, copy the executable to a directory in your system's PATH:

```bash
cp target/release/torbridge-cli /usr/local/bin/
```

## Usage

To use `torbridge-cli`, simply run the executable:

```bash
torbridge-cli
```

The tool will guide you through the process of obtaining Tor bridges. It will display the available transport modes, prompt you to choose a mode, show a captcha image for verification, and request your answer. Finally, it will display the retrieved Tor bridges in the terminal.

## Contributing

Contributions are welcome! If you find a bug, have a feature request, or want to contribute code, please open an issue or submit a pull request on the GitHub repository.

When contributing, please follow the existing code style and ensure your changes are well-tested.

## Donation

If you find this project helpful and would like to show your appreciation, consider making a donation. Your contributions helps me dedicate more time working on Free and open-source projects :heart:

[https://zolagonano.github.io/support](https://zolagonano.github.io/support)

## License

This project is licensed under the MIT License.

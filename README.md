# skript

skript is a natural language command prompt designed for the native Windows Terminal, enabling users to execute complex commands using plain English.

## Features
- Translate natural language instructions into Windows commands
- Seamless integration with Windows Terminal
- Secure and privacy-conscious: no data leaves your machine
- Extensible for custom workflows

## Getting Started

### Prerequisites
- Windows 10 or later
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- Windows Terminal

### Installation
```sh
# Clone the repository
git clone https://github.com/pojesh/skript.git
cd skript
# Build the project
cargo build --release
```

### Running skript
```sh
cargo run --release
```

## Usage
Type your command in natural language, for example:

```
List all files modified in the last 24 hours
```

skript will translate and execute the appropriate Windows command.

## Contributing
Contributions are welcome! Please open issues or submit pull requests.

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

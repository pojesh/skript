# skript

skript is a natural language command prompt designed for the native Windows Terminal, enabling users to execute complex commands using plain English.

## Features
- Translate natural language instructions into Windows PowerShell commands
- Seamless integration with Windows Terminal
- Secure and privacy-conscious: no data leaves your machine
- Extensible for custom workflows

## Getting Started

### Prerequisites
- Windows 10 or later
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- Windows Terminal
- A Gemini API key from Google AI Studio

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
cargo run 
```
cargo wix --nocapture

## Installation and Usage

### 1. Download and Install
- Go to the [GitHub Releases](https://github.com/pojesh/skript/releases) page.
- Download the latest `.msi` installer package for skript.
- Double-click the `.msi` file and follow the prompts to complete installation.

### 2. Launch skript
- Open any terminal (Windows Terminal, Command Prompt, or PowerShell).
- Type `skript` and press Enter to start skript terminal mode.

### 3. First Launch: Gemini API Key Setup
- On first launch, skript will prompt you for a Gemini API key.
- To create a Gemini API key, visit [Google AI Studio](https://aistudio.google.com/apikey).
- Copy your API key and paste it into skript when prompted.
- The key will be securely saved to Windows Credential Manager for future use.

### 4. Using skript
- Enter commands in plain English, for example:
  ```
  skript> check cuda in pytorch
  ```
- skript will translate your request and execute the appropriate command.

```
Executing: python -c "import torch; print(torch.cuda.is_available())"
```

## Removing the Gemini API Key from Windows Credential Manager
If you wish to delete the Gemini API key stored by skript, execute this:

```bash
cmdkey /delete:gemini.skript
```

The next time you run skript, you will be prompted to enter your Gemini API key again.

The Gemini API will automatically be deleted from your Windows Credential Manager when Skript program is uninsatalled.

## Contributing
Contributions are welcome! Please open issues or submit pull requests.

## License
This project is licensed under the GNU License. See [LICENSE](LICENSE) for details.

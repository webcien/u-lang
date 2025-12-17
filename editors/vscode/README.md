# U Language for Visual Studio Code

Official Visual Studio Code extension for U Language.

## Features

- **Syntax Highlighting** - Full syntax highlighting for U Language
- **Language Server** - IntelliSense, code completion, and diagnostics
- **Go to Definition** - Navigate to function and type definitions
- **Hover Information** - View type information on hover
- **Auto-formatting** - Format code automatically

## Installation

### From VSIX

1. Download the latest `.vsix` file from the [releases page](https://github.com/webcien/u-lang/releases)
2. Open VS Code
3. Go to Extensions (Ctrl+Shift+X)
4. Click the "..." menu and select "Install from VSIX..."
5. Select the downloaded `.vsix` file

### From Source

```bash
cd editors/vscode
npm install
npm run compile
npm run package
code --install-extension u-language-1.4.0.vsix
```

## Requirements

- U Language compiler (`ul`) installed and in PATH
- U Language Server (`u-lsp`) installed and in PATH

## Configuration

You can configure the extension in VS Code settings:

```json
{
  "u.lsp.path": "/path/to/u-lsp"
}
```

## Usage

1. Create a new file with `.ul` extension
2. Start typing U code
3. Enjoy syntax highlighting and IntelliSense!

## Example

```u
fn main() {
    let message = "Hello, U Language!";
    unsafe {
        printf("%s\n", message);
    }
    return 0;
}

extern "C" {
    fn printf(format: ptr, ...);
}
```

## License

MIT License — Copyright (c) 2025 Webcien and U contributors

# U Language - Native Compiler for Windows

**Date:** December 17, 2025  
**Version:** 1.6.0

---

## 🚀 Overview

U Language v1.6.0 introduces a **native compiler for Windows**, allowing you to build and run U programs directly on Windows without WSL.

This version also includes **full GUI support** with Skia, enabling the creation of native Windows applications.

---

## 1. Installation

### Prerequisites

- **Rust:** https://rustup.rs/
- **Zig:** https://ziglang.org/download/
- **Git:** https://git-scm.com/download/win

### Installation Script (Recommended)

```powershell
# Download and run the installer
Invoke-WebRequest -Uri https://raw.githubusercontent.com/webcien/u-lang/master/install_windows.ps1 -OutFile install.ps1

# Install without Skia (for CLI apps)
.\install.ps1

# Or install with Skia (for GUI apps)
.\install.ps1 -WithSkia
```

### Manual Installation

1. Clone the repository:
   ```powershell
   git clone https://github.com/webcien/u-lang.git
   cd u-lang
   ```

2. Build the compiler:
   ```powershell
   .\build.ps1
   ```

3. Add to PATH:
   ```powershell
   $env:Path += ";$PWD\compiler\target\release"
   ```

---

## 2. Compiling Programs

### CLI Applications

```powershell
# Compile
ul build my_app.ul

# Run
.\my_app.exe
```

### GUI Applications

1. **Install Skia:**
   ```powershell
   .\scripts\download_skia_windows.ps1
   ```

2. **Compile to C:**
   ```powershell
   ul build my_gui_app.ul --no-link
   ```

3. **Link with Skia:**
   ```powershell
   .\scripts\link_gui_windows.ps1 my_gui_app
   ```

4. **Run:**
   ```powershell
   .\my_gui_app.exe
   ```

---

## 3. GUI Development

### Skia Integration

- **Pre-compiled binaries:** Skia is downloaded and configured automatically.
- **Linking:** The `link_gui_windows.ps1` script handles linking with Skia.
- **Runtime:** `runtime/skia_windows.c` provides the glue code.

### Example: Hello GUI

```u
// examples/windows/hello_gui.ul
ui my_window {
    Container {
        width: 400,
        height: 300,
        child: Text { text: "Hello, Windows!" }
    }
}

fn main() {
    unsafe {
        skia_init();
        let surface = skia_create_surface(400, 300);
        let canvas = skia_get_canvas(surface);
        render_ui_my_window(canvas);
        skia_save_png(surface, "hello_gui.png");
    }
    return 0;
}
```

### Compiling the Example

```powershell
cd examples\windows
ul build hello_gui.ul --no-link
..\..\scripts\link_gui_windows.ps1 hello_gui
.\hello_gui.exe
```

This will create `hello_gui.png` with the rendered UI.

---

## 4. Scripts and Tools

### `build.ps1`
- Compiles the U compiler and LSP server.

### `install_windows.ps1`
- Installs U Language and its dependencies.
- Adds U to the PATH.
- Optionally installs Skia.

### `download_skia_windows.ps1`
- Downloads and configures pre-compiled Skia binaries.

### `link_gui_windows.ps1`
- Links U GUI applications with Skia.

---

## 5. Troubleshooting

### Error: "cl: command not found"
Install Visual Studio with C++ development tools.

### Error: "zig: command not found"
Install Zig and add it to your PATH.

### Skia Linking Errors
- Ensure `SKIA_DIR` is set correctly.
- Use the `download_skia_windows.ps1` script for automatic setup.

---

## 6. Roadmap

- **v1.7:** Interactive GUI (event loop integration)
- **v1.8:** Standalone executables (static linking)
- **v2.0:** LLVM backend for better performance

---

**Happy coding with U Language on Windows! 🚀**

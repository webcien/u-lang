# Installation Guide

Complete instructions for installing and setting up U on your system.

---

## System Requirements

### Minimum Requirements

- **OS**: Linux, macOS, or Windows (with WSL)
- **RAM**: 2 GB
- **Disk**: 500 MB free space
- **Internet**: For downloading dependencies

### Required Software

| Software | Version | Purpose |
|----------|---------|---------|
| **Rust** | 1.92.0+ | Compile the compiler |
| **Zig** | 0.13.0+ | Link binaries |
| **Git** | 2.30+ | Clone repository |
| **GCC/Clang** | Any recent | Compile C code |

---

## Installation Steps

### Step 1: Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version  # Should be 1.92.0+
cargo --version  # Should be 1.92.0+
```

### Step 2: Install Zig

Download from [ziglang.org](https://ziglang.org/download/):

**Linux:**
```bash
# Download (replace VERSION with latest)
wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz

# Extract
tar xf zig-linux-x86_64-0.13.0.tar.xz
sudo mv zig-0.13.0 /opt/zig

# Add to PATH
export PATH="/opt/zig:$PATH"
```

**macOS:**
```bash
# Using Homebrew
brew install zig

# Or download manually from ziglang.org
```

**Windows:**
```bash
# Download from https://ziglang.org/download/
# Extract to C:\zig
# Add C:\zig to PATH
```

Verify installation:

```bash
zig version  # Should be 0.13.0+
```

### Step 3: Clone the Repository

```bash
git clone https://github.com/webcien/u.git
cd u
```

### Step 4: Build the Compiler

```bash
cd compiler
cargo build --release
```

This will take 10-15 minutes on first build.

Verify build:

```bash
./target/release/ul --version
```

### Step 5: Add to PATH (Optional)

**Linux/macOS:**
```bash
# Temporary (current session only)
export PATH="$PATH:$(pwd)/target/release"

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="$PATH:'"$(pwd)/target/release"'"' >> ~/.bashrc
source ~/.bashrc
```

**Windows (PowerShell):**
```powershell
$env:PATH += ";$(pwd)\target\release"
```

---

## Verification

Test the installation:

```bash
# Create a test file
cat > hello.ul << 'EOF'
fn main() {
    print("Hello from U!");
}
EOF

# Compile
./compiler/target/release/ul build hello.ul

# Run
./hello
```

Expected output:
```
Hello from U!
```

---

## Troubleshooting

### "command not found: rustc"

**Solution**: Rust is not installed or not in PATH.

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Activate Rust
source $HOME/.cargo/env
```

### "command not found: zig"

**Solution**: Zig is not installed or not in PATH.

```bash
# Download and extract Zig
# Add to PATH
export PATH="/path/to/zig:$PATH"
```

### "error: linker `zig` not found"

**Solution**: Zig is not in PATH.

```bash
# Find zig
which zig

# Add to PATH
export PATH="/path/to/zig:$PATH"
```

### "error: could not compile `u`"

**Solution**: Check Rust version and dependencies.

```bash
# Update Rust
rustup update

# Clean and rebuild
cd compiler
cargo clean
cargo build --release
```

### "error: failed to run custom build command"

**Solution**: Missing system dependencies.

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y build-essential
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf groupinstall -y "Development Tools"
```

**macOS:**
```bash
xcode-select --install
```

---

## Updating U

To update to the latest version:

```bash
cd u
git pull origin master
cd compiler
cargo build --release
```

---

## Uninstalling U

To remove U:

```bash
# Remove repository
rm -rf ~/u

# Remove from PATH (if added)
# Edit ~/.bashrc or ~/.zshrc and remove the PATH line
```

---

## Development Setup

If you want to contribute to U:

1. **Fork** the repository on GitHub
2. **Clone** your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/u.git
   cd u
   ```
3. **Create** a feature branch:
   ```bash
   git checkout -b feature/your-feature
   ```
4. **Build** and test:
   ```bash
   cd compiler
   cargo build --release
   cargo test --release
   ```
5. **Format** and lint:
   ```bash
   cargo fmt
   cargo clippy --release
   ```

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

---

## Next Steps

After installation:

1. **Read** [README.md](README.md) for overview
2. **Check** [SPEC.md](docs/SPEC.md) for language syntax
3. **Try** examples in `examples/` directory
4. **Build** your first program
5. **Join** the community on GitHub

---

## Getting Help

- **Documentation**: [docs/SPEC.md](docs/SPEC.md)
- **FAQ**: [FAQ.md](FAQ.md)
- **Issues**: [GitHub Issues](https://github.com/webcien/u/issues)
- **Discussions**: [GitHub Discussions](https://github.com/webcien/u/discussions)

---

**U: Making systems programming safe, simple, and fun.**

*Installation Guide — December 16, 2025*

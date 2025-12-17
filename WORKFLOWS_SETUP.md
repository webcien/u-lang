# GitHub Workflows Setup Instructions

**Status**: Workflows need to be added manually via GitHub web UI  
**Reason**: GitHub token lacks `workflow` scope  
**Date**: December 16, 2025

---

## Files to Add

The following workflow files are backed up in `/tmp/workflows_backup/`:

1. **test.yml** — CI/CD pipeline for automated tests
2. **release.yml** — Automated release workflow

---

## Manual Setup Instructions

### Step 1: Navigate to Repository

Go to: https://github.com/webcien/u-lang

### Step 2: Create Workflows Directory

1. Click on "Add file" → "Create new file"
2. Type: `.github/workflows/test.yml`
3. GitHub will automatically create the directory structure

### Step 3: Add test.yml

Copy the content from `/tmp/workflows_backup/test.yml`:

```yaml
name: Tests

on:
  push:
    branches: [ master, develop, feature/* ]
  pull_request:
    branches: [ master, develop ]

jobs:
  test:
    name: Run Tests
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.92.0
        override: true
        components: rustfmt, clippy
    
    - name: Install Zig
      run: |
        wget -q https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
        tar xf zig-linux-x86_64-0.13.0.tar.xz
        sudo mv zig-linux-x86_64-0.13.0 /opt/zig
        echo "/opt/zig" >> $GITHUB_PATH
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: compiler/target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Build compiler
      working-directory: compiler
      run: cargo build --release
    
    - name: Run tests
      working-directory: compiler
      run: cargo test --release
    
    - name: Run clippy
      working-directory: compiler
      run: cargo clippy --release -- -D warnings
    
    - name: Check formatting
      working-directory: compiler
      run: cargo fmt -- --check
    
    - name: Compile examples
      run: |
        for file in examples/*.ul; do
          echo "Compiling $file..."
          ./compiler/target/release/ul build "$file" || exit 1
        done
```

### Step 4: Add release.yml

1. Click on "Add file" → "Create new file"
2. Type: `.github/workflows/release.yml`
3. Copy the content from `/tmp/workflows_backup/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    name: Create Release
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.92.0
        override: true
    
    - name: Install Zig
      run: |
        wget -q https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
        tar xf zig-linux-x86_64-0.13.0.tar.xz
        sudo mv zig-linux-x86_64-0.13.0 /opt/zig
        echo "/opt/zig" >> $GITHUB_PATH
    
    - name: Build compiler
      working-directory: compiler
      run: cargo build --release
    
    - name: Package release
      run: |
        mkdir -p release
        cp compiler/target/release/ul release/
        cp README.md LICENSE.txt INSTALLATION.md release/
        tar czf u-lang-${{ github.ref_name }}-linux-x86_64.tar.gz release/
    
    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: u-lang-${{ github.ref_name }}-linux-x86_64.tar.gz
        body: |
          Release ${{ github.ref_name }}
          
          See CHANGELOG for details.
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Step 5: Commit Workflows

1. Add commit message: "ci: add GitHub Actions workflows"
2. Click "Commit new file"
3. Repeat for the second workflow file

---

## Verification

After adding the workflows:

1. Go to "Actions" tab
2. Verify workflows appear
3. Trigger a test run by making a small commit
4. Check that tests pass

---

## Alternative: Use Token with `workflow` Scope

If you have a token with `workflow` scope:

```bash
cd /home/ubuntu/u-lang
cp -r /tmp/workflows_backup/* .github/workflows/
git add .github/workflows/
git commit -m "ci: add GitHub Actions workflows"
git push origin master
```

---

## Backup Location

Workflow files are backed up at:
- `/tmp/workflows_backup/test.yml`
- `/tmp/workflows_backup/release.yml`

---

**Status**: ⚠️ **MANUAL ACTION REQUIRED**

Workflows need to be added via GitHub web UI due to token scope limitations.

*Workflows Setup Instructions — December 16, 2025*

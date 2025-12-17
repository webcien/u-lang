# ul.toml Specification

**Version:** 1.0  
**Date:** December 17, 2025  
**Authors:** Manus AI, Webcien

---

## Overview

`ul.toml` is the manifest file format for U Language packages. It defines package metadata, dependencies, and build configuration in a TOML format similar to Rust's `Cargo.toml`.

---

## File Structure

```toml
[package]
name = "my-package"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
edition = "2025"
description = "A brief description of the package"
license = "MIT"
repository = "https://github.com/username/my-package"
homepage = "https://my-package.com"
documentation = "https://docs.my-package.com"
keywords = ["keyword1", "keyword2", "keyword3"]
categories = ["category1", "category2"]

[dependencies]
some-package = "1.0.0"
another-package = { version = "2.0.0", features = ["feature1"] }
local-package = { path = "../local-package" }
git-package = { git = "https://github.com/user/repo", tag = "v1.0.0" }

[dev-dependencies]
test-framework = "1.0.0"

[build-dependencies]
build-tool = "1.0.0"

[features]
default = ["feature1"]
feature1 = []
feature2 = ["dependency/feature"]

[profile.release]
opt-level = 3
debug = false

[profile.dev]
opt-level = 0
debug = true

[lib]
name = "my_lib"
path = "src/lib.ul"

[[bin]]
name = "my-app"
path = "src/main.ul"

[target.'x86_64-linux'.dependencies]
linux-only-dep = "1.0.0"

[target.'wasm32-wasi'.dependencies]
wasm-only-dep = "1.0.0"
```

---

## Sections

### `[package]`

Defines package metadata.

| Field | Type | Required | Description |
|:---|:---|:---:|:---|
| `name` | String | ✅ | Package name (lowercase, hyphens allowed) |
| `version` | String | ✅ | Semantic version (e.g., "1.0.0") |
| `authors` | Array<String> | ❌ | List of authors |
| `edition` | String | ❌ | U Language edition (e.g., "2025") |
| `description` | String | ❌ | Brief description |
| `license` | String | ❌ | SPDX license identifier |
| `repository` | String | ❌ | Source repository URL |
| `homepage` | String | ❌ | Homepage URL |
| `documentation` | String | ❌ | Documentation URL |
| `keywords` | Array<String> | ❌ | Search keywords (max 5) |
| `categories` | Array<String> | ❌ | Package categories |

### `[dependencies]`

Defines runtime dependencies.

**Simple version:**
```toml
package-name = "1.0.0"
```

**Detailed version:**
```toml
package-name = { version = "1.0.0", features = ["feature1"], optional = false }
```

**Path dependency:**
```toml
local-package = { path = "../local-package" }
```

**Git dependency:**
```toml
git-package = { git = "https://github.com/user/repo", tag = "v1.0.0" }
git-package = { git = "https://github.com/user/repo", branch = "main" }
git-package = { git = "https://github.com/user/repo", rev = "abc123" }
```

### `[dev-dependencies]`

Dependencies only needed for development and testing.

### `[build-dependencies]`

Dependencies needed for build scripts.

### `[features]`

Defines optional features.

```toml
[features]
default = ["feature1"]  # Enabled by default
feature1 = []           # No dependencies
feature2 = ["dep/feature"]  # Enables feature in dependency
```

### `[profile.release]` and `[profile.dev]`

Build profiles for optimization.

| Field | Type | Default (release) | Default (dev) | Description |
|:---|:---|:---:|:---:|:---|
| `opt-level` | Integer (0-3) | 3 | 0 | Optimization level |
| `debug` | Boolean | false | true | Include debug symbols |

### `[lib]`

Defines a library target.

| Field | Type | Required | Description |
|:---|:---|:---:|:---|
| `name` | String | ❌ | Library name (defaults to package name) |
| `path` | String | ❌ | Path to library root (defaults to "src/lib.ul") |

### `[[bin]]`

Defines a binary target. Can have multiple `[[bin]]` sections.

| Field | Type | Required | Description |
|:---|:---|:---:|:---|
| `name` | String | ✅ | Binary name |
| `path` | String | ❌ | Path to binary root (defaults to "src/main.ul") |

### `[target.'<triple>'.dependencies]`

Platform-specific dependencies.

```toml
[target.'x86_64-linux'.dependencies]
linux-only = "1.0.0"

[target.'wasm32-wasi'.dependencies]
wasm-only = "1.0.0"
```

---

## Version Requirements

U Language uses semantic versioning with the following operators:

| Operator | Example | Meaning |
|:---|:---|:---|
| `=` | `= 1.0.0` | Exact version |
| `^` | `^1.0.0` | Compatible with 1.0.0 (>=1.0.0, <2.0.0) |
| `~` | `~1.0.0` | Approximately 1.0.0 (>=1.0.0, <1.1.0) |
| `>` | `> 1.0.0` | Greater than 1.0.0 |
| `>=` | `>= 1.0.0` | Greater than or equal to 1.0.0 |
| `<` | `< 2.0.0` | Less than 2.0.0 |
| `<=` | `<= 2.0.0` | Less than or equal to 2.0.0 |
| `*` | `1.*` | Any version starting with 1 |

**Default:** If no operator is specified, `^` is assumed.

---

## Example: Complete `ul.toml`

```toml
[package]
name = "awesome-app"
version = "1.0.0"
authors = ["Alice <alice@example.com>", "Bob <bob@example.com>"]
edition = "2025"
description = "An awesome U Language application"
license = "MIT"
repository = "https://github.com/alice/awesome-app"
homepage = "https://awesome-app.com"
keywords = ["app", "awesome", "gui"]
categories = ["applications", "gui"]

[dependencies]
u-std = "1.0.0"
u-gui = { version = "1.2.0", features = ["skia", "sdl2"] }
u-actor = "1.1.0"

[dev-dependencies]
u-test = "1.0.0"

[features]
default = ["gui"]
gui = ["u-gui"]
cli = []

[profile.release]
opt-level = 3
debug = false

[profile.dev]
opt-level = 0
debug = true

[[bin]]
name = "awesome-app"
path = "src/main.ul"

[lib]
name = "awesome_lib"
path = "src/lib.ul"
```

---

## Commands

### `ul init`

Create a new package with a default `ul.toml`.

```bash
ul init my-package
```

### `ul build`

Build the package using `ul.toml` configuration.

```bash
ul build
ul build --release
```

### `ul install`

Install a package from a registry or Git.

```bash
ul install some-package
ul install some-package@1.0.0
ul install git+https://github.com/user/repo
```

### `ul publish`

Publish the package to the U Language registry.

```bash
ul publish
```

### `ul update`

Update dependencies to the latest compatible versions.

```bash
ul update
```

---

## Registry

The default registry is **https://packages.u-lang.dev** (to be implemented).

Packages are organized by name and version:
```
https://packages.u-lang.dev/api/v1/packages/{name}/{version}
```

---

## License

This specification is licensed under the MIT License.

**Copyright © 2025 Webcien and U contributors**

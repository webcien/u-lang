# Contributing to U

Thank you for your interest in contributing to the U programming language! This document provides guidelines and instructions for contributing to the project.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Making Changes](#making-changes)
5. [Commit Guidelines](#commit-guidelines)
6. [Testing](#testing)
7. [Pull Request Process](#pull-request-process)
8. [Reporting Bugs](#reporting-bugs)
9. [Suggesting Features](#suggesting-features)
10. [Questions?](#questions)

---

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

---

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.92.0 or later**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo 1.92.0 or later** (comes with Rust)
  ```bash
  cargo --version
  ```

- **Zig 0.13.0 or later** (for linking)
  ```bash
  # Download from https://ziglang.org/download/
  ```

- **Git 2.30 or later**
  ```bash
  git --version
  ```

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/u.git
   cd u
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/webcien/u.git
   ```

---

## Development Setup

### Build the Compiler

```bash
cd compiler
cargo build --release
```

The compiled binary will be at `compiler/target/release/ul`.

### Run Tests

```bash
cd compiler
cargo test --release
```

Expected output: `test result: ok. 27+ passed; 0 failed`

### Run Examples

```bash
./compiler/target/release/ul build examples/hello.ul
./hello
```

### Format Code

```bash
cd compiler
cargo fmt
```

### Lint Code

```bash
cd compiler
cargo clippy --release
```

---

## Making Changes

### Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/` — New features
- `fix/` — Bug fixes
- `docs/` — Documentation
- `test/` — Tests
- `refactor/` — Code refactoring
- `perf/` — Performance improvements

### Keep Your Branch Updated

```bash
git fetch upstream
git rebase upstream/master
```

### Make Your Changes

1. Make focused, logical changes
2. Follow the code style (see below)
3. Add tests for new functionality
4. Update documentation as needed

### Code Style

- **Rust**: Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- **Naming**: Use descriptive names for functions and variables
- **Comments**: Write clear comments in English
- **Documentation**: Add doc comments for public APIs
- **Formatting**: Run `cargo fmt` before committing

Example:

```rust
/// Parse a generic trait definition.
/// 
/// # Arguments
/// 
/// * `input` - The input string to parse
/// 
/// # Returns
/// 
/// A parsed `GenericTrait` or an error
fn parse_generic_trait(input: &str) -> Result<GenericTrait, ParseError> {
    // Implementation
}
```

---

## Commit Guidelines

### Commit Message Format

Follow the conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **ci**: Changes to CI configuration files and scripts
- **chore**: Changes to build process, dependencies, etc.

### Scope

Optional scope to specify what part of the code is affected:
- `parser` — Parser module
- `type-checker` — Type checker module
- `codegen` — Code generation module
- `lexer` — Lexer module
- `cli` — Command-line interface
- `stdlib` — Standard library
- `examples` — Examples
- `docs` — Documentation

### Subject

- Use imperative mood ("add feature" not "added feature")
- Don't capitalize first letter
- No period (.) at the end
- Limit to 50 characters

### Body

- Wrap at 72 characters
- Explain what and why, not how
- Separate from subject with a blank line

### Footer

Optional footer for breaking changes or issue references:

```
Fixes #123
BREAKING CHANGE: description of breaking change
```

### Examples

```
feat(parser): add support for generic traits

Add parsing support for generic trait definitions with type parameters,
bounds, and where clauses. This enables traits like:

  trait Iterator<T> {
      fn next() -> Option<T>;
  }

Fixes #45
```

```
fix(type-checker): resolve type arguments in trait implementations

Previously, type arguments in trait implementations were not being
resolved correctly, causing type checking errors. Now we properly
resolve T to concrete types in impl blocks.

Fixes #67
```

```
docs: update README with installation instructions

Add clear step-by-step instructions for installing U on different
platforms (Linux, macOS, Windows).
```

---

## Testing

### Unit Tests

Add unit tests for new functionality:

```rust
#[test]
fn test_parse_generic_trait_basic() {
    let input = "trait Iterator<T> { fn next() -> Option<T>; }";
    let result = parse_generic_trait(input);
    assert!(result.is_ok());
}
```

### Integration Tests

For larger features, add integration tests:

```bash
# Create test file
touch examples/my_feature.ul

# Compile and run
./compiler/target/release/ul build examples/my_feature.ul
./my_feature
```

### Running Tests

```bash
# Run all tests
cargo test --release

# Run specific test
cargo test test_parse_generic_trait_basic --release

# Run with output
cargo test --release -- --nocapture
```

### Test Coverage

Aim for at least 80% code coverage:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --release --out Html
```

---

## Pull Request Process

### Before Submitting

1. **Update your branch**
   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

2. **Run tests**
   ```bash
   cargo test --release
   ```

3. **Format code**
   ```bash
   cargo fmt
   ```

4. **Lint code**
   ```bash
   cargo clippy --release
   ```

5. **Check for regressions**
   - Ensure all existing tests still pass
   - Ensure no new compiler warnings

### Submitting a PR

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Create a Pull Request on GitHub with:
   - Clear title describing the change
   - Description of what changed and why
   - Reference to related issues (e.g., "Fixes #123")
   - Screenshots/examples if applicable

3. Fill out the PR template completely

### PR Template

```markdown
## Description
Brief description of the changes made.

## Type of Change
- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation update

## Related Issues
Fixes #(issue number)

## Testing
- [ ] I have tested these changes locally
- [ ] I have added tests for new functionality
- [ ] All tests pass (`cargo test --release`)
- [ ] No new compiler warnings

## Checklist
- [ ] Code follows the style guidelines
- [ ] Documentation has been updated
- [ ] Commit messages follow conventions
- [ ] No breaking changes (or documented)
- [ ] Changes are focused and logical
```

### Review Process

- At least one maintainer review required
- All CI checks must pass
- No merge conflicts
- Constructive feedback will be provided
- Be prepared to make changes based on feedback

---

## Reporting Bugs

### Before Reporting

- Check existing issues to avoid duplicates
- Test with the latest version
- Gather relevant information

### Bug Report Template

```markdown
## Description
Brief description of the bug.

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: (e.g., Ubuntu 22.04)
- Rust version: (output of `rustc --version`)
- U version: (output of `ul --version`)

## Additional Context
Any other relevant information.
```

---

## Suggesting Features

### Before Suggesting

- Check existing issues and roadmap
- Ensure the feature aligns with U's philosophy
- Consider the implementation complexity

### Feature Request Template

```markdown
## Description
Clear description of the proposed feature.

## Motivation
Why this feature is needed and what problem it solves.

## Proposed Solution
How the feature should work.

## Example Usage
```ul
// Example code showing how the feature would be used
```
```

## Alternatives
Other ways to solve the problem.

## Additional Context
Any other relevant information.
```

---

## Questions?

- **Documentation**: Check [README.md](README.md) and [docs/SPEC.md](docs/SPEC.md)
- **Issues**: Search existing issues on GitHub
- **Discussions**: Start a discussion on GitHub
- **Email**: Contact the maintainers

---

## Recognition

Contributors will be recognized in:
- [ACKNOWLEDGEMENTS.md](ACKNOWLEDGEMENTS.md)
- GitHub contributors page
- Release notes

---

## License

By contributing to U, you agree that your contributions will be licensed under the [MIT License](LICENSE.txt).

---

Thank you for contributing to U! 🎉

*U: Making systems programming safe, simple, and fun.*

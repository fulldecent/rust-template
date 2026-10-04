# Rust project template

[![Build and test](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml/badge.svg?branch=main)](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml)
[![Lint](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml)

Start a Rust command-line project with a working program, tests, and continuous integration. The included program prints:

```text
Hi there
```

This builds on [project-template](https://github.com/fulldecent/project-template), retaining its MIT license, EditorConfig, Markdown/Prettier checks, automated releases, and build attestations. Rust builds, tests, formatting, and Clippy checks use Cargo. The program has no external Rust dependencies.

## Development

Open a terminal (PowerShell on Windows). Install Rust, Git, and the native build tools using your package manager:

### Linux

Ubuntu 22.04+ or Debian 12+:

```sh
sudo apt update
sudo apt install git build-essential rustc cargo rustfmt rust-clippy
```

Fedora:

```sh
sudo dnf install git gcc rust cargo rustfmt clippy
```

### macOS (Homebrew)

```sh
xcode-select --install
```

Complete Apple's Command Line Tools installation dialog; it supplies the linker and SDK. Skip this step if those tools are already installed. Then install Rust:

```sh
brew install git rust
```

Homebrew's Rust package includes Cargo, rustfmt, and Clippy.

### Windows (winget)

```powershell
winget install --exact --id Git.Git
winget install --exact --id Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --exact --id Rustlang.Rustup
```

Allow any administrator prompts and wait for installation to finish. The C++ workload supplies the MSVC linker and Windows SDK required by Rust. Open a new PowerShell window so the updated PATH takes effect, then run:

```powershell
rustup default stable
rustup component add rustfmt clippy
```

### Get the project and run it

Select **Use this template** on GitHub to create your own repository. Clone it using its URL, or try this template directly:

```sh
git clone https://github.com/fulldecent/rust-template.git
cd rust-template
rustc --version
cargo --version
cargo run --locked
```

Rust 1.63 or newer is required. Edit `src/main.rs` in any text editor, then run these commands from the project directory:

```sh
cargo test --locked
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

Use `cargo fmt --all` to apply formatting. `tests/cli.rs` checks the program's exit status, standard output, and standard error. Cargo puts build outputs in the ignored `target/` directory.

Run the optimized binary with `./target/release/rust-template` on Linux/macOS or `.\target\release\rust-template.exe` in PowerShell. Commit `Cargo.lock` to keep application builds reproducible.

## Make it your own

- Rename the package in `Cargo.toml`, the binary name in `tests/cli.rs`, and the binary path in `.github/workflows/build-test.yml`; run `cargo generate-lockfile` afterward.
- Replace the greeting and its test with your application's behavior.
- Update this README, its badge URLs, and the copyright in [LICENSE](LICENSE). Choose a license appropriate for your project.

## Contributing and releases

[Build and test](.github/workflows/build-test.yml) builds and tests a release-mode Linux binary, then attests and uploads it as the `build` artifact. [Lint](.github/workflows/lint.yml) checks Rust formatting, Clippy, Markdown, and other file formatting.

The inherited [release workflow](.github/workflows/release.yml) uses Conventional Commits (`fix:`, `feat:`, or `BREAKING CHANGE:`) to draft release pull requests. Merging a release pull request publishes the Linux binary and attestation sidecar. Release Please uses the `simple` release type; update the package version in `Cargo.toml` and regenerate `Cargo.lock` when preparing a release.

In GitHub repository settings, enable **Allow GitHub Actions to create and approve pull requests** under **Actions → General → Workflow permissions**, and enable release immutability under **General → Releases**.

## Maintenance and references

Every quarter, review external Actions in `.github/workflows` for safe updates.

1. Based on [project-template](https://github.com/fulldecent/project-template), including its build attestation and release workflows.
1. We use title case for titles and proper nouns, not for other headings.

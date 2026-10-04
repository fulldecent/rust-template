# Rust project template

[![Build and test](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml/badge.svg?branch=main)](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml)
[![Lint](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml)

> [!IMPORTANT]
> Replace this top heading with your own project name and status badges, and replace the rest of this section with what your project does, and show it (e.g. with screenshots or command output).

This is an opinionated template for Rust projects that provides:

- A working Cargo application and CLI test
- An explicit license (MIT, at [LICENSE](LICENSE))
- [EditorConfig](.editorconfig) with modern defaults
- A [.gitignore](.gitignore) incorporating GitHub's Rust ignore template
- A [Rust toolchain file](rust-toolchain.toml) shared by local rustup and CI
- Continuous integration for Cargo builds, tests, rustfmt, and Clippy
- Markdown checks with markdownlint, other file checks with Prettier
- Build provenance attestations and automated releases

This builds on [project-template](https://github.com/fulldecent/project-template). The included program has no external Rust dependencies and prints:

```text
Hi there
```

## Project scope

> [!IMPORTANT]
> Briefly introduce your community, who they are and why they care. Then describe your project's scope and what is out-of-scope so people know which contributions are welcome.

We maintain this starting point for people creating Rust projects. It provides a small command-line application with working development and release workflows, without choosing an application framework or adding external Rust dependencies.

## Development

> [!IMPORTANT]
> Adapt these instructions for people developing your project. Keep the package-manager setup steps and replace the example commands and output as your application changes.

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

Rust 1.63 or newer is required. When using rustup, [rust-toolchain.toml](rust-toolchain.toml) selects the toolchain and installs rustfmt and Clippy automatically in this directory; CI uses the same file. Package-manager installations without rustup use the packaged toolchain.

Edit `src/main.rs` in any text editor, then run these commands from the project directory:

```sh
cargo test --locked
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

Use `cargo fmt --all` to apply formatting. `tests/cli.rs` checks the program's exit status, standard output, and standard error. Cargo puts build outputs in the ignored `target/` directory.

Run the optimized binary with `./target/release/rust-template` on Linux/macOS or `.\target\release\rust-template.exe` in PowerShell. Commit `Cargo.lock` to keep application builds reproducible.

## Make it your own

> [!IMPORTANT]
> Rename the package in `Cargo.toml`, the binary name in `tests/cli.rs`, and the binary path in `.github/workflows/build-test.yml`; run `cargo generate-lockfile` afterward.
>
> Replace the greeting and its test with your application's behavior. Update this README and its badge URLs.
>
> Choose your Rust toolchain in `rust-toolchain.toml`; use a numbered channel if your project needs a pinned compiler version.

## Contributing and releases

[Build and test](.github/workflows/build-test.yml) builds and tests a release-mode Linux binary, then attests and uploads it as the `build` artifact. [Lint](.github/workflows/lint.yml) checks Rust formatting, Clippy, Markdown, and other file formatting.

The inherited [release workflow](.github/workflows/release.yml) uses Conventional Commits (`fix:`, `feat:`, or `BREAKING CHANGE:`) to draft release pull requests. Merging a release pull request publishes the Linux binary and attestation sidecar. Release Please uses the `simple` release type; update the package version in `Cargo.toml` and regenerate `Cargo.lock` when preparing a release.

> [!IMPORTANT]
> In your GitHub repository settings, enable **Allow GitHub Actions to create and approve pull requests** under **Actions → General → Workflow permissions**, and enable release immutability under **General → Releases**.

## Maintenance and dependency updates

Every quarter we should review external Actions in `.github/workflows` for safe updates. Please send a PR if you see updates available.

## References

> [!IMPORTANT]
> We use an MIT license for this template. Carefully consider which license to apply to your own project, and replace the copyright line in LICENSE.
>
> If your project materially relied on external sources to make decisions, cite them here. We cite a text formatting policy and the project-template starting point below; adapt these references for your project.

1. Based on [project-template](https://github.com/fulldecent/project-template), including its build attestation and release workflows.
1. We use title case for titles and proper nouns, not for other headings.

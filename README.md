# ASCII Filter

> [!TIP]
> This template is a starting point you can use for every Rust project. We offer:
>
> - Clear structure and quick examples
> - Cargo builds, tests, rustfmt and Clippy in continuous integration
> - Automated releases with [Release Please](.github/workflows/release.yml) and SLSA provenance attestation
> - A shared, versioned [Rust toolchain](rust-toolchain.toml) for local development and CI
>
> What is in-scope for this template?
>
> We the people who manage Rust projects, in order to advocate for a safer installation path and great defaults for Rust, maintain this starting point for all projects.
>
> This rust-template must remain broad—addressing the needs of many kinds of projects. Every project deserves a README, and a clear rule on basic formatting questions, this is why we include continuous integration linting.
>
> We do not specify that GitHub and GitHub Actions are the only way to host projects, others may consider our GitHub-specific notes as a starting point guide for implementing outside of GitHub.
>
> And now below is the template, shown for a specific hypothetical project, enjoy!

[![Build and test](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml/badge.svg)](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml) [![Lint](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml/badge.svg)](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml)

Keep the ASCII characters. Drop everything else.

ASCII Filter reads standard input and writes only ASCII bytes to standard output. Its memory use stays bounded, no matter how large the input file is.

```console
$ printf 'caf\303\251\n' | ascii-filter
caf
```

> [!NOTE]
> Replace the project name, description, demonstration and badge URLs with your own. Show what your project does before asking people to read further.

## Installation

Select the latest release and install the command:

```sh
name="ascii-filter"
base="https://github.com/fulldecent/rust-template/releases/latest/download"
curl -fL -o "$name" "$base/$name"
curl -fL -o release.sigstore.jsonl "$base/release.sigstore.jsonl"
chmod +x "$name"
```

If you have the `gh` command, verify the file build attestation:

```sh
gh attestation verify "$name" \
  -R fulldecent/rust-template \
  --bundle release.sigstore.jsonl
```

The published binary is for Linux. Put `ascii-filter` on your `PATH`.

> [!NOTE]
> Explain what your users need to install. Replace the repository URL and command name with your own. Building from source stays in Development.

## Usage

On Linux and macOS, filter a file or a pipeline:

```sh
ascii-filter < input.txt > output.txt
printf 'caf\303\251\n' | ascii-filter
```

From PowerShell, use `cmd` redirection to preserve the file's bytes:

```powershell
cmd /c "ascii-filter < input.txt > output.txt"
```

Do not use the same file for input and output; the shell truncates the output file before the filter starts reading.

Bytes from 0 through 127 pass through unchanged, including tabs, newlines and control characters. Bytes from 128 through 255 are removed. Input does not need to be valid UTF-8. This is filtering, not transliteration: the UTF-8 input shown above becomes `caf`, not `cafe`.

The command reads one byte at a time without loading the file into memory. Read and write failures produce an error on standard error and a nonzero exit status.

> [!NOTE]
> Explain how to use your project, including the limits that matter to users.

## Development

Thank you for taking an interest in improving ASCII Filter and the pipelines of people using it!

You will need Git, your platform's native build tools and the Rust tooling described in [rust-toolchain.toml](rust-toolchain.toml). We recommend rustup to manage your rust tooling, but install it using your system package manager rather than the dangerous methods on the rustup website ([ref](#references)).

### Linux

On Ubuntu 22.04+ or Debian 12+:

```sh
sudo apt update
sudo apt install git build-essential rustup
```

On Fedora:

```sh
sudo dnf install git gcc rustup
```

### macOS

Install Apple's Command Line Tools (linker and SDK) if they are not already installed:

```sh
xcode-select --install
```

With Homebrew installed, install Git and rustup:

```sh
brew install git rustup
```

### Windows

Use winget to install Git, the native build tools (linker and SDK) and rustup:

```powershell
winget install --exact --id Git.Git
winget install --exact --id Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --exact --id Rustlang.Rustup
```

Then open a new PowerShell window so rustup is on `PATH`.

### Build

Clone the project and work from that directory. The implementation is in [src/main.rs](src/main.rs). On Linux and macOS:

> [!NOTE]
> If you do not require cross compilation, remove the `DYLD_LIBRARY_PATH="..."` part of all commands in this README.

```sh
git clone https://github.com/fulldecent/rust-template.git
cd rust-template
export PATH="$(dirname "$(rustup which cargo)"):$PATH"
export DYLD_LIBRARY_PATH="$(cd "$(dirname "$(rustup which cargo)")/../lib" && pwd)" # macOS only; omit on Linux
cargo build --locked
cargo run --locked < input.txt > output.txt
```

Keep the command small and its memory use independent of input size. Commit [Cargo.lock](Cargo.lock) so application dependencies remain reproducible.

In PowerShell:

```powershell
$bin = Split-Path (rustup which cargo)
$env:PATH = "$bin;$env:PATH"
cargo build --locked
cmd /c "cargo run --locked < input.txt > output.txt"
```

### Testing

All project updates must conform to our test suite, including [checks](./.github/workflows) run by GitHub Actions. You can also run them locally before sending proposed changes:

```sh
export PATH="$(dirname "$(rustup which cargo)"):$PATH"
export DYLD_LIBRARY_PATH="$(cd "$(dirname "$(rustup which cargo)")/../lib" && pwd)" # macOS only
cargo test --locked
cargo fmt --all
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

In PowerShell:

```powershell
$bin = Split-Path (rustup which cargo)
$env:PATH = "$bin;$env:PATH"
cargo test --locked
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

With an actively maintained version of Node.js installed, correct other formatting issues before sending proposed changes:

```sh
npx prettier@latest --check . --write
npx markdownlint-cli@latest "**/*.md" --fix
```

Cargo puts build outputs in the ignored `target/` directory. Run the optimized binary with `./target/release/ascii-filter` on Linux/macOS or `.\target\release\ascii-filter.exe` in PowerShell.

### Releases

Use `fix:`, `feat:` or `BREAKING CHANGE:` in your commit messages. This triggers our bot to make a release draft pull request. Merging that pull request triggers a new tag and GitHub Release.

> [!NOTE]
> In your GitHub repository settings, under Actions, General, Workflow permissions, check "Allow GitHub Actions to create and approve pull requests". Release Please needs this to open the release draft pull request.
>
> If you are adopting this template from an existing project and your first Release Please PR is not getting the version number right, check the Release Please documentation—you may need to add a configuration option if you do not use a "v" prefix in your release tag names, and for other situations.

### Maintenance

The project administrator completes these maintenance tasks each month. If they are 3+ months late, please remind them or send your own issue/pull request.

1. Identify external Actions in [.github/workflows](.github/workflows) and look for available new versions. Review and update them if it is safe. GitHub-supported Actions (under the actions/ organization) may require only cursory review.
1. Check new Rust releases and whether our minimum supported version or [toolchain](rust-toolchain.toml) should change. Keep the installation instructions and [Cargo.toml](Cargo.toml) consistent with that decision.

## Project scope

We are people who work with text files and command-line pipelines. Sometimes we need to keep only ASCII bytes, including in files too large to fit in memory.

ASCII Filter does that one job with standard input, standard output and bounded memory use. It accepts arbitrary bytes and preserves the order of every byte it keeps.

We specifically will not add transliteration, encoding detection, a graphical interface or options for choosing other character sets.

> [!NOTE]
> Introduce your community, explain what is in scope and say what is out of scope. Help people recognize when their own work belongs here.

## References

1. We use title case only for proper nouns, including the name of our project.
1. We use a workaround so that rustup works without replacing your system Rust programs ([issue](https://github.com/rust-lang/rustup/issues/5025#issuecomment-6869331794)).
1. We recommend to use your package manager to install rustup because the rust website prefers the unsafe `curl|sh` method ([issue](https://github.com/rust-lang/rust/issues/163468)).
1. This project is built based on [best practices documented in rust-template](https://github.com/fulldecent/rust-template/), release 1.1.0.
1. This project is built based on [best practices documented in project-template](https://github.com/fulldecent/project-template), release v1.4.1.
1. The Rust ignore rules in [.gitignore](.gitignore) come from [GitHub's Rust gitignore](https://github.com/github/gitignore/blob/main/Rust.gitignore).
1. This project is released under the [MIT license](LICENSE.md).

> [!NOTE]
> Carefully consider which license to apply to your project and replace the copyright line in [LICENSE.md](LICENSE.md). Cite external sources that materially informed your decisions, including the release of this Rust template you used.

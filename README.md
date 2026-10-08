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

[![Build and test](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml/badge.svg?branch=main)](https://github.com/fulldecent/rust-template/actions/workflows/build-test.yml) [![Lint](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/rust-template/actions/workflows/lint.yml)

Keep the ASCII characters. Drop everything else.

ASCII Filter reads standard input and writes only ASCII bytes to standard output. Its memory use stays bounded, no matter how large the input file is.

```console
$ printf 'caf\303\251\n' | ascii-filter
caf
```

> [!NOTE]
> Replace the project name, description, demonstration and badge URLs with your own. Show what your project does before asking people to read further.

## Installation

You will need rustup, Git and your platform's native build tools.

> [!WARNING]
> rustup is the Rust toolchain manager, which installs rustc and friends at versions we specify in [rust-toolchain.toml](rust-toolchain.toml). We recommend to install rustup using your package manager as this is safer than the advice on the rustup website ([ref](#references)).

If you do not use rustup and instead modify the commands below to use rustc directly, this may use your package manager's (possibly ancient) version. That build may fail and will be unsupported by this project.

```sh
"$(rustup which rustc)" --version
"$(rustup which cargo)" --version
```

Open a terminal (PowerShell on Windows) and use the instructions for your operating system.

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

If your distribution has no `rustup` package, [other rustup installation methods](https://rust-lang.github.io/rustup/installation/other.html) are available, but beware as that page does also recommend some dangerous methods ([ref](#references)).

### macOS

Install Apple's Command Line Tools if they are not already installed:

```sh
xcode-select --install
```

Complete the installation dialog; these tools supply the linker and SDK. With Homebrew installed, install Git and rustup:

```sh
brew install git rustup
```

Homebrew's `rust` formula is a standalone compiler. It does not honor `rust-toolchain.toml`. Use `rustup` instead.

### Windows

Use winget to install Git, the native build tools and rustup:

```powershell
winget install --exact --id Git.Git
winget install --exact --id Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --exact --id Rustlang.Rustup
```

Allow administrator prompts and wait for installation to finish. The C++ workload supplies the MSVC linker and Windows SDK. Open a new PowerShell window so rustup is on `PATH`.

### Build and install

Clone the project and install the command. From this directory, rustup installs the toolchain in [rust-toolchain.toml](rust-toolchain.toml) on first use. CI uses the same file.

```sh
git clone https://github.com/fulldecent/rust-template.git
cd rust-template
"$(rustup which rustc)" --version
"$(rustup which cargo)" --version
"$(rustup which cargo)" install --path . --locked
```

In PowerShell:

```powershell
& (rustup which rustc) --version
& (rustup which cargo) --version
& (rustup which cargo) install --path . --locked
```

Cargo installs `ascii-filter` in `$HOME/.cargo/bin` (`%USERPROFILE%\.cargo\bin` on Windows). Add that directory to your PATH if it is not already there.

> [!NOTE]
> Explain what your users need to install, including the tools your project is built on. Replace the repository URL and command name with your own.

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

Follow the installation instructions above to get rustup and the native build tools. Work from the project directory. The implementation is in [src/main.rs](src/main.rs); you can run it without installing it:

```sh
"$(rustup which cargo)" run --locked < input.txt > output.txt
```

In PowerShell, use `cmd /c "& (rustup which cargo) run --locked < input.txt > output.txt"`. Keep the command small and its memory use independent of input size. Commit [Cargo.lock](Cargo.lock) so application dependencies remain reproducible.

### Testing

All project updates that we release must conform to our test suite. GitHub Actions runs [checks](./.github/workflows) on pushes to `main` and pull requests. You can also run them locally before sending proposed changes:

```sh
"$(rustup which cargo)" test --locked
"$(rustup which cargo)" fmt --all -- --check
"$(rustup which cargo)" clippy --all-targets --locked -- -D warnings
"$(rustup which cargo)" build --release --locked
```

Use `"$(rustup which cargo)" fmt --all` to apply Rust formatting. The tests in [tests/cli.rs](tests/cli.rs) check empty input, all 256 possible byte values and a multi-megabyte stream. They check the exit status, standard output and standard error of the actual program.

With an actively maintained version of Node.js installed, correct other formatting issues before sending proposed changes:

```sh
npx prettier@latest --check . --write
npx markdownlint-cli@latest "**/*.md" --fix
```

Cargo puts build outputs in the ignored `target/` directory. Run the optimized binary with `./target/release/ascii-filter` on Linux/macOS or `.\target\release\ascii-filter.exe` in PowerShell.

### Releases

Use `fix:`, `feat:` or `BREAKING CHANGE:` in your commit messages. This triggers our bot to make a release draft pull request. Merging that pull request triggers a new tag and GitHub Release.

The [release workflow](.github/workflows/release.yml) uses Release Please's `simple` release type. Set the version in [Cargo.toml](Cargo.toml) and [Cargo.lock](Cargo.lock) to the proposed release version before merging the release pull request.

[Build and test](.github/workflows/build-test.yml) builds and tests a release-mode Linux binary, then attests and uploads it. The release includes `ascii-filter` and `release.sigstore.jsonl`, containing build provenance and version attestations. The published binary is for Linux; build from source on macOS or Windows.

> [!NOTE]
> In your GitHub repository settings, under Actions, General, Workflow permissions, select read and write permissions and check "Allow GitHub Actions to create and approve pull requests". Under General, Releases, enable release immutability. Attestations are available for public repositories; private repositories require GitHub Enterprise Cloud.

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
1. We recommend to use your package manager to install rustup because the rust website prefers the unsafe `curl|sh` method ([issue](https://github.com/rust-lang/rust/issues/163468)).
1. This project is built based on [best practices documented in rust-template](https://github.com/fulldecent/rust-template/), release 1.0.0.
1. The Rust ignore rules in [.gitignore](.gitignore) come from [GitHub's Rust gitignore](https://github.com/github/gitignore/blob/main/Rust.gitignore).
1. This project is released under the [MIT license](LICENSE.md).

> [!NOTE]
> Carefully consider which license to apply to your project and replace the copyright line in [LICENSE.md](LICENSE.md). Cite external sources that materially informed your decisions, including the release of this Rust template you used.

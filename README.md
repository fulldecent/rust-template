# Project template

[![Lint](https://github.com/fulldecent/project-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/project-template/actions/workflows/lint.yml)

> [!IMPORTANT]
> Replace this top heading with your own project name and status badge, and replace the rest of this section with what the project does, and show it (e.g. with screenshots).
>
> This template does not include "try it out", "installation", "usage" or "development/contributing" sections because each project should decide which, if any, of these apply. We include a GitHub Action workflow for continuous integration of file formatting but do not provide instructions for running that ad-hoc. Your own project may wish to add such instructions to your development/contributing section if your audience is comfortable using the command line and installing packages.

This is an opinionated template for every project, unless a more specific template applies, that provides:

- An explicit license (MIT, at [LICENSE](LICENSE))
- [EditorConfig](.editorconfig) with modern defaults
- A [.gitignore](.gitignore) with modern defaults
- Continuous integration to [check formatting](.github/workflows/lint.yml)
- Markdown checks with markdownlint, other file checks with Prettier
- A starting point for SLSA provenance attestation (if you produce build artifacts)
- Automated releases using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), [SemVer](https://semver.org/) and [Release Please](.github/workflows/release-please.yml)

## More specific templates

Use a more specific template if it applies. These all provide additional features:

- [Node.js template](https://github.com/fulldecent/node.js-template): Node.js module (e.g. on NPM) or application
- [GitHub Pages template](https://github.com/fulldecent/github-pages-template): collaboratively edited HTML websites
- [Swift 6 module template](https://github.com/fulldecent/swift6-module-template): reusable Swift 6 module (e.g. with Swift Package Manager)
- [Solidity template](https://github.com/fulldecent/solidity-template): Solidity contracts (technology preview)
- [Moodle plugin template](https://github.com/fulldecent/moodle-local_plugin_template): Moodle plugin (work in progress)
- [Podcast template](https://github.com/fulldecent/podcast-template): podcast on your own domain

## Project scope

> [!IMPORTANT]
> In the first paragraph, briefly introduce your community, who they are and why they care.
>
> After that, add your project's scope. This tells people what kinds of things you care about. This inspires people to become *contributors* here when they are doing their own work and see that their work is also welcome here.
>
> Last, it is good to also say what is out-of-scope. These exclusions serve the same purpose and demonstrate that you are thoughtful about your scoping.

We the people who manage projects, in order to surface up records of past decisions and make projects inviting for a growing audience, maintain this starting point for all projects.

This project-template must remain broad—addressing the needs of many kinds of projects. This includes projects related to compiling code as well as others. Every project deserves a README, and a clear rule on basic formatting questions, this is why we include continuous integration linting.

This project-template does not address items which only apply to projects involving compiling source code. We do not specify that GitHub and GitHub Actions are the only way to host projects, others may consider our GitHub-specific notes as a starting point guide for implementing outside of GitHub.

## Contributing and releases

> [!IMPORTANT]
> In your GitHub repository settings, under Actions, General, Workflow permissions, check "Allow GitHub Actions to create and approve pull requests". Release Please needs this to open the release pull request.

Commits in this project using `fix:`, `feat:` or `BREAKING CHANGE:` will draft a new release pull request. Merging that pull request triggers a new tag and GitHub Release.

## Maintenance and dependency updates

Every quarter we should check these things. Please send a PR if you see updates available:

1. Identify external Actions in [.github/workflows](./.github/workflows) scripts and look for available new versions. Review and then update to the new version if it is safe. GitHub-supported Actions (i.e. under the actions/ organization) may require only cursory review.

## References

> [!IMPORTANT]
> We use an MIT license for this template. You should carefully consider which license to apply to your own project. Replace the copyright line in LICENSE.
>
> If your project materially relied on external sources to make some decisions, cite them here.
>
> We cite a text formatting policy below. This applies to our README above as well as our workflow rules and other configuration files. If you have a different policy, then please implement it throughout.
>
> We cite the project-template release you copied.

1. We use title case for titles and proper nouns; not for headings and other things.
1. This project is built based on [best practices documented in project-template](https://github.com/fulldecent/project-template), release 1.1.1.

# Contributing to Shys

Thank you for your interest in Shys! This document explains how to contribute.

Shys is in early development. Before starting work on a significant change,
please open an issue to discuss it, so we can agree on the approach before you
invest your time.

## Reporting security issues

Never report security vulnerabilities in public issues or pull requests.
Follow the process described in [SECURITY.md](SECURITY.md).

## Reporting bugs and requesting features

Use the issue templates. When reporting a bug, never paste real secrets,
Shys links or keys: a Shys link contains the decryption key.

## Development setup

Build and test instructions will be added here once the Rust workspace is in
place.

## Pull requests

- Create a branch from `main` and keep each pull request focused on a single
  change.
- Write the pull request title using
  [Conventional Commits](https://www.conventionalcommits.org/), for example
  `feat(cli): add --expire option`. Pull requests are squash-merged, so the
  title becomes the commit message on `main`.
- Sign your commits. See GitHub's documentation on
  [commit signature verification](https://docs.github.com/en/authentication/managing-commit-signature-verification).
- Add or update tests and documentation when relevant.
- Changes to cryptographic code in `shys-core` receive extra scrutiny. Explain
  the security reasoning in the pull request description and expect the review
  to take longer.

## Code of conduct

This project follows a [Code of Conduct](CODE_OF_CONDUCT.md). By
participating, you agree to uphold it.

## Licensing of contributions

Shys uses different licenses for different components, as described in the
[README](README.md#license) and defined in `REUSE.toml`.

Unless you explicitly state otherwise, any contribution you intentionally
submit for inclusion in Shys is licensed under the license that applies to the
files it adds or modifies: MIT OR Apache-2.0 for `shys-core`, `shys-cli` and
everything outside the crates listed below, and AGPL-3.0-or-later for
`shys-server` and `shys-web`, without any additional terms or conditions.

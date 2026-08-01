# Contributing to iOverlay

Thanks for your interest in contributing!

iOverlay has a complex geometry core, so even small-looking changes can affect correctness, robustness, performance, or API behavior. Please read these guidelines before opening a pull request.

## Ways to contribute

* Report bugs or request features via GitHub Issues.
* Improve documentation and examples.
* Add tests for existing behavior.
* Submit focused code changes via pull requests.

## Development setup

* Use the Rust stable toolchain.
* Run tests: `cargo test`.
* Run formatter: `cargo fmt`.
* Run lints when possible: `cargo clippy`.

## Before opening a pull request

For non-trivial changes, please open an issue first.

This is especially important for:

* algorithm changes;
* public API changes;
* behavior changes;
* performance-related changes;
* numeric robustness changes;
* geometry edge-case fixes;
* changes touching core data structures.

The issue should describe:

* the problem;
* the expected behavior;
* examples or test cases;
* possible edge cases;
* the proposed approach, if you already have one.

Please wait for the direction to be discussed before investing time into a large implementation.

## Pull requests

Pull requests should be focused and small when possible.

A pull request should include:

* a link to the related issue, unless the change is trivial;
* a clear description of the problem being solved;
* an explanation of the chosen implementation;
* tests for new behavior or bug fixes;
* updated documentation or examples if behavior changes.

Pull requests that introduce non-trivial changes without prior discussion may be closed and redirected to an issue first.

This is not meant to discourage contributions. It helps keep the project maintainable and avoids spending review time on solutions before the problem and design are clear.

## Trivial changes

A prior issue is usually not required for:

* typo fixes;
* documentation improvements;
* small examples;
* small test additions;
* simple refactoring with no behavior change;
* clearly isolated fixes.

## Communication

If you are unsure whether your change needs an issue, please open an issue first and briefly describe what you want to change.

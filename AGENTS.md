# AGENTS.md

> This file is intended to help AI coding assistants (e.g. GitHub Copilot, OpenAI Codex, Codeium) understand this Rust project's design conventions, naming patterns, architectural assumptions, and development intentions.

It complements README.md and CONTRIBUTING.md by providing deeper semantic hints to assist automated tooling.

---

## Project Purpose

This is a Rust project focused on exploring git histories, with interactive filter and sort functionality.

Key principles:

- **Type safety first**, prefer zero-cost abstractions.
- **Async-first architecture** with \[Tokio / async-std].
- **Minimal dependencies**, prefer standard library when possible.
- **Idiomatic error handling** via `Result<T, E>` with `thiserror`.

---

## Naming Conventions

### Types

- `Foo` is a core struct.
- `FooBuilder` is its builder.
- `FooError` is its error type.
- `FooHandle` is a thread-safe wrapper or command interface.

### Modules

- `mod foo` contains related types/functions for a concept.
- `mod prelude` re-exports common traits, types, and helpers.
- `mod utils` holds generic helpers; avoid business logic here.

---

## 🧪 Testing Instructions

- Run `cargo test` from the project root to execute all unit and integration tests.
- To run a specific test module: `cargo test module_name`.
- Use `cargo test -- --nocapture` to see `println!()` or `dbg!()` output during test execution.
- Add `#[tokio::test]` to test async functions—ensure the `tokio` test feature is enabled in `Cargo.toml`.
- Place integration tests in the `tests/` directory; each file compiles as a separate crate.
- Use `cargo nextest run` if available, for parallel and deterministic test execution.
- After changing public APIs or error flows, update related tests and run the suite again.
- To run benchmarks (if enabled): `cargo bench`.
- Fix any failing tests, panics, or warnings before merging a commit.
- Always run `rustfmt -v` on each rust file after changes are made, to ensure that they meet standards. Use `-v` to ensure you have all the insight you can get. Fix any issues that cannot be automatically resolved.
- Always add or update tests when changing code—even if nobody explicitly asked.

---

## ✅ Testing Best Practices

- **Use `#[cfg(test)]` modules** inside source files for localized unit coverage.
- **Structure tests under `mod tests`** and use helper functions to reduce duplication.
- **Name tests descriptively**, following the pattern `fn does_x_given_y()`.
- **Avoid `unwrap()` or `expect()`** in tests unless testing panics intentionally.
- **Assert clearly** using `assert_eq!`, `assert!`, and `matches!` for readability.
- **Use property-based testing** (`proptest`, `quickcheck`) for input fuzzing and edge case discovery.
- **Test all error paths**, not just the happy path—especially with `Result<T, E>`.
- **Prefer `#[should_panic(expected = ...)]`** for known failure conditions, not broad assertions.
- **Use integration tests** for high-level workflows or cross-module validation.
- **Group tests by behavior**, not implementation—e.g., `auth_flow.rs`, not `auth.rs`.
- **Ensure tests are isolated**—no shared mutable state or filesystem dependencies unless sandboxed.
- **Add regression tests** for fixed bugs, linking to issue numbers or commit hashes for traceability.

---

## Architectural Assumptions

- Core logic resides in `src/lib.rs`; `src/main.rs` is glue only.
- Error types should be `Send + Sync + 'static` for integration ease.
- Prefer `tracing` over `log`.
- Use `#[derive(Debug)]` on all public types for loggability.
- Avoid panics outside of `#[test]`.

---

## File/Folder Structure (High-Level Intent)

| Path          | Intent                                                          |
| ------------- | --------------------------------------------------------------- |
| `src/lib.rs`  | Public API and core orchestration                               |
| `src/main.rs` | CLI or binary entry point                                       |
| `src/agents/` | Contains long-running async tasks or logical background workers |
| `src/types/`  | Core shared types across modules                                |
| `src/utils/`  | Utility functions with no side effects                          |
| `tests/`      | Integration-level validation                                    |
| `benches/`    | Criterion-based performance benchmarks                          |

---

## AI Completion Hints

- When writing a new module, default to private unless clearly public.
- When implementing traits, prefer `impl From<T> for U` for conversions.
- Use `?` for propagating errors unless there's a recovery path.
- If an identifier ends in `_tx` or `_rx`, it is part of a `mpsc`/`oneshot` channel pair.
- Use `spawn_blocking` only when interfacing with blocking I/O.

---

## 🔀 Pull Request Best Practices

- **One topic per PR**: Keep each pull request focused on a single logical change to simplify review and rollback.
- **Descriptive title**: Start with an issue or ticket number and a concise summary (e.g. `#42: Fix race condition in sync worker`).
- **Use Conventional Commits**:
  - **Format**: `<type>(<scope>): <short description>`
  - **Types**:
    - `feat` — new feature
    - `fix` — bug fix
    - `docs` — documentation only changes
    - `style` — formatting, missing semi-colons, etc. (no code change)
    - `refactor` — code change that neither fixes a bug nor adds a feature
    - `perf` — performance improvement
    - `test` — adding or updating tests
    - `chore` — build process, auxiliary tools, libraries
  - **Scope**: optional, but highly recommended for clarity (e.g. `feat(auth): add JWT support`).
- **Clear description**: Explain what you changed, why it matters, and any side-effects or migration steps.
- **Link related issues**: Reference issue numbers, design docs, or RFCs to provide context and traceability.
- **Include tests**: Add or update unit/integration tests to cover new behavior and edge cases.
- **Update docs**: Adjust README, CHANGELOG, or inline docs for public API changes or new features.
- **Run CI locally**: Ensure `cargo test`, `cargo fmt -- --check`, and `cargo clippy` pass before pushing.
- **Small, incremental commits**: Break large changes into atomic commits with meaningful Conventional-Commit messages.
- **Request reviewers early**: Tag relevant teammates or teams once the PR is ready for initial feedback.
- **Address feedback promptly**: Iterate on comments quickly and request re-review when done.
- **Clean up history**: Squash or rebase commits into logical units that follow the Conventional Commits spec before merge.
- **Merge only green PRs**: Do not merge until CI is passing and approvals meet repo policy.

---

## TODO Patterns

Use the following tags in comments to help AI tools prioritize code tasks:

```rust
// TODO(ai:stub): This function is a stub — generate the body.
fn validate_input(...) -> Result<_, _> {
    // ...
}

// TODO(ai:skeleton): Complete match arms for enum variants.
match foo {
    Foo::Bar => { /* TODO */ },
    Foo::Baz => { /* TODO */ },
}

// TODO(ai:tests): Add property-based tests using proptest.
```

---

## Code Style Preferences

- Prefer `match` over `if let` for enums unless only one variant matters.
- `unwrap()` is allowed in tests, never in library code.
- Modules should compile independently and minimize circular dependencies.

---

## Licensing & Attribution

This project is licensed under MIT. Contributions should maintain this license.

If AI assistants generate novel implementations, they should be reviewed and adapted to match these conventions before merge.

---

## Future Enhancements

- Implement LSP-based hover docs for all public functions.
- Add doc tests to exported functions.
- Introduce codegen markers (`// BEGIN AUTOSECTION`) if codegen is adopted.

---

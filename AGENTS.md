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

### Tests

- Unit tests go inline in `mod tests` within each file.
- Integration tests live in `tests/`.

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

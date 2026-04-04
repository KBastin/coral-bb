# Project Guidelines

## Code Style
Rust code style follows standard conventions with workspace lints enforcing pedantic clippy and forbidding unsafe code. See [coral-bb-core/src/models/user.rs](coral-bb-core/src/models/user.rs) for exemplar struct definitions with documentation.

## Architecture
Hexagonal architecture with compiler-enforced boundaries: core contains domain models and ports, app contains services and use cases. See [README.md](README.md) for planned stack and design principles.

## Build and Test
- Build: `cargo build`
- Test: `cargo test`
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all -- -D warnings`

## Conventions
Use Resource + NewResource struct pairs for domain entities. All struct fields are private to enforce encapsulation. See [coral-bb-core/src/models/](coral-bb-core/src/models/) for examples.

ID types are inconsistent (Uuid for User, String for Thread/Message, u32 for SubForum) — standardize before scaling.
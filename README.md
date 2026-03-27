# 🪸 CoralBB

> Where threads are safe, lifetimes are valid, and turbofishes swim free.

A modular, high-performance bulletin board engine for Rustaceans.

**Status: early design phase — not yet usable.**

---

## Vision

Most forum software is slow because performance was never a design goal. CoralBB takes the opposite approach: speed is a first-class concern at every layer, from database indexes to in-process caching to compile-time templates.

The goal is not to be "fast enough". The goal is to be the fastest.

---

## Planned stack

- **HTTP** — Axum
- **Database** — PostgreSQL via sqlx (async, compile-time checked queries)
- **Templates** — Askama (compiled to Rust at build time)
- **Cache** — moka (in-process, LFU eviction) + Redis for sessions
- **Runtime** — Tokio multi-thread

---

## Planned architecture

CoralBB will be structured as a Cargo workspace using **Hexagonal Architecture** (Ports & Adapters):

- `coral-bb-core` — domain models and port traits, zero external dependencies
- `coral-bb-app` — services and use cases, depends only on core
- `coral-bb-db` — sqlx adapter implementing core ports
- `coral-bb-cache` — moka and Redis adapters
- `coral-bb-web` — Axum handlers and Askama templates
- `coral-bb-cli` — admin tooling

The core never depends on infrastructure. Boundaries are enforced by the compiler through `Cargo.toml`, not just by convention.

---

## Contributing

This is a tightly controlled open source project. Contributions are welcome but all design decisions go through GitHub Issues before any code is written.

If you want to contribute, open or comment on an issue first.

**Good first issues will be tagged** [`good first issue`](../../issues?q=is%3Aissue+label%3A%22good+first+issue%22) on GitHub.

---

## Licence

MIT
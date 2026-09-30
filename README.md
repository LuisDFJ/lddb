# LD-DB: A Database Built from Scratch in Rust

LD-DB is a learning project: a database built from scratch in Rust with no external dependencies. It follows the [Trial of Code "Build Your Own Database" challenge](https://trialofcode.org/database/), with the twist of implementing everything in Rust.

> **Status:** Work in progress. Not yet usable as a library or standalone binary.

## Roadmap

- [ ] ACID key/value storage engine (in progress)
- [ ] _Next milestones, to be filled in as the challenge progresses_

## ACID Key/Value Storage Engine

The first milestone is a key/value storage engine with ACID guarantees. The approach:

1. **In-memory store.** A `KV` struct built on top of a `HashMap` serves reads and writes from memory, supporting:
   - `Set(key, value) -> Result<bool>`
   - `Get(key) -> Result<V, bool>`
   - `Del(key) -> Result<bool>`
2. **Binary serialization.** Each operation is encoded as a binary `Entry`.
3. **Append-only log (next).** Serialized entries will be atomically appended to a log file, which gives the engine its atomicity and durability guarantees. Replaying the log on startup will rebuild the in-memory state.

## Getting Started

LD-DB has no external dependencies; you only need a recent [Rust toolchain](https://rustup.rs/).

```sh
git clone https://github.com/LuisDFJ/lddb.git
cd ld-db
```

Usage instructions will be added once there is a working minimal version.

## Testing

```sh
cargo test
```

Current coverage:

- `Set`, `Get` and `Del` operations on the `KV` struct
- Serialization and deserialization of a binary `Entry`

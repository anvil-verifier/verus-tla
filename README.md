# verus_temporal_logic

[![CI](https://github.com/anvil-verifier/Verus-TLA-embedding/actions/workflows/ci.yml/badge.svg)](https://github.com/anvil-verifier/Verus-TLA-embedding/actions/workflows/ci.yml)

A [Verus](https://github.com/verus-lang/verus) embedding of TLA+ temporal logic, ported from [Anvil](https://github.com/anvil-verifier/anvil).

The crate provides:

- **`defs`** — `Execution`, `TempPred`, and the temporal connectives (`always`, `eventually`, `leads_to`, `weak_fairness`, `valid`, `entails`, `tla_forall`, `tla_exists`, `stable`, …).
- **`rules`** — verified TLA helper lemmas (modus ponens, weak-fairness, induction, monotonicity, ESR, etc.) for liveness and safety proofs.
- **`state_machine`** — `Action`, `StateMachine`, and `NetworkStateMachine` structs for defining state-transition systems with preconditions, transitions, and weak-fairness assumptions.

**Execution Model**

```rust
pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}
```

## Usage

```toml
[dependencies]
vstd = "=0.0.0-2026-06-14-0213"
verus_temporal_logic = { git = "https://github.com/anvil-verifier/Verus-TLA-embedding" }
```

The `vstd` version must match the Verus toolchain you build with.

## Example

See [`src/mutex_example.rs`](src/mutex_example.rs).

## Build & verify

This crate uses [`cargo verus`](https://github.com/verus-lang/verus).

```sh
# Verify the library (defs + rules + state_machine)
cargo verus verify --lib

# Verify the mutex liveness example
cargo verus verify --bin mutex_example
```

## License

MIT (inherited from the original [anvil temporal_logic module](https://github.com/anvil-verifier/anvil/tree/main/src/temporal_logic)).

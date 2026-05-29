# verus_temporal_logic

[![CI](https://github.com/anvil-verifier/Verus-TLA-embedding/actions/workflows/ci.yml/badge.svg)](https://github.com/anvil-verifier/Verus-TLA-embedding/actions/workflows/ci.yml)

A [Verus](https://github.com/verus-lang/verus) embedding of TLA+ temporal logic, ported from [Anvil](https://github.com/anvil-verifier/anvil).

The crate provides:

- **`defs`** — `Execution`, `TempPred`, and the temporal connectives
  (`always`, `eventually`, `leads_to`, `weak_fairness`, `valid`, `entails`,
  `tla_forall`, `tla_exists`, `stable`, …).
- **`rules`** — verified lemmas (modus ponens, weak-fairness,
  induction, monotonicity, ESR, etc.) for proving liveness and safety
  about state-transition systems.

**Execution Model**

```rust
pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}
```

## Usage

```toml
[dependencies]
vstd = "=0.0.0-2026-05-24-0157"
verus_temporal_logic = { git = "https://github.com/anvil-verifier/Verus-TLA-embedding" }
```

```rust
use verus_temporal_logic::defs::*;
use verus_temporal_logic::rules::*;
```

The `vstd` version must match the Verus toolchain you build with;
this crate pins it because Verus pre-releases break frequently.

## Example

Chaining two `leads_to` claims by transitivity:

```rust
use vstd::prelude::*;
use verus_temporal_logic::defs::*;
use verus_temporal_logic::rules::leads_to_trans;

verus! {

// Given `spec |= p ~> q` and `spec |= q ~> r`, conclude `spec |= p ~> r`.
proof fn example<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures
        spec.entails(p.leads_to(r)),
{
    leads_to_trans(spec, p, q, r);
}

}
```

See [`src/rules.rs`](src/rules.rs) for the full set of lemmas.

## Build & verify

This crate uses [`cargo verus`](https://github.com/verus-lang/verus).

```sh
cargo verus verify --lib -- --rlimit 50 --time
```

## License

MIT (inherited from the original [anvil temporal_logic module](https://github.com/anvil-verifier/anvil/tree/main/src/temporal_logic)).

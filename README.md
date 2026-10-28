# Verus-TLA

[![CI](https://github.com/anvil-verifier/verus-tla/actions/workflows/ci.yml/badge.svg)](https://github.com/anvil-verifier/verus-tla/actions/workflows/ci.yml)

A [Verus](https://github.com/verus-lang/verus) embedding of TLA (Temporal Logic of Actions), ported from [Anvil](https://github.com/anvil-verifier/anvil).

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
vstd = { git = "https://github.com/verus-lang/verus.git" }
verus_temporal_logic = { git = "https://github.com/anvil-verifier/verus-tla" }
```

Build your local Verus binary following the instructions in [`BUILD.md`](https://github.com/verus-lang/verus/blob/main/BUILD.md) from the `main` branch of Verus.

## Opaque operators and broadcast groups

The temporal operators `always`, `eventually`, `later`, `leads_to`, `stable` and `weak_fairness` are opaque: the SMT solver sees them as uninterpreted atoms rather than as quantifiers over executions, similar to how TLAPS [coalesces](https://arxiv.org/abs/1409.3819) modal subformulas before calling a first-order prover. Broadcast groups, which callers enable explicitly, recover what the definitions used to provide:

- **`rules::group_tla_rules`** is opt-in: add `broadcast use verus_temporal_logic::rules::group_tla_rules;` to a module (or a proof body), or use one of its category groups. It unfolds the derived operators (`leads_to`, `stable`, `weak_fairness`) into `always`/`eventually`, distributes `later` over the boolean connectives and rewrites `later(lift_state(p))` to `lift_state_prime(p)`, so action-level facts over `(s, s_prime)` discharge primed obligations. It also lets the solver apply common rules on its own: necessitation (`valid(p)` gives `spec.entails(always(p))`), `entails_and`, `always_and_equality`, `entails_implies_leads_to`, `or_leads_to`, `leads_to_exists_intro`, `always_p_is_stable`, `stable_and` and others. Each of these rules is triggered by its conclusion, so the solver chains them backwards from the goal; rules that need a choice the goal does not determine (a witness, an index for `tla_forall`, the middle formula of a transitivity step, an invariant) are not included and are still called explicitly. These lemmas are still ordinary proof functions and can be called explicitly as before. `group_tla_rules` is the union of one group per kind of rule: `group_tla_safety_rules` (`always`), `group_tla_later_rules`, `group_tla_stability_rules`, `group_tla_liveness_rules` (`eventually`, `leads_to`, `weak_fairness`), `group_tla_entails_rules`, `group_tla_constant_rules` (`true_pred`, `false_pred`), `group_tla_forall_rules` and `group_tla_exists_rules`.
- **`rules::group_tla_equality_rules`** (opt-in, not in `group_tla_rules`) adds `true_pred().and(p) == p` and `p.and(true_pred()) == p`. On Anvil it removes 29 `temp_pred_equality` calls but raises the total rlimit by about 12%.

No exported rule mentions executions; the meaning of the operators over executions is used only inside this crate to prove the rules. Proofs that used to reason about `ex.suffix(i)` can instead use helper lemmas stated over state and action predicates, such as `tla_forall_lift_state_equality` and `tla_exists_lift_state_equality` (quantifiers over lifted state predicates), `spec_entails_always_lift_state_forall_within_domain_equality`, `leads_to_always_lift_state_forall_within_domain`, `refinement_mapping_preserves_invariant` and `history_variable_preserves_invariant`. Lemmas about a family `a_to_p: spec_fn(A) -> StatePred<T>` restricted to a `domain` take the combined predicate as an argument `p` together with a first-order premise such as `forall |s| p(s) <==> (forall |a| domain.contains(a) ==> a_to_p(a)(s))`, so callers can pass their own named predicate without proving closure equalities. The domain is a `Set<A>` (finite) when the proof needs finiteness, and an `ISet<A>` otherwise. Weakening an invariant by a state-level implication `forall |s| p(s) ==> q(s)` is automatic (`always_lift_state_weaken`, in `group_tla_safety_rules`) when both `spec.entails(always(lift_state(p)))` and `spec.entails(always(lift_state(q)))` appear in the proof; `always_weaken` remains for arbitrary temporal predicates. `TempPred` is `#[verifier::ext_equal]`, so `assert(p =~= q)` is an alternative to `temp_pred_equality(p, q)` when the two predicates agree propositionally.

## Example

See [`src/mutex_example.rs`](src/mutex_example.rs) and [proofs](https://github.com/anvil-verifier/anvil/blob/main/src/controllers/vdeployment_controller/proof/liveness/rolling_update/composition.rs) in Anvil.

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

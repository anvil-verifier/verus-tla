// A liveness proof for writes under optimistic concurrency control.
//
// Model: a controller updates an object through an API server. The object
// carries a resource version. The controller reads the version and then
// sends a write carrying that version. The API server accepts the write only
// if the version is still the current one; otherwise the write fails and the
// controller retries. Other controllers can update the object at any time,
// which bumps the version.
//
// Weak or strong fairness of the accept action is not enough: another
// controller can always bump the version between the controller's read and
// its write, so the accept action is never enabled. Under hyperfairness of
// the accept action, the controller's write is eventually accepted.
//
// This demonstrates `hf1`, `can_enable_unfold`, and `leads_to_apply`.
#![allow(unused_imports)]
use verus_temporal_logic::action::*;
use verus_temporal_logic::defs::*;
use verus_temporal_logic::rules::*;
use vstd::prelude::*;

verus! {

struct ClusterState {
    version: nat,
    read: Option<nat>,
    write: Option<nat>,
    updated: bool,
}

spec fn init() -> StatePred<ClusterState> {
    |s: ClusterState| s.read is None && s.write is None && !s.updated
}

spec fn next() -> ActionPred<ClusterState> {
    |s, s_prime: ClusterState| {
        ||| controller_reads().forward(())(s, s_prime)
        ||| controller_writes().forward(())(s, s_prime)
        ||| api_server_accepts_write().forward(())(s, s_prime)
        ||| api_server_rejects_write().forward(())(s, s_prime)
        ||| other_controller_updates().forward(())(s, s_prime)
        ||| stutter().forward(())(s, s_prime)
    }
}

spec fn controller_reads() -> Action<ClusterState, (), ()> {
    Action {
        precondition: |input: (), s: ClusterState| {
            s.read is None && s.write is None
        },
        transition: |input: (), s: ClusterState| {
            (ClusterState { read: Some(s.version), ..s }, ())
        },
    }
}

spec fn controller_writes() -> Action<ClusterState, (), ()> {
    Action {
        precondition: |input: (), s: ClusterState| {
            s.read is Some && s.write is None
        },
        transition: |input: (), s: ClusterState| {
            (ClusterState { read: None, write: s.read, ..s }, ())
        },
    }
}

spec fn api_server_accepts_write() -> Action<ClusterState, (), ()> {
    Action {
        precondition: |input: (), s: ClusterState| {
            s.write == Some(s.version)
        },
        transition: |input: (), s: ClusterState| {
            (ClusterState { version: s.version + 1, write: None, updated: true, ..s }, ())
        },
    }
}

spec fn api_server_rejects_write() -> Action<ClusterState, (), ()> {
    Action {
        precondition: |input: (), s: ClusterState| {
            s.write is Some && s.write != Some(s.version)
        },
        transition: |input: (), s: ClusterState| {
            (ClusterState { write: None, ..s }, ())
        },
    }
}

spec fn other_controller_updates() -> Action<ClusterState, (), ()> {
    Action {
        precondition: |input: (), s: ClusterState| { true },
        transition: |input: (), s: ClusterState| {
            (ClusterState { version: s.version + 1, ..s }, ())
        },
    }
}

spec fn stutter() -> Action<ClusterState, (), ()> {
    Action {
        precondition: |input: (), s: ClusterState| { true },
        transition: |input: (), s: ClusterState| { (s, ()) },
    }
}

spec fn accept() -> ActionPred<ClusterState> {
    api_server_accepts_write().forward(())
}

spec fn updated() -> StatePred<ClusterState> {
    |s: ClusterState| s.updated
}

// The number of steps to reach a state where the accept action is enabled,
// if no other controller updates the object in the meantime.
spec fn steps_to_accept(s: ClusterState) -> nat {
    if s.write == Some(s.version) {
        0
    } else if s.write is Some {
        if s.read == Some(s.version) { 2 } else if s.read is Some { 5 } else { 3 }
    } else if s.read == Some(s.version) {
        1
    } else if s.read is Some {
        4
    } else {
        2
    }
}

proof fn next_can_enable_accept(s: ClusterState)
    ensures can_enable(next(), accept())(s),
    decreases steps_to_accept(s),
{
    if s.write == Some(s.version) {
        let s_prime = ClusterState { version: s.version + 1, write: None, updated: true, ..s };
        let (n, a) = (next(), accept());
        let next_and_accept = |s: ClusterState, s_prime: ClusterState| n(s, s_prime) && a(s, s_prime);
        assert(next_and_accept(s, s_prime));
        assert(enabled(next_and_accept)(s));
    } else {
        // Take the step that gets closer to enabling the accept action.
        let s_prime = if s.write is Some {
            ClusterState { write: None, ..s }
        } else if s.read is Some {
            ClusterState { read: None, write: s.read, ..s }
        } else {
            ClusterState { read: Some(s.version), ..s }
        };
        assert(next()(s, s_prime)) by {
            if s.write is Some {
                assert(api_server_rejects_write().forward(())(s, s_prime));
            } else if s.read is Some {
                assert(controller_writes().forward(())(s, s_prime));
            } else {
                assert(controller_reads().forward(())(s, s_prime));
            }
        }
        next_can_enable_accept(s_prime);
    }
    can_enable_unfold(next(), accept(), s);
}

proof fn write_is_eventually_accepted(model: TempPred<ClusterState>)
    requires
        model.entails(lift_state(init())),
        model.entails(always(lift_action(next()))),
        model.entails(hyperfairness(next(), accept())),
    ensures
        model.entails(eventually(lift_state(updated()))),
{
    let not_updated = |s: ClusterState| !s.updated;
    assert forall |s: ClusterState| #[trigger] not_updated(s) implies can_enable(next(), accept())(s) by {
        next_can_enable_accept(s);
    }
    hf1(model, next(), accept(), not_updated, updated());

    entails_trans(model, lift_state(init()), lift_state(not_updated));
    leads_to_apply(model, lift_state(not_updated), lift_state(updated()));
}

fn main() {}

}

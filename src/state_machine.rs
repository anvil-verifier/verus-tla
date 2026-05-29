// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::action::*;
use vstd::{multiset::*, prelude::*};

verus! {

#[verifier(reject_recursive_types(State))]
#[verifier(reject_recursive_types(Input))]
#[verifier(reject_recursive_types(ActionInput))]
#[verifier(reject_recursive_types(Output))]
#[verifier(reject_recursive_types(Step))]
pub struct StateMachine<State, Input, ActionInput, Output, Step> {
    pub init: spec_fn(State) -> bool,
    pub actions: Set<Action<State, ActionInput, Output>>,
    pub step_to_action: spec_fn(Step) -> Action<State, ActionInput, Output>,
    pub action_input: spec_fn(Step, Input) -> ActionInput,
}

impl<State, Input, ActionInput, Output, Step> StateMachine<State, Input, ActionInput, Output, Step> {
    pub open spec fn next_result(self, input: Input, s: State) -> ActionResult<State, Output> {
        if exists |step| (#[trigger] (self.step_to_action)(step).precondition)((self.action_input)(step, input), s) {
            let witness_step = choose |step| (#[trigger] (self.step_to_action)(step).precondition)((self.action_input)(step, input), s);
            let action = (self.step_to_action)(witness_step);
            let action_input = (self.action_input)(witness_step, input);
            ActionResult::Enabled((action.transition)(action_input, s).0, (action.transition)(action_input, s).1)
        } else {
            ActionResult::Disabled
        }
    }

    pub open spec fn next_action_result(self, action: Action<State, ActionInput, Output>, action_input: ActionInput, s: State) -> ActionResult<State, Output> {
        if (action.precondition)(action_input, s) {
            ActionResult::Enabled((action.transition)(action_input, s).0, (action.transition)(action_input, s).1)
        } else {
            ActionResult::Disabled
        }
    }
}

#[verifier(reject_recursive_types(State))]
#[verifier(reject_recursive_types(MessageOps))]
pub struct NetworkStateMachine<State, MessageOps> {
    pub init: spec_fn(State) -> bool,
    pub deliver: Action<State, MessageOps, ()>,
}

impl<State, MessageOps> NetworkStateMachine<State, MessageOps> {
    pub open spec fn next_result(self, msg_ops: MessageOps, s: State) -> ActionResult<State, ()> {
        if (self.deliver.precondition)(msg_ops, s) {
            ActionResult::Enabled((self.deliver.transition)(msg_ops, s).0, (self.deliver.transition)(msg_ops, s).1)
        } else {
            ActionResult::Disabled
        }
    }
}

}

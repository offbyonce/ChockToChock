// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! A pilot's progress through checklists during one flight (decision 0012).
//!
//! A skipped step is still open: a list is complete only when every counted step is done. Ticks
//! and skips record the time they happened, passed in by the caller, since the core has no clock.
//! Steps the simulator does not model can be ticked but are not counted.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::{Checklist, Step, StepId};

/// Milliseconds since 1970-01-01 UTC, as the interface's clock gives them.
pub type Millis = u64;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SessionError {
    #[error("step `{0}` is not in this checklist")]
    UnknownStep(StepId),
}

/// Done and skipped steps, with when each happened. A step is never both.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Session {
    #[cfg_attr(feature = "ts", ts(type = "Record<string, number>"))]
    done: BTreeMap<StepId, Millis>,
    #[cfg_attr(feature = "ts", ts(type = "Record<string, number>"))]
    skipped: BTreeMap<StepId, Millis>,
}

/// Where a step stands. `Current` is the first step in view that is neither done nor skipped.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepState {
    ToDo,
    Current,
    Done,
    Skipped,
}

/// Counts for a progress bar. Only counted steps are included (not `not_simulated` ones).
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub done: usize,
    pub skipped: usize,
    pub to_do: usize,
    pub total: usize,
}

impl Progress {
    /// True when every counted step is done. Skipped steps are still open.
    pub fn complete(&self) -> bool {
        self.done == self.total
    }

    /// Steps still open: skipped or to do.
    pub fn open(&self) -> usize {
        self.skipped + self.to_do
    }

    /// Adds another list's counts, for a phase made of several lists.
    #[must_use]
    pub fn plus(self, other: Self) -> Self {
        Self {
            done: self.done + other.done,
            skipped: self.skipped + other.skipped,
            to_do: self.to_do + other.to_do,
            total: self.total + other.total,
        }
    }
}

/// Whether a step counts toward progress and completion.
pub fn counts(step: &Step) -> bool {
    !step.not_simulated
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_done(&self, id: &StepId) -> bool {
        self.done.contains_key(id)
    }

    pub fn is_skipped(&self, id: &StepId) -> bool {
        self.skipped.contains_key(id)
    }

    /// When the step was ticked, if it is done.
    pub fn done_at(&self, id: &StepId) -> Option<Millis> {
        self.done.get(id).copied()
    }

    /// When the step was skipped, if it is skipped.
    pub fn skipped_at(&self, id: &StepId) -> Option<Millis> {
        self.skipped.get(id).copied()
    }

    /// Mark a step done at `at` (un-skipping it if needed).
    pub fn tick(&mut self, list: &Checklist, id: &StepId, at: Millis) -> Result<(), SessionError> {
        Self::check(list, id)?;
        self.skipped.remove(id);
        self.done.insert(id.clone(), at);
        Ok(())
    }

    /// Clear a step's done mark.
    pub fn untick(&mut self, list: &Checklist, id: &StepId) -> Result<(), SessionError> {
        Self::check(list, id)?;
        self.done.remove(id);
        Ok(())
    }

    /// Skip a step at `at` (clearing its done mark if needed). It stays open.
    pub fn skip(&mut self, list: &Checklist, id: &StepId, at: Millis) -> Result<(), SessionError> {
        Self::check(list, id)?;
        self.done.remove(id);
        self.skipped.insert(id.clone(), at);
        Ok(())
    }

    /// Each step's state, in order. The first step that is neither done nor skipped is current.
    pub fn states<'a>(
        &self,
        steps: impl IntoIterator<Item = &'a Step>,
    ) -> Vec<(&'a StepId, StepState)> {
        let mut current_found = false;
        steps
            .into_iter()
            .map(|step| {
                let state = if self.is_done(&step.id) {
                    StepState::Done
                } else if self.is_skipped(&step.id) {
                    StepState::Skipped
                } else if current_found {
                    StepState::ToDo
                } else {
                    current_found = true;
                    StepState::Current
                };
                (&step.id, state)
            })
            .collect()
    }

    /// Progress over the given steps, counting only those that count.
    pub fn progress<'a>(&self, steps: impl IntoIterator<Item = &'a Step>) -> Progress {
        let mut p = Progress::default();
        for step in steps.into_iter().filter(|s| counts(s)) {
            p.total += 1;
            if self.is_done(&step.id) {
                p.done += 1;
            } else if self.is_skipped(&step.id) {
                p.skipped += 1;
            } else {
                p.to_do += 1;
            }
        }
        p
    }

    /// Clear everything for a new flight.
    pub fn clear(&mut self) {
        self.done.clear();
        self.skipped.clear();
    }

    fn check(list: &Checklist, id: &StepId) -> Result<(), SessionError> {
        if list.contains(id) {
            Ok(())
        } else {
            Err(SessionError::UnknownStep(id.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::model::{ListKind, Step};

    fn step(id: &str) -> Step {
        Step::new(id, format!("{id} text"), "ON")
    }

    fn list() -> Checklist {
        let mut unmodeled = step("d");
        unmodeled.not_simulated = true;
        Checklist {
            id: "demo".into(),
            title: "Demo".into(),
            kind: ListKind::Checklist,
            basis: vec!["sim-observed".into()],
            memory: false,
            when: None,
            why: None,
            steps: vec![step("a"), step("b"), step("c"), unmodeled],
        }
    }

    fn id(s: &str) -> StepId {
        StepId::new(s)
    }

    #[test]
    fn tick_skip_and_progress() {
        let l = list();
        let mut s = Session::new();
        s.tick(&l, &id("a"), 1_000).unwrap();
        s.skip(&l, &id("c"), 2_000).unwrap();
        assert_eq!(
            s.progress(&l.steps),
            Progress {
                done: 1,
                skipped: 1,
                to_do: 1,
                total: 3
            }
        );
        assert_eq!(s.done_at(&id("a")), Some(1_000));
        assert_eq!(s.skipped_at(&id("c")), Some(2_000));
        s.tick(&l, &id("c"), 3_000).unwrap();
        assert!(!s.is_skipped(&id("c")), "ticking clears skip");
        assert_eq!(s.done_at(&id("c")), Some(3_000));
    }

    #[test]
    fn a_skipped_step_keeps_the_list_open() {
        let l = list();
        let mut s = Session::new();
        s.tick(&l, &id("a"), 1).unwrap();
        s.tick(&l, &id("b"), 2).unwrap();
        s.skip(&l, &id("c"), 3).unwrap();
        let p = s.progress(&l.steps);
        assert!(!p.complete(), "a skipped step is still open");
        assert_eq!(p.open(), 1);
        s.tick(&l, &id("c"), 4).unwrap();
        assert!(s.progress(&l.steps).complete());
    }

    #[test]
    fn steps_the_simulator_does_not_model_can_be_ticked_but_do_not_count() {
        let l = list();
        let mut s = Session::new();
        for (n, step) in ["a", "b", "c"].into_iter().enumerate() {
            s.tick(&l, &id(step), n as u64).unwrap();
        }
        assert!(s.progress(&l.steps).complete(), "d is not counted");
        s.tick(&l, &id("d"), 9).unwrap();
        assert!(s.is_done(&id("d")));
        assert_eq!(s.progress(&l.steps).total, 3);
    }

    #[test]
    fn the_first_open_step_is_current() {
        let l = list();
        let mut s = Session::new();
        s.tick(&l, &id("a"), 1).unwrap();
        s.skip(&l, &id("b"), 2).unwrap();
        let states: Vec<StepState> = s.states(&l.steps).into_iter().map(|(_, st)| st).collect();
        assert_eq!(
            states,
            [
                StepState::Done,
                StepState::Skipped,
                StepState::Current,
                StepState::ToDo
            ]
        );
    }

    #[test]
    fn unknown_steps_are_rejected() {
        let mut s = Session::new();
        let err = s.tick(&list(), &id("nope"), 1).unwrap_err();
        assert_eq!(err, SessionError::UnknownStep(id("nope")));
    }

    #[test]
    fn a_session_round_trips_through_json() {
        let l = list();
        let mut s = Session::new();
        s.tick(&l, &id("a"), 1_794_000_000_000).unwrap();
        s.skip(&l, &id("b"), 1_794_000_000_500).unwrap();
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(
            json,
            r#"{"done":{"a":1794000000000},"skipped":{"b":1794000000500}}"#
        );
        assert_eq!(serde_json::from_str::<Session>(&json).unwrap(), s);
    }

    proptest! {
        /// Whatever the pilot does, a step is never both done and skipped, at most one step is
        /// current, and the counts always add up.
        #[test]
        fn states_and_counts_stay_consistent(actions in proptest::collection::vec((0u8..3, 0usize..4), 0..40)) {
            let l = list();
            let mut s = Session::new();
            for (n, (action, idx)) in actions.into_iter().enumerate() {
                let step = l.steps[idx].id.clone();
                match action {
                    0 => s.tick(&l, &step, n as u64).unwrap(),
                    1 => s.untick(&l, &step).unwrap(),
                    _ => s.skip(&l, &step, n as u64).unwrap(),
                }
                for st in &l.steps {
                    prop_assert!(!(s.is_done(&st.id) && s.is_skipped(&st.id)));
                }
                let states = s.states(&l.steps);
                prop_assert!(states.iter().filter(|(_, st)| *st == StepState::Current).count() <= 1);
                let p = s.progress(&l.steps);
                prop_assert_eq!(p.done + p.skipped + p.to_do, p.total);
            }
        }
    }
}

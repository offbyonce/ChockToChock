// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! A pilot's progress through checklists during one flight.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::model::{Checklist, StepId};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SessionError {
    #[error("step `{0}` is not in this checklist")]
    UnknownStep(StepId),
}

/// Done and skipped steps. A step is never both.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    done: BTreeSet<StepId>,
    skipped: BTreeSet<StepId>,
}

/// Counts for a progress bar.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    pub done: usize,
    pub skipped: usize,
    pub total: usize,
}

impl Progress {
    /// True when every step is either done or skipped.
    pub fn complete(&self) -> bool {
        self.done + self.skipped == self.total
    }
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_done(&self, id: &StepId) -> bool {
        self.done.contains(id)
    }

    pub fn is_skipped(&self, id: &StepId) -> bool {
        self.skipped.contains(id)
    }

    /// Mark a step done (un-skipping it if needed).
    pub fn tick(&mut self, list: &Checklist, id: &StepId) -> Result<(), SessionError> {
        Self::check(list, id)?;
        self.skipped.remove(id);
        self.done.insert(id.clone());
        Ok(())
    }

    /// Clear a step's done mark.
    pub fn untick(&mut self, list: &Checklist, id: &StepId) -> Result<(), SessionError> {
        Self::check(list, id)?;
        self.done.remove(id);
        Ok(())
    }

    /// Skip a step (clearing its done mark if needed).
    pub fn skip(&mut self, list: &Checklist, id: &StepId) -> Result<(), SessionError> {
        Self::check(list, id)?;
        self.done.remove(id);
        self.skipped.insert(id.clone());
        Ok(())
    }

    /// The first step that is neither done nor skipped.
    pub fn next_step<'a>(&self, list: &'a Checklist) -> Option<&'a StepId> {
        list.steps
            .iter()
            .map(|s| &s.id)
            .find(|id| !self.done.contains(*id) && !self.skipped.contains(*id))
    }

    pub fn progress(&self, list: &Checklist) -> Progress {
        let count =
            |set: &BTreeSet<StepId>| list.steps.iter().filter(|s| set.contains(&s.id)).count();
        Progress {
            done: count(&self.done),
            skipped: count(&self.skipped),
            total: list.steps.len(),
        }
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
        Step::new(id, format!("{id} text"), "SET")
    }

    fn list() -> Checklist {
        Checklist {
            id: "demo".into(),
            title: "Demo".into(),
            kind: ListKind::Checklist,
            basis: vec!["sim-observed".into()],
            memory: false,
            when: None,
            why: None,
            steps: vec![step("a"), step("b"), step("c")],
        }
    }

    #[test]
    fn tick_skip_and_progress() {
        let l = list();
        let mut s = Session::new();
        s.tick(&l, &StepId::new("a")).unwrap();
        s.skip(&l, &StepId::new("c")).unwrap();
        assert_eq!(
            s.progress(&l),
            Progress {
                done: 1,
                skipped: 1,
                total: 3
            }
        );
        assert_eq!(s.next_step(&l), Some(&StepId::new("b")));
        s.tick(&l, &StepId::new("c")).unwrap();
        assert!(!s.is_skipped(&StepId::new("c")), "ticking clears skip");
    }

    #[test]
    fn unknown_steps_are_rejected() {
        let mut s = Session::new();
        let err = s.tick(&list(), &StepId::new("nope")).unwrap_err();
        assert_eq!(err, SessionError::UnknownStep(StepId::new("nope")));
    }

    proptest! {
        /// Whatever the pilot does, a step is never both done and skipped,
        /// and the counts never exceed the list length.
        #[test]
        fn done_and_skipped_never_overlap(actions in proptest::collection::vec((0u8..3, 0usize..3), 0..40)) {
            let l = list();
            let mut s = Session::new();
            for (action, idx) in actions {
                let id = l.steps[idx].id.clone();
                match action {
                    0 => s.tick(&l, &id).unwrap(),
                    1 => s.untick(&l, &id).unwrap(),
                    _ => s.skip(&l, &id).unwrap(),
                }
                for st in &l.steps {
                    prop_assert!(!(s.is_done(&st.id) && s.is_skipped(&st.id)));
                }
                let p = s.progress(&l);
                prop_assert!(p.done + p.skipped <= p.total);
            }
        }
    }
}

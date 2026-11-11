// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `faults.yaml`: likely causes when a step goes wrong.

use crate::model::StepId;

data_types! {
    /// The likely causes when a step does not give the expected result.
    pub struct Fault {
        pub steps: Vec<StepId>,
        pub causes: Vec<String>,
        pub action: String,
    }

    /// `faults.yaml`.
    pub struct FaultsFile {
        pub faults: Vec<Fault>,
    }
}

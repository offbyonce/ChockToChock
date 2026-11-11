// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `lights.yaml`: what panel lights mean.

use crate::model::Confirmation;

data_types! {
    /// What a panel light means.
    pub struct LightNote {
        /// `<panel>/<section>/<label>/<n>`, where n counts items with that label from 1.
        pub locator: String,
        pub meaning: String,
        pub when: String,
        pub action: String,
        /// True when seeing this light is normal.
        pub normal: bool,
        /// Internal record of what this note was confirmed against (decision 0013).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub confirmed_by: Vec<Confirmation>,
    }

    /// `lights.yaml`.
    pub struct LightsFile {
        pub lights: Vec<LightNote>,
    }
}

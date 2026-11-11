// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `lights.yaml`: what panel lights mean.

use crate::model::Status;

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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<Status>,
    }

    /// `lights.yaml`.
    pub struct LightsFile {
        pub lights: Vec<LightNote>,
    }
}

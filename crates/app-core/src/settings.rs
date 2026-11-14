// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! What outlives a flight (decision 0012). Saved on the device with a format number.

use std::collections::{BTreeMap, BTreeSet};

use checklist_core::StepId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The saved format of [`Settings`]. Raise it, with a migration and tests, when the shape changes.
pub const SETTINGS_FORMAT: u32 = 5;

/// The highest dimming step; 0 is full brightness.
pub const MAX_DIMMING: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Theme {
    /// Follow the device's light or dark setting.
    #[default]
    System,
    Dark,
    Light,
}

/// Text and controls size, applied on top of the wide-screen scaling (decision 0021).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TextSize {
    #[default]
    #[serde(rename = "100")]
    Standard,
    #[serde(rename = "115")]
    Larger,
    #[serde(rename = "130")]
    Largest,
}

impl TextSize {
    /// The size as a percentage of the standard size.
    #[must_use]
    pub const fn percent(self) -> u16 {
        match self {
            Self::Standard => 100,
            Self::Larger => 115,
            Self::Largest => 130,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Detail {
    /// Item and value only.
    Quick,
    /// Where it is, why, and the rest.
    #[default]
    Expanded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ListView {
    /// Every item in the list.
    #[default]
    Checklist,
    /// One item at a time, in the current-item box.
    Flow,
}

/// The pilot's own additions to and removals from Essentials, for one pack.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct EssentialsChanges {
    pub added: BTreeSet<StepId>,
    pub removed: BTreeSet<StepId>,
}

// Each flag is an independent on/off choice the pilot makes in Settings; an enum would not help.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export)]
pub struct Settings {
    pub format: u32,
    pub theme: Theme,
    /// 0 (full brightness) to [`MAX_DIMMING`].
    pub dimming: u8,
    pub text_size: TextSize,
    pub detail: Detail,
    pub list_view: ListView,
    /// Essentials only, instead of every step.
    pub essentials: bool,
    /// Pack id to the pilot's changes to Essentials.
    pub essentials_changes: BTreeMap<String, EssentialsChanges>,
    /// Settings › Panel preview in steps (round 6): the label-free panel beside each step. Show
    /// panel and Where is this? always work.
    pub panel_preview: bool,
    /// The Flight pages' Descriptions On/Off (batch 4): each field's description under its value.
    /// Off keeps the label, the value and the chip; ? keeps the longer note. Set only from the
    /// Flight header.
    pub descriptions: bool,
    pub chime: bool,
    pub simbrief_user: Option<String>,
    pub load_plan_on_open: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            format: SETTINGS_FORMAT,
            theme: Theme::default(),
            dimming: 0,
            text_size: TextSize::default(),
            detail: Detail::default(),
            list_view: ListView::default(),
            essentials: false,
            essentials_changes: BTreeMap::new(),
            panel_preview: true,
            descriptions: true,
            chime: true,
            simbrief_user: None,
            load_plan_on_open: false,
        }
    }
}

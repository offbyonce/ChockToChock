// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `controls.yaml`: zones, controls and the cockpit overview.

data_types! {
    /// An area of the cockpit.
    pub struct Zone {
        pub id: String,
        pub area: String,
        pub title: String,
        pub short_title: String,
    }

    /// A switch, lever or other control a step refers to.
    pub struct Control {
        pub id: String,
        pub name: String,
        /// Absent for controls with no place in the cockpit, such as a briefing.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zone: Option<String>,
        pub location: String,
        pub function: String,
    }

    /// A caption on the cockpit overview map.
    pub struct OverviewCaption {
        pub text: String,
        pub x: f64,
        pub y: f64,
    }

    /// A rectangle on the cockpit overview map. A zone can have several.
    pub struct OverviewArea {
        pub zone: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        /// Overrides the zone's short title on the map.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
    }

    /// A part of the cockpit on the overview map, e.g. the forward overhead panel. A map of one
    /// region shows the areas whose centers fall inside its rectangle (the first region that
    /// holds them), with its captions around the edges.
    pub struct OverviewRegion {
        pub name: String,
        /// A short reference shown after the name, e.g. "P5".
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        /// Captions for the edges, saying which way each faces.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub top: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bottom: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub left: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub right: Option<String>,
        /// Where to look from the seat, for "Where is this?".
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locate: Option<String>,
    }

    /// A schematic of the whole cockpit, in our own layout.
    pub struct Overview {
        pub width: f64,
        pub height: f64,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub captions: Vec<OverviewCaption>,
        pub areas: Vec<OverviewArea>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub regions: Vec<OverviewRegion>,
    }

    /// `controls.yaml`.
    pub struct ControlsFile {
        pub zones: Vec<Zone>,
        pub controls: Vec<Control>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub overview: Option<Overview>,
    }
}

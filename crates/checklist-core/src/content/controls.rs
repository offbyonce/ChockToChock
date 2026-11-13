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

    /// How the renderer draws a shape in a side or seat view (decision 0023). Roles name no
    /// aircraft part, so any pack can draw its own airplane.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum ViewRole {
        /// The airplane's outline (fuselage, walls): filled with the fuselage color.
        Structure,
        /// Windshield and windows.
        Glass,
        /// The pilot's seat and head.
        Seat,
        /// Where the pilot's eyes are: one point. The side view's sight line starts here.
        Eye,
    }

    /// One shape in a side or seat view: an outline through its points (a curve is given as many
    /// points), closed when `closed`. An `eye` is one point.
    pub struct ViewShape {
        pub points: Vec<[f64; 2]>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub closed: bool,
        pub role: ViewRole,
    }

    /// Which way a label runs from its point.
    #[derive(Copy, Eq, Default)]
    #[serde(rename_all = "snake_case")]
    pub enum LabelAnchor {
        Start,
        #[default]
        Middle,
        End,
    }

    /// A fixed label in a side or seat view, e.g. "WINDSHIELD" or "CABIN →".
    pub struct ViewLabel {
        pub text: String,
        pub x: f64,
        pub y: f64,
        #[serde(default)]
        pub anchor: LabelAnchor,
    }

    /// Where one overview region sits in a side or seat view: one or more closed outlines.
    pub struct ViewPlace {
        /// The region's `name` on the overview.
        pub region: String,
        pub shapes: Vec<Vec<[f64; 2]>>,
        /// Where the region's name goes when it is the one shown, if the view names it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label_at: Option<[f64; 2]>,
    }

    /// A side or seat view of the flight deck, in its own units (decision 0023).
    pub struct ViewFrame {
        pub width: f64,
        pub height: f64,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub shapes: Vec<ViewShape>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub places: Vec<ViewPlace>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub labels: Vec<ViewLabel>,
    }

    /// The side and seat views for "Where is this?".
    pub struct OverviewViews {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub side: Option<ViewFrame>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub seat: Option<ViewFrame>,
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
        /// The side and seat views (decision 0023).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub views: Option<OverviewViews>,
    }

    /// `controls.yaml`.
    pub struct ControlsFile {
        pub zones: Vec<Zone>,
        pub controls: Vec<Control>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub overview: Option<Overview>,
    }
}

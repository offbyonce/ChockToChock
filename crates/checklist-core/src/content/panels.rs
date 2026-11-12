// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `panels/*.yaml`: panel drawings.

data_types! {
    /// What a panel item is, which decides how it is drawn.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum PanelItemKind {
        ToggleSwitch,
        Selector,
        PushButton,
        Light,
        Gauge,
        Readout,
        DisplayUnit,
        /// A lever moved along a slot, such as the start, speedbrake, gear and flap levers.
        Lever,
        /// A handle that is pulled, such as the parking brake and the fire handles.
        Handle,
    }

    /// The color of a panel light.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum LightColor {
        Amber,
        Red,
        Green,
        Blue,
        White,
    }

    /// One item on a panel drawing. Positions are our own layout.
    pub struct PanelItem {
        pub kind: PanelItemKind,
        pub label: String,
        /// Required on a light, and only valid on a light.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<LightColor>,
        /// Has a protective cover. Only valid on a toggle switch or push button.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub guarded: bool,
        pub x: f64,
        pub y: f64,
        /// Ids of the controls this item belongs to.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub controls: Vec<String>,
        /// The positions it can be set to, as text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub positions: Option<String>,
        /// Where it rests when the airplane is cold and dark: one of `positions`. Drawings show it,
        /// dimmed, on controls a step does not use (decision 0015).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rest: Option<String>,
        /// Only on a light: the position, on another item it shares a control with, that turns
        /// it on, e.g. the parking brake at SET. Drawings show the light on, as expected, when a
        /// step asks for that position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lit_at: Option<String>,
        /// Only on a selector: each position's angle on the dial, in degrees clockwise from
        /// straight up, as on the panel (decision 0017). Without it, the positions spread over
        /// the top of the dial.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub angles: Option<Vec<f64>>,
        pub description: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub width: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub height: Option<f64>,
    }

    /// A labeled section of a panel.
    pub struct PanelSection {
        pub id: String,
        pub title: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        pub items: Vec<PanelItem>,
    }

    /// One `panels/*.yaml` file.
    pub struct Panel {
        pub id: String,
        pub title: String,
        pub width: f64,
        pub height: f64,
        pub sections: Vec<PanelSection>,
    }
}

// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Semantic checks that a schema cannot express, including the project's content rules
//! (decision 0007): no banned or unapproved links, no images, a basis on every list, and every
//! reference between content files resolving.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use checklist_core::{
    Alias, AliasKind, CduField, CduInline, CduPart, CduRouteCount, CduText, CduWhen, Checklist,
    FillField, MAX_ROUTE_LEG, OptionTest, PanelItem, PanelItemKind, ShowIf, Step, StepId,
};
use serde::Serialize;
use serde_json::Value;

use crate::{Pack, SCHEMA_VERSION};

/// Domains that must never be linked (non-public or redistributed manuals, and second-hand
/// study material). Matching covers subdomains too (`www.`, `en.` and so on).
pub const BLOCKED_DOMAINS: &[&str] = &[
    "brainscape.com",
    "scribd.com",
    "studylib.net",
    "manualshive.com",
    "allafricanairways.com",
    "slideshare.net",
    "archive.org",
    "aviationhunt.com",
];

/// `(domain, lower-case path prefix)` pairs that must never be linked.
pub const BLOCKED_PATHS: &[(&str, &str)] = &[("data.ntsb.gov", "/docket")];

/// First-party links the maintainer has approved, as `host` or `host/path-prefix`. Any other
/// link in a pack is an error, and a blocked link can never be listed here.
pub const ALLOWED_LINKS: &[&str] = &[];

/// File types a pack must not contain. Vendor images and screenshots never ship.
const BLOCKED_FILE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tif", "tiff", "pdf",
];

/// How a blank is written in a step's expected response.
const BLANK: &str = "___";

const ROUTE_LEG_ONLY_IN_CDU: &str =
    "`route_leg` is only valid in CDU text, with `index` and `part`";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub severity: Severity,
    /// Where: a list id, a step id, a file name or a path into the content.
    pub at: String,
    pub message: String,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.severity {
            Severity::Warning => "warning",
            Severity::Error => "error",
        };
        write!(f, "{level}: {}: {}", self.at, self.message)
    }
}

fn error(at: impl Into<String>, message: impl Into<String>) -> Issue {
    Issue {
        severity: Severity::Error,
        at: at.into(),
        message: message.into(),
    }
}

/// The lower-case host and the rest of a URL (path, query and fragment, original case).
fn split_url(url: &str) -> Option<(String, &str)> {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let rest = rest.strip_prefix("//").unwrap_or(rest);
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let (authority, path) = rest.split_at(end);
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let host = host.split(':').next()?;
    (!host.is_empty()).then(|| (host.to_ascii_lowercase(), path))
}

fn host_is(host: &str, domain: &str) -> bool {
    host == domain
        || host
            .strip_suffix(domain)
            .is_some_and(|sub| sub.ends_with('.'))
}

/// True when `path` is `prefix` or continues it at a path boundary. Ignores case.
fn path_has_prefix(path: &str, prefix: &str) -> bool {
    let path = path.to_ascii_lowercase();
    path.strip_prefix(&prefix.to_ascii_lowercase())
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(['/', '?', '#']))
}

/// True when `url` points at a banned domain or path, including subdomains.
pub fn is_blocked_link(url: &str) -> bool {
    split_url(url).is_some_and(|(host, path)| {
        BLOCKED_DOMAINS.iter().any(|d| host_is(&host, d))
            || BLOCKED_PATHS
                .iter()
                .any(|(d, prefix)| host_is(&host, d) && path_has_prefix(path, prefix))
    })
}

fn is_allowed_link(url: &str, allow: &[&str]) -> bool {
    split_url(url).is_some_and(|(host, path)| {
        allow.iter().any(|entry| match entry.split_once('/') {
            Some((h, p)) => host == *h && path_has_prefix(path, &format!("/{p}")),
            None => host == *entry,
        })
    })
}

/// Everything in `text` that is a link: anything with a scheme, anything starting `www.` or
/// `//`, and bare mentions of a banned domain.
pub fn find_links(text: &str) -> Vec<&str> {
    text.split(|c: char| c.is_whitespace() || "()[]<>\"'`".contains(c))
        .map(|t| t.trim_end_matches(['.', ',', ';', ':', '!', '?']))
        .filter(|t| {
            !t.is_empty()
                && (t.contains("://")
                    || t.to_ascii_lowercase().starts_with("www.")
                    || t.starts_with("//")
                    || is_blocked_link(t))
        })
        .collect()
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '-'))
        && !id.starts_with(['.', '-'])
        && !id.ends_with(['.', '-'])
}

/// One part of a step id: lower-case letters, digits and inner dashes.
fn valid_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !segment.starts_with('-')
        && !segment.ends_with('-')
}

/// True when `id` is `<pack-id>.<phase>.<short-name>`. The phase part records where the step
/// was first published, so it is not required to match the step's current phase.
pub fn valid_step_id(pack_id: &str, id: &str) -> bool {
    id.strip_prefix(pack_id)
        .and_then(|rest| rest.strip_prefix('.'))
        .and_then(|rest| rest.split_once('.'))
        .is_some_and(|(phase, short)| valid_segment(phase) && valid_segment(short))
}

/// Collect ids, reporting any used twice.
fn unique<'a>(
    what: &str,
    ids: impl IntoIterator<Item = &'a str>,
    issues: &mut Vec<Issue>,
) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id) {
            issues.push(error(id, format!("{what} id is used twice in this pack")));
        }
    }
    seen
}

/// The ids content files can refer to.
struct Index<'a> {
    pack_id: &'a str,
    tags: BTreeSet<&'a str>,
    lists: BTreeSet<&'a str>,
    steps: BTreeSet<&'a str>,
    blanks: BTreeMap<&'a str, usize>,
    briefing_items: BTreeSet<&'a str>,
    start_states: BTreeSet<&'a str>,
    phases: BTreeSet<&'a str>,
    options: BTreeMap<&'a str, BTreeSet<&'a str>>,
    zones: BTreeSet<&'a str>,
    controls: BTreeSet<&'a str>,
    how: BTreeSet<&'a str>,
    explain: BTreeSet<&'a str>,
    walkthroughs: BTreeSet<&'a str>,
}

impl<'a> Index<'a> {
    fn build(pack: &'a Pack, issues: &mut Vec<Issue>) -> Self {
        let c = &pack.content;
        let phases = c.phases.as_ref();
        let controls = c.controls.as_ref();
        let notes = c.notes.as_ref();
        let mut options = BTreeMap::new();
        let all_options = phases.map_or(&[][..], |p| p.options.as_slice());
        unique("option", all_options.iter().map(|o| o.id.as_str()), issues);
        for option in all_options {
            let values = unique(
                &format!("option `{}` value", option.id),
                option.values.iter().map(String::as_str),
                issues,
            );
            options.insert(option.id.as_str(), values);
        }
        Self {
            pack_id: &pack.manifest.id,
            tags: unique(
                "basis tag",
                pack.manifest.basis_tags.iter().map(String::as_str),
                issues,
            ),
            lists: unique(
                "list",
                pack.checklists.iter().map(|l| l.id.as_str()),
                issues,
            ),
            steps: unique(
                "step",
                pack.checklists
                    .iter()
                    .flat_map(|l| &l.steps)
                    .map(|s| s.id.as_str()),
                issues,
            ),
            phases: unique(
                "phase",
                phases.iter().flat_map(|p| &p.phases).map(|p| p.id.as_str()),
                issues,
            ),
            options,
            zones: unique(
                "zone",
                controls
                    .iter()
                    .flat_map(|c| &c.zones)
                    .map(|z| z.id.as_str()),
                issues,
            ),
            controls: unique(
                "control",
                controls
                    .iter()
                    .flat_map(|c| &c.controls)
                    .map(|c| c.id.as_str()),
                issues,
            ),
            how: unique(
                "how note",
                notes.iter().flat_map(|n| &n.how).map(|n| n.id.as_str()),
                issues,
            ),
            explain: unique(
                "explanation",
                notes.iter().flat_map(|n| &n.explain).map(|n| n.id.as_str()),
                issues,
            ),
            blanks: pack
                .checklists
                .iter()
                .flat_map(|l| &l.steps)
                .map(|s| (s.id.as_str(), s.expect.matches(BLANK).count()))
                .collect(),
            briefing_items: unique(
                "briefing item",
                c.briefings
                    .iter()
                    .flat_map(|f| &f.briefings)
                    .flat_map(|b| &b.items)
                    .map(|i| i.id.as_str()),
                issues,
            ),
            start_states: unique(
                "start state",
                c.start_states
                    .iter()
                    .flat_map(|f| &f.states)
                    .map(|s| s.id.as_str()),
                issues,
            ),
            walkthroughs: unique(
                "walkthrough",
                c.cdu
                    .iter()
                    .flat_map(|f| &f.walkthroughs)
                    .map(|w| w.id.as_str()),
                issues,
            ),
        }
    }

    fn require_step(&self, at: &str, id: &StepId, issues: &mut Vec<Issue>) {
        if !self.steps.contains(id.as_str()) {
            issues.push(error(at, format!("refers to unknown step `{id}`")));
        }
    }

    fn require_phase(&self, at: &str, id: &str, issues: &mut Vec<Issue>) {
        if !self.phases.contains(id) {
            issues.push(error(at, format!("refers to unknown phase `{id}`")));
        }
    }

    fn require_control(&self, at: &str, id: &str, issues: &mut Vec<Issue>) {
        if !self.controls.contains(id) {
            issues.push(error(at, format!("refers to unknown control `{id}`")));
        }
    }

    fn check_option_value(&self, at: &str, option: &str, value: &str, issues: &mut Vec<Issue>) {
        match self.options.get(option) {
            None => issues.push(error(at, format!("refers to unknown option `{option}`"))),
            Some(values) if !values.contains(value) => issues.push(error(
                at,
                format!("option `{option}` has no value `{value}`"),
            )),
            Some(_) => {}
        }
    }

    fn check_test(&self, at: &str, test: &OptionTest, issues: &mut Vec<Issue>) {
        match test {
            OptionTest::Equals { option, equals } => {
                self.check_option_value(at, option, equals, issues);
            }
            OptionTest::OneOf { option, one_of } => {
                if one_of.is_empty() {
                    issues.push(error(at, "`one_of` must list at least one value"));
                }
                for value in one_of {
                    self.check_option_value(at, option, value, issues);
                }
            }
        }
    }

    fn check_show_if(&self, at: &str, show_if: &ShowIf, issues: &mut Vec<Issue>) {
        let tests = match show_if {
            ShowIf::Test(test) => std::slice::from_ref(test),
            ShowIf::AllOf { all_of: tests } | ShowIf::AnyOf { any_of: tests } => tests.as_slice(),
        };
        if tests.is_empty() {
            issues.push(error(
                at,
                "`all_of` and `any_of` must list at least one test",
            ));
        }
        for test in tests {
            self.check_test(at, test, issues);
        }
    }
}

fn check_manifest(pack: &Pack, issues: &mut Vec<Issue>) {
    let m = &pack.manifest;
    if !valid_id(&m.id) {
        issues.push(error(
            "pack.json",
            format!(
                "pack id `{}` must be lower-case letters, digits, dots and dashes",
                m.id
            ),
        ));
    }
    if m.schema_version > SCHEMA_VERSION {
        issues.push(error(
            "pack.json",
            format!(
                "schema version {} is newer than this app supports ({SCHEMA_VERSION})",
                m.schema_version
            ),
        ));
    }
    if m.kind == crate::PackKind::Aircraft {
        if m.checklists.is_empty() {
            issues.push(error(
                "pack.json",
                "an aircraft pack must list at least one checklist",
            ));
        }
        if pack.content.phases.is_none() {
            issues.push(error(
                "pack.json",
                "an aircraft pack must list a phases file",
            ));
        }
    }
    for tag in &m.basis_tags {
        if !valid_segment(tag) {
            issues.push(error(
                "pack.json",
                format!("basis tag `{tag}` must be lower-case letters, digits and dashes"),
            ));
        }
    }
}

fn check_files(pack: &Pack, issues: &mut Vec<Issue>) {
    for file in &pack.other_files {
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if BLOCKED_FILE_EXTENSIONS.contains(&ext.as_str()) {
            issues.push(error(
                file.to_string_lossy(),
                "packs must not contain images or PDFs; vendor artwork, screenshots and manuals never ship",
            ));
        }
    }
}

fn check_step(step: &Step, idx: &Index, issues: &mut Vec<Issue>) {
    let at = step.id.as_str();
    if !valid_step_id(idx.pack_id, at) {
        issues.push(error(
            at,
            format!("step id must be `{}.<phase>.<short-name>`", idx.pack_id),
        ));
    }
    if step.text.trim().is_empty() || step.expect.trim().is_empty() {
        issues.push(error(
            at,
            "step text and expected response must not be empty",
        ));
    }
    for control in step.controls.iter().chain(step.panel_lights.keys()) {
        idx.require_control(at, control, issues);
    }
    if step.simulator_model && step.note.is_none() {
        issues.push(error(at, "`simulator_model` needs a `note` to apply to"));
    }
    let blanks = step.expect.matches(BLANK).count();
    if !step.fill.is_empty() && step.fill.len() != blanks {
        issues.push(error(
            at,
            format!(
                "`fill` has {} entries but `expect` has {blanks} blanks",
                step.fill.len()
            ),
        ));
    }
    if step.hint.iter().any(|h| h.field == FillField::Manual) {
        issues.push(error(at, "`manual` is only valid in a step's `fill`"));
    }
    let hint_fields = step.hint.iter().map(|h| &h.field);
    if step
        .fill
        .iter()
        .chain(hint_fields)
        .any(|f| *f == FillField::RouteLeg)
    {
        issues.push(error(at, ROUTE_LEG_ONLY_IN_CDU));
    }
    if let Some(id) = &step.opens
        && !idx.walkthroughs.contains(id.as_str())
    {
        issues.push(error(at, format!("opens unknown walkthrough `{id}`")));
    }
    if let Some(id) = &step.how
        && !idx.how.contains(id.as_str())
    {
        issues.push(error(at, format!("refers to unknown how note `{id}`")));
    }
    if let Some(id) = &step.explain
        && !idx.explain.contains(id.as_str())
    {
        issues.push(error(at, format!("refers to unknown explanation `{id}`")));
    }
    if let Some(show_if) = &step.show_if {
        idx.check_show_if(at, show_if, issues);
    }
    if let Some(auto) = step.timer.as_ref().and_then(|t| t.auto_start.as_ref()) {
        for id in &auto.from {
            idx.require_step(at, id, issues);
        }
    }
}

fn check_list(list: &Checklist, idx: &Index, issues: &mut Vec<Issue>) {
    if list.steps.is_empty() {
        issues.push(error(&list.id, "checklist has no steps"));
    }
    if list.basis.is_empty() {
        issues.push(error(&list.id, "list has no `basis`"));
    }
    for tag in &list.basis {
        if !idx.tags.contains(tag.as_str()) {
            issues.push(error(
                &list.id,
                format!("basis `{tag}` is not declared in pack.json"),
            ));
        }
    }
    for step in &list.steps {
        check_step(step, idx, issues);
    }
}

fn check_phases(pack: &Pack, idx: &Index, issues: &mut Vec<Issue>) {
    let Some(file) = &pack.content.phases else {
        return;
    };
    for option in &file.options {
        idx.check_option_value(&option.id, &option.id, &option.default, issues);
    }
    let mut placed = BTreeSet::new();
    for phase in &file.phases {
        for list in &phase.lists {
            if !idx.lists.contains(list.as_str()) {
                issues.push(error(&phase.id, format!("refers to unknown list `{list}`")));
            }
            if !placed.insert(list.as_str()) {
                issues.push(error(list, "list is placed in more than one phase"));
            }
        }
    }
    for list in &idx.lists {
        if !placed.contains(list) {
            issues.push(error(*list, "list is not placed in any phase"));
        }
    }
    for stage in &file.stages {
        for place in &stage.phases {
            idx.require_phase(&stage.title, &place.phase, issues);
            if let Some(show_if) = &place.show_if {
                idx.check_show_if(&stage.title, show_if, issues);
            }
        }
    }
}

fn check_panels_and_lights(pack: &Pack, idx: &Index, issues: &mut Vec<Issue>) {
    let c = &pack.content;
    if let Some(file) = &c.controls {
        for control in &file.controls {
            if let Some(zone) = &control.zone
                && !idx.zones.contains(zone.as_str())
            {
                issues.push(error(
                    &control.id,
                    format!("refers to unknown zone `{zone}`"),
                ));
            }
        }
    }
    if let Some(overview) = c.controls.as_ref().and_then(|f| f.overview.as_ref()) {
        for area in &overview.areas {
            if !idx.zones.contains(area.zone.as_str()) {
                issues.push(error(
                    "overview",
                    format!("refers to unknown zone `{}`", area.zone),
                ));
            }
        }
    }
    unique("panel", c.panels.iter().map(|p| p.id.as_str()), issues);
    let mut labels: BTreeMap<(&str, &str, &str), usize> = BTreeMap::new();
    for panel in &c.panels {
        unique(
            &format!("panel `{}` section", panel.id),
            panel.sections.iter().map(|s| s.id.as_str()),
            issues,
        );
        for section in &panel.sections {
            for item in &section.items {
                *labels
                    .entry((&panel.id, &section.id, &item.label))
                    .or_default() += 1;
                for control in &item.controls {
                    idx.require_control(&section.id, control, issues);
                }
                check_panel_item(&section.id, item, issues);
            }
        }
    }
    let Some(file) = &c.lights else {
        return;
    };
    unique(
        "light note",
        file.lights.iter().map(|l| l.locator.as_str()),
        issues,
    );
    for light in &file.lights {
        if !locator_resolves(&light.locator, &labels) {
            issues.push(error(&light.locator, "does not match any panel item"));
        }
    }
}

fn check_panel_item(at: &str, item: &PanelItem, issues: &mut Vec<Issue>) {
    let label = &item.label;
    let is_light = item.kind == PanelItemKind::Light;
    if is_light && item.color.is_none() {
        issues.push(error(at, format!("light `{label}` needs a `color`")));
    }
    if !is_light && item.color.is_some() {
        issues.push(error(at, format!("`{label}`: only a light has a `color`")));
    }
    if item.guarded
        && !matches!(
            item.kind,
            PanelItemKind::ToggleSwitch | PanelItemKind::PushButton
        )
    {
        issues.push(error(
            at,
            format!("`{label}`: only a toggle switch or push button can be `guarded`"),
        ));
    }
}

/// `<panel>/<section>/<label>/<n>`. Labels can contain slashes, so split from both ends.
fn locator_resolves(locator: &str, labels: &BTreeMap<(&str, &str, &str), usize>) -> bool {
    let mut front = locator.splitn(3, '/');
    let (Some(panel), Some(section), Some(rest)) = (front.next(), front.next(), front.next())
    else {
        return false;
    };
    let Some((label, n)) = rest.rsplit_once('/') else {
        return false;
    };
    let Ok(n) = n.parse::<usize>() else {
        return false;
    };
    n >= 1
        && labels
            .get(&(panel, section, label))
            .is_some_and(|c| n <= *c)
}

fn check_alias(alias: &Alias, idx: &Index, issues: &mut Vec<Issue>) {
    let at = alias.from.as_str();
    let to = alias.to.as_str();
    let found = |set: &BTreeSet<&str>| set.contains(to);
    let unknown = |what: &str| error(at, format!("refers to unknown {what} `{to}`"));
    match alias.kind {
        AliasKind::Step if !found(&idx.steps) => issues.push(unknown("step")),
        AliasKind::BriefingItem if !found(&idx.briefing_items) => {
            issues.push(unknown("briefing item"));
        }
        AliasKind::StartState if !found(&idx.start_states) => issues.push(unknown("start state")),
        AliasKind::List if !found(&idx.lists) => issues.push(unknown("list")),
        AliasKind::Phase if !found(&idx.phases) => issues.push(unknown("phase")),
        AliasKind::Zone if !found(&idx.zones) => issues.push(unknown("zone")),
        AliasKind::Blank => {
            let blank = to.rsplit_once('#').and_then(|(step, n)| {
                let n = n.parse::<usize>().ok()?;
                (n < *idx.blanks.get(step)?).then_some(())
            });
            if blank.is_none() {
                issues.push(unknown("blank"));
            }
        }
        AliasKind::OptionValue => match to.split_once(':') {
            Some((option, value)) => idx.check_option_value(at, option, value, issues),
            None => issues.push(error(at, "option alias must be `<option>:<value>`")),
        },
        AliasKind::FillField
            if !matches!(
                serde_json::from_value::<FillField>(Value::String(to.to_owned())),
                Ok(f) if f != FillField::Manual && f != FillField::RouteLeg
            ) =>
        {
            issues.push(unknown("fill field"));
        }
        _ => {}
    }
}

fn check_other_references(pack: &Pack, idx: &Index, issues: &mut Vec<Issue>) {
    let c = &pack.content;
    if let Some(file) = &c.faults {
        for fault in &file.faults {
            for step in &fault.steps {
                idx.require_step("faults", step, issues);
            }
        }
    }
    if let Some(file) = &c.glossary {
        unique(
            "glossary term",
            file.terms.iter().map(|t| t.term.as_str()),
            issues,
        );
    }
    if let Some(file) = &c.briefings {
        unique(
            "briefing",
            file.briefings.iter().map(|b| b.id.as_str()),
            issues,
        );
        for briefing in &file.briefings {
            idx.require_phase(&briefing.id, &briefing.phase, issues);
        }
    }
    if let Some(file) = &c.start_states {
        for state in &file.states {
            idx.require_phase(&state.id, &state.phase, issues);
            for (option, value) in &state.set {
                idx.check_option_value(&state.id, option, value, issues);
            }
            for step in state.done.iter().chain(&state.skip) {
                idx.require_step(&state.id, step, issues);
            }
        }
    }
    if let Some(file) = &c.aliases {
        let mut seen = BTreeSet::new();
        for alias in &file.aliases {
            if !seen.insert((alias.kind, alias.from.as_str())) {
                issues.push(error(&alias.from, "alias id is used twice in this pack"));
            }
            check_alias(alias, idx, issues);
        }
    }
}

fn check_cdu_field(at: &str, field: &CduField, issues: &mut Vec<Issue>) {
    if field.field == FillField::Manual {
        issues.push(error(at, "`manual` is only valid in a step's `fill`"));
    }
    if field.field == FillField::RouteLeg {
        if field.part.is_none() || field.index.is_none() {
            issues.push(error(at, "`route_leg` needs an `index` and a `part`"));
        }
        if field
            .index
            .is_some_and(|n| !(1..=MAX_ROUTE_LEG).contains(&n))
        {
            issues.push(error(
                at,
                format!("`route_leg` index must be from 1 to {MAX_ROUTE_LEG}"),
            ));
        }
    } else if field.index.is_some() || field.part.is_some() {
        issues.push(error(at, "only `route_leg` takes an `index` or `part`"));
    }
}

fn check_route_count(at: &str, count: &CduRouteCount, issues: &mut Vec<Issue>) {
    if count.per_page == 0 {
        issues.push(error(at, "`per_page` must be at least 1"));
    }
}

fn check_cdu_inline(at: &str, part: &CduInline, issues: &mut Vec<Issue>) {
    match part {
        CduInline::Field(field) => check_cdu_field(at, field, issues),
        CduInline::RouteCount(count) => check_route_count(at, count, issues),
        CduInline::Text(_) | CduInline::Styled(_) => {}
    }
}

fn check_cdu_when(at: &str, when: &CduWhen, issues: &mut Vec<Issue>) {
    if when.when_known.is_empty() {
        issues.push(error(at, "`when_known` must name a field"));
    }
    if when.when_known.contains(&FillField::Manual) {
        issues.push(error(at, "`manual` is only valid in a step's `fill`"));
    }
    if when.when_known.contains(&FillField::RouteLeg) {
        issues.push(error(
            at,
            "`when_known` cannot test `route_leg`; test `route`",
        ));
    }
    for part in when.show.iter().chain(&when.otherwise) {
        check_cdu_inline(at, part, issues);
    }
}

fn check_cdu_text(at: &str, text: &CduText, issues: &mut Vec<Issue>) {
    for part in text {
        match part {
            CduPart::Field(field) => check_cdu_field(at, field, issues),
            CduPart::RouteCount(count) => check_route_count(at, count, issues),
            CduPart::When(when) => check_cdu_when(at, when, issues),
            CduPart::Text(_) | CduPart::Styled(_) => {}
        }
    }
}

fn check_cdu(pack: &Pack, issues: &mut Vec<Issue>) {
    let Some(file) = &pack.content.cdu else {
        return;
    };
    for walkthrough in &file.walkthroughs {
        for page in &walkthrough.pages {
            let at = format!("{} page `{}`", walkthrough.id, page.title);
            let screen = &page.screen;
            let cells = screen
                .rows
                .iter()
                .flat_map(|r| [&r.left_label, &r.left, &r.right_label, &r.right]);
            let texts = [
                &page.action,
                &page.check,
                &page.note,
                &screen.title,
                &screen.page,
                &screen.scratchpad,
            ];
            for text in texts.into_iter().chain(cells) {
                check_cdu_text(&at, text, issues);
            }
        }
    }
}

/// Check one link: banned links are always errors; others need to be on the allow-list.
fn link_issue(at: &str, link: &str, allow: &[&str]) -> Option<Issue> {
    if is_blocked_link(link) {
        Some(error(at, format!("link `{link}` is on the blocked list")))
    } else if is_allowed_link(link, allow) {
        None
    } else {
        Some(error(
            at,
            format!("link `{link}` is not allowed; packs carry no third-party URLs"),
        ))
    }
}

fn scan_value(at: &str, value: &Value, allow: &[&str], issues: &mut Vec<Issue>) {
    match value {
        Value::String(text) => {
            issues.extend(
                find_links(text)
                    .into_iter()
                    .filter_map(|l| link_issue(at, l, allow)),
            );
        }
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                scan_value(&format!("{at}/{i}"), item, allow, issues);
            }
        }
        Value::Object(fields) => {
            for (key, item) in fields {
                scan_value(&format!("{at}/{key}"), item, allow, issues);
            }
        }
        _ => {}
    }
}

fn scan_links(at: &str, data: &impl Serialize, allow: &[&str], issues: &mut Vec<Issue>) {
    match serde_json::to_value(data) {
        Ok(value) => scan_value(at, &value, allow, issues),
        Err(e) => issues.push(error(at, format!("cannot scan for links: {e}"))),
    }
}

fn check_links(pack: &Pack, allow: &[&str], issues: &mut Vec<Issue>) {
    scan_links("pack.json", &pack.manifest, allow, issues);
    for list in &pack.checklists {
        scan_links(&format!("list {}", list.id), list, allow, issues);
    }
    scan_links("content", &pack.content, allow, issues);
}

/// Run every semantic check. An empty list, or warnings only, means the pack may ship.
pub fn validate_pack(pack: &Pack) -> Vec<Issue> {
    validate_with_allow_list(pack, ALLOWED_LINKS)
}

fn validate_with_allow_list(pack: &Pack, allow: &[&str]) -> Vec<Issue> {
    let mut issues = Vec::new();
    check_manifest(pack, &mut issues);
    check_files(pack, &mut issues);
    let idx = Index::build(pack, &mut issues);
    for list in &pack.checklists {
        check_list(list, &idx, &mut issues);
    }
    check_phases(pack, &idx, &mut issues);
    check_panels_and_lights(pack, &idx, &mut issues);
    check_other_references(pack, &idx, &mut issues);
    check_cdu(pack, &mut issues);
    check_links(pack, allow, &mut issues);
    issues.sort_by(|a, b| b.severity.cmp(&a.severity).then_with(|| a.at.cmp(&b.at)));
    issues
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use checklist_core::{
        Alias, AliasKind, AliasesFile, BriefItem, Briefing, BriefingsFile, CduField, CduFile,
        CduPage, CduScreen, CduWalkthrough, CduWhen, Control, ControlsFile, HintPart, LightColor,
        LightNote, LightsFile, ListKind, Overview, OverviewArea, Panel, PanelItem, PanelItemKind,
        PanelSection, Phase, PhasesFile, RouteLegCount, RouteLegPart, Stage, StagePhase,
        StartState, StartStatesFile, Status, UserOption, Zone,
    };
    use proptest::prelude::*;

    use super::*;
    use crate::PackContent;
    use crate::manifest::{PackKind, PackManifest};

    const PACK: &str = "demo-aircraft";

    fn manifest() -> PackManifest {
        PackManifest {
            json_schema: None,
            id: PACK.into(),
            name: "Demo".into(),
            version: semver::Version::new(0, 1, 0),
            schema_version: SCHEMA_VERSION,
            kind: PackKind::Aircraft,
            engine: semver::VersionReq::parse(">=0.1").unwrap(),
            dependencies: std::collections::BTreeMap::default(),
            license: "CC-BY-NC-SA-4.0".into(),
            attribution: vec![],
            basis_tags: vec!["sim-observed".into()],
            checklists: vec!["preflight.yaml".into()],
            phases: Some("phases.yaml".into()),
            controls: None,
            notes: None,
            panels: vec![],
            lights: None,
            faults: None,
            glossary: None,
            briefings: None,
            start_states: None,
            cdu: None,
            aliases: None,
        }
    }

    fn step(short: &str) -> Step {
        Step::new(format!("{PACK}.preflight.{short}"), "Battery", "ON")
    }

    fn phases() -> PhasesFile {
        PhasesFile {
            options: vec![UserOption {
                id: "power".into(),
                title: "Power".into(),
                values: vec!["gpu".into(), "apu".into()],
                default: "gpu".into(),
            }],
            stages: vec![Stage {
                title: "Preflight".into(),
                phases: vec![StagePhase {
                    phase: "preflight".into(),
                    show_if: None,
                }],
            }],
            phases: vec![Phase {
                id: "preflight".into(),
                title: "Preflight".into(),
                reference: false,
                intro: vec![],
                lists: vec!["preflight".into()],
            }],
        }
    }

    fn pack(steps: Vec<Step>, other_files: Vec<&str>) -> Pack {
        Pack {
            manifest: manifest(),
            checklists: vec![Checklist {
                id: "preflight".into(),
                title: "Preflight".into(),
                kind: ListKind::Checklist,
                basis: vec!["sim-observed".into()],
                memory: false,
                when: None,
                why: None,
                steps,
            }],
            content: PackContent {
                phases: Some(phases()),
                ..PackContent::default()
            },
            other_files: other_files.into_iter().map(PathBuf::from).collect(),
        }
    }

    fn messages(p: &Pack) -> String {
        validate_pack(p)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn assert_flags(p: &Pack, expected: &str) {
        let joined = messages(p);
        assert!(
            joined.contains(expected),
            "wanted `{expected}` in:\n{joined}"
        );
    }

    #[test]
    fn clean_pack_has_no_issues() {
        assert_eq!(
            validate_pack(&pack(vec![step("a")], vec![])),
            Vec::<Issue>::new()
        );
    }

    #[test]
    fn a_step_needs_no_source() {
        // Decision 0007 dropped the per-step source rule.
        assert_eq!(messages(&pack(vec![step("a")], vec![])), "");
    }

    #[test]
    fn blocked_domains_match_subdomains_only() {
        assert!(is_blocked_link("https://www.brainscape.com/flashcards/x"));
        assert!(is_blocked_link("http://ARCHIVE.org/details/fcom"));
        assert!(is_blocked_link("https://aviationhunt.com/b737"));
        assert!(!is_blocked_link("https://notbrainscape.com/x"));
        assert!(!is_blocked_link("https://example.com/brainscape.com"));
    }

    #[test]
    fn docket_paths_are_blocked_by_prefix_ignoring_case() {
        assert!(is_blocked_link(
            "https://data.ntsb.gov/Docket/Document/docBLOB?ID=1"
        ));
        assert!(is_blocked_link("https://data.ntsb.gov/docket"));
        assert!(is_blocked_link("data.ntsb.gov/DOCKET?x=1"));
        assert!(!is_blocked_link("https://data.ntsb.gov/carol-main/"));
        assert!(!is_blocked_link("https://data.ntsb.gov/docketing"));
        assert!(!is_blocked_link("https://www.ntsb.gov/Docket"));
    }

    #[test]
    fn links_are_found_in_prose() {
        assert_eq!(
            find_links(
                "See (https://a.example/x), www.b.example. Or //c.example and scribd.com/doc."
            ),
            vec![
                "https://a.example/x",
                "www.b.example",
                "//c.example",
                "scribd.com/doc"
            ]
        );
        assert_eq!(
            find_links("Set FL350/M.78, V1 142. N1 85.5 and A/T ON"),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn links_are_found_in_any_text_field() {
        let mut banned_note = step("a");
        banned_note.note = Some("From https://www.scribd.com/doc/1".into());
        assert_flags(&pack(vec![banned_note], vec![]), "blocked list");

        let mut other_link = step("a");
        other_link.check = Some("Details at https://example.com/page".into());
        assert_flags(&pack(vec![other_link], vec![]), "not allowed");

        let mut p = pack(vec![step("a")], vec![]);
        p.manifest.attribution = vec!["See brainscape.com/x".into()];
        assert_flags(&p, "pack.json/attribution/0");

        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.stages[0].title = "See https://example.com".into();
        }
        assert_flags(&p, "content/phases/stages/0/title");
    }

    #[test]
    fn the_allow_list_starts_empty_and_cannot_override_a_ban() {
        assert_eq!(ALLOWED_LINKS, Vec::<&str>::new());
        let mut s = step("a");
        s.note = Some("https://first.example/docs/x and https://first.example/other".into());
        let p = pack(vec![s], vec![]);
        let issues = validate_with_allow_list(&p, &["first.example/docs"]);
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert!(issues[0].message.contains("first.example/other"));

        let mut s = step("a");
        s.note = Some("https://scribd.com/x".into());
        let issues = validate_with_allow_list(&pack(vec![s], vec![]), &["scribd.com"]);
        assert!(issues[0].message.contains("blocked list"), "{issues:?}");
    }

    #[test]
    fn content_rules_are_enforced() {
        let joined = messages(&pack(
            vec![step("a"), step("a")],
            vec!["cockpit.png", "manual.pdf", "notes.md"],
        ));
        assert!(joined.contains("used twice"), "{joined}");
        assert!(joined.contains("cockpit.png"), "{joined}");
        assert!(joined.contains("manual.pdf"), "{joined}");
        assert!(!joined.contains("notes.md"), "{joined}");
    }

    #[test]
    fn newer_schema_is_rejected() {
        let mut p = pack(vec![step("a")], vec![]);
        p.manifest.schema_version = SCHEMA_VERSION + 1;
        assert_flags(&p, "newer than this app");
    }

    #[test]
    fn lists_need_a_declared_basis() {
        let mut p = pack(vec![step("a")], vec![]);
        p.checklists[0].basis.clear();
        assert_flags(&p, "no `basis`");

        let mut p = pack(vec![step("a")], vec![]);
        p.checklists[0].basis = vec!["some-manual".into()];
        assert_flags(&p, "not declared");

        let mut p = pack(vec![step("a")], vec![]);
        p.manifest.basis_tags = vec!["Some Manual".into()];
        assert_flags(&p, "must be lower-case");
    }

    #[test]
    fn step_ids_follow_the_format() {
        for bad in [
            "demo-aircraft.a",
            "other.preflight.a",
            "demo-aircraft.preflight.a.b",
            "demo-aircraft.preflight.A",
            "demo-aircraft..a",
        ] {
            let p = pack(vec![Step::new(bad, "T", "E")], vec![]);
            assert_flags(&p, "step id must be");
        }
    }

    #[test]
    fn a_moved_step_keeps_its_id() {
        // The phase part records first publication, so it need not name a current phase.
        let p = pack(
            vec![Step::new(format!("{PACK}.old-phase.a"), "T", "E")],
            vec![],
        );
        assert_eq!(messages(&p), "");
    }

    #[test]
    fn library_pack_ids_may_contain_dots() {
        assert!(valid_step_id("lib.shared-cdu", "lib.shared-cdu.perf.fuel"));
    }

    #[test]
    fn show_if_must_use_declared_options_and_values() {
        for (show_if, expected) in [
            (
                ShowIf::Test(OptionTest::Equals {
                    option: "nope".into(),
                    equals: "gpu".into(),
                }),
                "unknown option",
            ),
            (
                ShowIf::Test(OptionTest::OneOf {
                    option: "power".into(),
                    one_of: vec!["nope".into()],
                }),
                "has no value",
            ),
            (
                ShowIf::Test(OptionTest::OneOf {
                    option: "power".into(),
                    one_of: vec![],
                }),
                "at least one value",
            ),
            (ShowIf::AllOf { all_of: vec![] }, "at least one test"),
            (ShowIf::AnyOf { any_of: vec![] }, "at least one test"),
        ] {
            let mut s = step("a");
            s.show_if = Some(show_if);
            assert_flags(&pack(vec![s], vec![]), expected);
        }
    }

    #[test]
    fn stage_places_are_checked_too() {
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.stages[0].phases[0].show_if = Some(ShowIf::Test(OptionTest::Equals {
                option: "power".into(),
                equals: "nope".into(),
            }));
        }
        assert_flags(&p, "has no value");
    }

    #[test]
    fn fill_must_match_the_blanks() {
        let mut s = step("a");
        s.expect = "V1 ___ VR ___".into();
        s.fill = vec![FillField::V1];
        assert_flags(
            &pack(vec![s.clone()], vec![]),
            "1 entries but `expect` has 2 blanks",
        );
        s.fill = vec![FillField::V1, FillField::Manual];
        assert_eq!(messages(&pack(vec![s], vec![])), "");
    }

    #[test]
    fn simulator_model_applies_to_a_note() {
        let mut s = step("a");
        s.simulator_model = true;
        assert_flags(&pack(vec![s.clone()], vec![]), "needs a `note`");
        s.note = Some("The guard closes by itself.".into());
        assert_eq!(messages(&pack(vec![s], vec![])), "");
    }

    #[test]
    fn manual_is_only_for_step_blanks() {
        let mut s = step("a");
        s.hint = vec![HintPart {
            prefix: "X ".into(),
            field: FillField::Manual,
        }];
        assert_flags(&pack(vec![s], vec![]), "only valid in a step's `fill`");

        let mut p = pack(vec![step("a")], vec![]);
        p.content.cdu = Some(CduFile {
            walkthroughs: vec![CduWalkthrough {
                id: "preflight".into(),
                title: "Preflight".into(),
                pages: vec![CduPage {
                    page: "IDENT".into(),
                    key: None,
                    title: "Check".into(),
                    action: vec![CduPart::Field(CduField {
                        field: FillField::Manual,
                        style: None,
                        before: None,
                        after: None,
                        empty: None,
                        empty_style: None,
                        index: None,
                        part: None,
                    })],
                    check: vec![],
                    note: vec![],
                    screen: CduScreen {
                        title: vec![],
                        page: vec![],
                        rows: vec![],
                        highlight: vec![],
                        scratchpad: vec![],
                    },
                }],
            }],
        });
        assert_flags(&p, "only valid in a step's `fill`");
    }

    #[test]
    fn step_references_must_resolve() {
        let mut s = step("a");
        s.controls = vec!["nope".into()];
        assert_flags(&pack(vec![s], vec![]), "unknown control");

        let mut s = step("a");
        s.opens = Some("nope".into());
        assert_flags(&pack(vec![s], vec![]), "unknown walkthrough");

        let mut s = step("a");
        s.how = Some("nope".into());
        assert_flags(&pack(vec![s], vec![]), "unknown how note");

        let mut s = step("a");
        s.explain = Some("nope".into());
        assert_flags(&pack(vec![s], vec![]), "unknown explanation");
    }

    #[test]
    fn every_list_sits_in_exactly_one_phase() {
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.phases[0].lists.clear();
        }
        assert_flags(&p, "not placed in any phase");

        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.phases[0].lists = vec!["preflight".into(), "preflight".into(), "ghost".into()];
        }
        assert_flags(&p, "more than one phase");
        assert_flags(&p, "unknown list");
    }

    #[test]
    fn an_aircraft_pack_needs_phases() {
        let mut p = pack(vec![step("a")], vec![]);
        p.content.phases = None;
        assert_flags(&p, "phases file");
    }

    fn panel_pack() -> Pack {
        let mut p = pack(vec![step("a")], vec![]);
        p.content.controls = Some(ControlsFile {
            zones: vec![],
            controls: vec![Control {
                id: "bat".into(),
                name: "Battery".into(),
                zone: None,
                location: "Overhead".into(),
                function: "Connects the battery".into(),
            }],
            overview: None,
        });
        p.content.panels = vec![Panel {
            id: "oh".into(),
            title: "Overhead".into(),
            width: 100.0,
            height: 100.0,
            sections: vec![PanelSection {
                id: "elec".into(),
                title: "Electrical".into(),
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
                items: vec![PanelItem {
                    kind: PanelItemKind::Light,
                    label: "A/B LOW".into(),
                    color: Some(LightColor::Amber),
                    guarded: false,
                    x: 1.5,
                    y: 2.0,
                    controls: vec!["bat".into()],
                    positions: None,
                    description: "Low".into(),
                    width: None,
                    height: None,
                }],
            }],
        }];
        p
    }

    fn light(locator: &str) -> LightsFile {
        LightsFile {
            lights: vec![LightNote {
                locator: locator.into(),
                meaning: "M".into(),
                when: "W".into(),
                action: "A".into(),
                normal: false,
                status: Some(Status::Draft),
            }],
        }
    }

    #[test]
    fn light_locators_resolve_even_with_slashes_in_labels() {
        let mut p = panel_pack();
        p.content.lights = Some(light("oh/elec/A/B LOW/1"));
        assert_eq!(messages(&p), "");
        for bad in [
            "oh/elec/A/B LOW/2",
            "oh/elec/A/B LOW/0",
            "oh/nope/X/1",
            "oh/elec",
        ] {
            let mut p = panel_pack();
            p.content.lights = Some(light(bad));
            assert_flags(&p, "does not match any panel item");
        }
    }

    #[test]
    fn overview_areas_must_name_known_zones() {
        let mut p = alias_pack(vec![]);
        let area = |zone: &str| OverviewArea {
            zone: zone.into(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            label: None,
        };
        if let Some(file) = &mut p.content.controls {
            file.overview = Some(Overview {
                width: 100.0,
                height: 100.0,
                captions: vec![],
                areas: vec![area("overhead"), area("overhead"), area("ghost")],
            });
        }
        let joined = messages(&p);
        assert!(joined.contains("unknown zone `ghost`"), "{joined}");
        assert!(!joined.contains("`overhead`"), "{joined}");
    }

    #[test]
    fn panel_items_must_name_known_controls() {
        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0]
            .controls
            .push("nope".into());
        assert_flags(&p, "unknown control");
    }

    #[test]
    fn lights_need_a_color_and_only_lights_have_one() {
        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0].color = None;
        assert_flags(&p, "needs a `color`");

        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0].kind = PanelItemKind::Gauge;
        assert_flags(&p, "only a light has a `color`");
    }

    #[test]
    fn only_switches_and_buttons_can_be_guarded() {
        for kind in [PanelItemKind::ToggleSwitch, PanelItemKind::PushButton] {
            let mut p = panel_pack();
            let item = &mut p.content.panels[0].sections[0].items[0];
            item.kind = kind;
            item.color = None;
            item.guarded = true;
            assert_eq!(messages(&p), "");
        }
        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0].guarded = true;
        assert_flags(&p, "can be `guarded`");
    }

    #[test]
    fn route_leg_needs_a_valid_index_and_part() {
        let leg = |index, part| {
            let mut f = field(FillField::RouteLeg);
            f.index = index;
            f.part = part;
            cdu_pack(vec![CduPart::Field(f)])
        };
        assert_eq!(messages(&leg(Some(1), Some(RouteLegPart::Via))), "");
        assert_eq!(
            messages(&leg(Some(MAX_ROUTE_LEG), Some(RouteLegPart::To))),
            ""
        );
        assert_flags(&leg(None, Some(RouteLegPart::To)), "needs an `index`");
        assert_flags(&leg(Some(1), None), "needs an `index`");
        assert_flags(&leg(Some(0), Some(RouteLegPart::To)), "from 1 to 30");
        assert_flags(&leg(Some(31), Some(RouteLegPart::To)), "from 1 to 30");
    }

    #[test]
    fn route_leg_stays_in_cdu_text() {
        let mut f = field(FillField::Runway);
        f.index = Some(1);
        assert_flags(&cdu_pack(vec![CduPart::Field(f)]), "only `route_leg` takes");

        let mut s = step("a");
        s.expect = "___".into();
        s.fill = vec![FillField::RouteLeg];
        assert_flags(&pack(vec![s], vec![]), "only valid in CDU text");

        let mut s = step("a");
        s.hint = vec![HintPart {
            prefix: "X ".into(),
            field: FillField::RouteLeg,
        }];
        assert_flags(&pack(vec![s], vec![]), "only valid in CDU text");

        let when = CduWhen {
            when_known: vec![FillField::RouteLeg],
            show: vec![CduInline::Text("x".into())],
            otherwise: vec![],
        };
        assert_flags(
            &cdu_pack(vec![CduPart::When(when)]),
            "cannot test `route_leg`",
        );

        let p = alias_pack(vec![alias(AliasKind::FillField, "x", "route_leg")]);
        assert_flags(&p, "unknown fill field");
    }

    #[test]
    fn route_counts_need_a_page_size() {
        let count = |per_page| {
            cdu_pack(vec![CduPart::RouteCount(CduRouteCount {
                route_legs: RouteLegCount::Pages,
                per_page,
                style: None,
                before: None,
                after: None,
                empty: None,
            })])
        };
        assert_eq!(messages(&count(5)), "");
        assert_flags(&count(0), "`per_page` must be at least 1");
    }

    #[test]
    fn an_option_default_must_be_one_of_its_values() {
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.options[0].default = "solar".into();
        }
        assert_flags(&p, "has no value `solar`");
    }

    fn cdu_pack(action: CduText) -> Pack {
        let mut p = pack(vec![step("a")], vec![]);
        p.content.cdu = Some(CduFile {
            walkthroughs: vec![CduWalkthrough {
                id: "setup".into(),
                title: "Setup".into(),
                pages: vec![CduPage {
                    page: "RTE".into(),
                    key: None,
                    title: "Route".into(),
                    action,
                    check: vec![],
                    note: vec![],
                    screen: CduScreen {
                        title: vec![],
                        page: vec![],
                        rows: vec![],
                        highlight: vec![],
                        scratchpad: vec![],
                    },
                }],
            }],
        });
        p
    }

    fn field(field: FillField) -> CduField {
        CduField {
            field,
            style: None,
            before: None,
            after: None,
            empty: None,
            empty_style: None,
            index: None,
            part: None,
        }
    }

    #[test]
    fn cdu_groups_are_checked() {
        let good = CduWhen {
            when_known: vec![FillField::Runway],
            show: vec![
                CduInline::Text("<SEL> ".into()),
                CduInline::Field(field(FillField::Runway)),
            ],
            otherwise: vec![],
        };
        assert_eq!(messages(&cdu_pack(vec![CduPart::When(good.clone())])), "");

        let mut empty = good.clone();
        empty.when_known.clear();
        assert_flags(
            &cdu_pack(vec![CduPart::When(empty)]),
            "`when_known` must name",
        );

        let mut manual_shown = good.clone();
        manual_shown
            .show
            .push(CduInline::Field(field(FillField::Manual)));
        assert_flags(&cdu_pack(vec![CduPart::When(manual_shown)]), "`manual`");

        let mut manual_tested = good;
        manual_tested.when_known = vec![FillField::Manual];
        assert_flags(&cdu_pack(vec![CduPart::When(manual_tested)]), "`manual`");
    }

    fn alias(kind: AliasKind, from: &str, to: &str) -> Alias {
        Alias {
            kind,
            from: from.into(),
            to: to.into(),
        }
    }

    fn alias_pack(aliases: Vec<Alias>) -> Pack {
        let mut s = step("a");
        s.expect = "V1 ___ VR ___".into();
        let mut p = pack(vec![s], vec![]);
        p.content.controls = Some(ControlsFile {
            zones: vec![Zone {
                id: "overhead".into(),
                area: "Overhead".into(),
                title: "Overhead".into(),
                short_title: "OH".into(),
            }],
            controls: vec![],
            overview: None,
        });
        p.content.briefings = Some(BriefingsFile {
            briefings: vec![Briefing {
                id: "departure".into(),
                title: "Departure".into(),
                mnemonic: "R".into(),
                phase: "preflight".into(),
                items: vec![BriefItem {
                    id: "departure.runway".into(),
                    lead: "R".into(),
                    rest: "UNWAY".into(),
                    detail: None,
                }],
            }],
        });
        p.content.start_states = Some(StartStatesFile {
            states: vec![StartState {
                id: "ready".into(),
                title: "Ready".into(),
                description: "Ready".into(),
                set: std::collections::BTreeMap::default(),
                phase: "preflight".into(),
                done: vec![],
                skip: vec![],
            }],
        });
        p.content.aliases = Some(AliasesFile { aliases });
        p
    }

    #[test]
    fn aliases_of_every_kind_resolve() {
        let step_id = format!("{PACK}.preflight.a");
        let blank = format!("{step_id}#1");
        let p = alias_pack(vec![
            alias(AliasKind::Step, "preflight-0-0", &step_id),
            alias(AliasKind::BriefingItem, "b-0-0", "departure.runway"),
            alias(AliasKind::Blank, "preflight-0-0-v1", &blank),
            alias(AliasKind::StartState, "bes", "ready"),
            alias(AliasKind::OptionValue, "power:ground", "power:gpu"),
            alias(AliasKind::FillField, "toflaps", "takeoff_flaps"),
            alias(AliasKind::List, "m:Smoke", "preflight"),
            alias(AliasKind::Phase, "f:preflight", "preflight"),
            alias(AliasKind::Zone, "c:Overhead", "overhead"),
        ]);
        assert_eq!(messages(&p), "");
    }

    #[test]
    fn aliases_with_unknown_targets_are_errors() {
        let step_id = format!("{PACK}.preflight.a");
        let past_end = format!("{step_id}#2");
        for (a, expected) in [
            (alias(AliasKind::Step, "x", "ghost"), "unknown step"),
            (
                alias(AliasKind::BriefingItem, "x", "ghost"),
                "unknown briefing item",
            ),
            (alias(AliasKind::Blank, "x", &past_end), "unknown blank"),
            (alias(AliasKind::Blank, "x", "ghost#0"), "unknown blank"),
            (
                alias(AliasKind::StartState, "x", "ghost"),
                "unknown start state",
            ),
            (
                alias(AliasKind::OptionValue, "x", "power:solar"),
                "has no value",
            ),
            (
                alias(AliasKind::OptionValue, "x", "power"),
                "must be `<option>:<value>`",
            ),
            (
                alias(AliasKind::FillField, "x", "ghost"),
                "unknown fill field",
            ),
            (alias(AliasKind::List, "x", "ghost"), "unknown list"),
            (alias(AliasKind::Phase, "x", "ghost"), "unknown phase"),
            (alias(AliasKind::Zone, "x", "ghost"), "unknown zone"),
        ] {
            assert_flags(&alias_pack(vec![a]), expected);
        }
    }

    #[test]
    fn the_same_old_id_may_appear_once_per_kind() {
        let step_id = format!("{PACK}.preflight.a");
        let p = alias_pack(vec![
            alias(AliasKind::Step, "preflight", &step_id),
            alias(AliasKind::Phase, "preflight", "preflight"),
        ]);
        assert_eq!(messages(&p), "");

        let p = alias_pack(vec![
            alias(AliasKind::Step, "preflight", &step_id),
            alias(AliasKind::Step, "preflight", &step_id),
        ]);
        assert_flags(&p, "alias id is used twice");
    }

    proptest! {
        /// Link scanning returns a result, never a panic, on arbitrary text.
        #[test]
        fn find_links_never_panics(input in ".{0,300}") {
            let _ = find_links(&input);
        }

        /// A banned link is caught whatever subdomain or path it uses, and however it is
        /// embedded in a sentence.
        #[test]
        fn banned_links_are_always_found(
            sub in "[a-z]{1,8}",
            path in "[a-zA-Z0-9/_-]{0,20}",
            idx in 0..BLOCKED_DOMAINS.len(),
        ) {
            let url = format!("https://{sub}.{}/{path}", BLOCKED_DOMAINS[idx]);
            let text = format!("See ({url}), then continue.");
            let links = find_links(&text);
            prop_assert_eq!(links.len(), 1);
            prop_assert!(is_blocked_link(links[0]));
        }

        /// Well-formed ids pass, and adding a level or changing the pack fails.
        #[test]
        fn step_id_format(phase in "[a-z0-9]{1,10}", short in "[a-z0-9]{1,10}") {
            let id = format!("demo.{phase}.{short}");
            prop_assert!(valid_step_id("demo", &id));
            let extra = format!("{id}.x");
            prop_assert!(!valid_step_id("demo", &extra));
            prop_assert!(!valid_step_id("other", &id));
        }
    }
}

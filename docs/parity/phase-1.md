<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Phase 1 parity report

Every step in the 3.0 checklist, and where it is in `packs/ifly-737-max8`.

## Summary

- **3.0 steps:** 510. **Moved:** 510. **Merged:** 0. **Dropped:** 0.
- **Moved to a different list:** 3, to match the order in the vendor documentation.
- **Wording or setting changed during validation:** 58.
- **Added during validation:** 16, listed below. **Steps in the pack now:** 526.

## Other content

- **Fault causes:** 24 in 3.0, 22 in the pack. The other 2 matched no step, so 3.0 never showed them.
- **Section headings:** all 4 carried over as the `section` field on the step that follows.
- **Sources:** the 3.0 reference list (44 entries) is not carried over (decision 0007).
- **Step fields `add` and `a`:** dropped; they were a build marker and an id-numbering aid. **`alt`:** not carried over, since no step sets it (decision 0007).
- **Practice text and the guided tour:** not in this pack. Practice text is Phase 3; the tour is Phase 2.
- **Practice scores saved per cockpit area:** no alias yet (Phase 3).

## Lists reordered during validation

Steps in these lists now follow the vendor order:

- `after-landing-after-landing-procedure`
- `engine-start-engine-start`
- `takeoff-line-up`

## Text rewritten on import

These passages were reworded in the converter because the overlap check flagged them. Steps are also marked in the table below.

| Item | Field |
| --- | --- |
| `step ifly-737-max8.procedures.80-knots-to-v1` | `expect` |
| `step ifly-737-max8.procedures.below-80-knots` | `expect` |
| `step ifly-737-max8.quick-reference.descent` | `expect` |
| `step ifly-737-max8.quick-reference.thrust-levers` | `expect` |
| `step ifly-737-max8.quick-reference.if-the-engine-fire-switch-or-eng-overheat-light` | `text` |
| `step ifly-737-max8.procedures.stab-out-of-trim-light-on-the-ground` | `text` |
| `step ifly-737-max8.procedures.attitude-terrain-escape` | `expect` |
| `step ifly-737-max8.memory.outflow-valve-switch` | `expect` |
| `step ifly-737-max8.memory.if-cabin-altitude-is-controllable` | `expect` |
| `step ifly-737-max8.preflight.c-r-switch` | `expect` |
| `step ifly-737-max8.approach.multi-mode-receiver` | `expect` |
| `step ifly-737-max8.before-start.fuel` | `expect` |
| `list memory-airspeed-unreliable` | `when` |
| `control dispsw` | `function` |
| `control motor` | `function` |
| `briefing item takeoff-briefing-items-gate-stand.atis-weather` | `detail` |
| `briefing item approach-briefing.atis-weather` | `detail` |
| `briefing item takeoff-briefing-items-gate-stand.taxi-route` | `detail` |
| `briefing item approach-briefing.taxi-route` | `detail` |
| `briefing item takeoff-briefing-items-gate-stand.clearance` | `detail` |
| `briefing item approach-briefing.clearance` | `detail` |
| `briefing item takeoff-briefing-items-gate-stand.threats-and-abnormals` | `detail` |
| `briefing item approach-briefing.threats-and-abnormals` | `detail` |
| `briefing item takeoff-briefing-taxi-out.runway` | `detail` |
| `briefing item takeoff-briefing-taxi-out.departure` | `detail` |
| `briefing item takeoff-briefing-taxi-out.initial-altitude` | `detail` |

## Words changed to American spelling

| 3.0 | Pack | Times |
| --- | --- | --- |
| centre | center | 4 |
| labelled | labeled | 1 |

## Steps added during validation

| Step id | List |
| --- | --- |
| `ifly-737-max8.after-landing.transponder` | `after-landing-after-landing-procedure` |
| `ifly-737-max8.apu-power.fire-switch-and-overheat-detector` | `apu-power-establishing-apu-power` |
| `ifly-737-max8.before-start.landing-gear-pins` | `before-start-before-start-procedure` |
| `ifly-737-max8.climb.center-fuel-pumps` | `climb-climb-procedure-above-10-000-ft` |
| `ifly-737-max8.engine-start.ground-support-items` | `engine-start-engine-start` |
| `ifly-737-max8.engine-start.secondary-engine-indications` | `engine-start-engine-start` |
| `ifly-737-max8.landing-roll.autopilot-and-autothrottle` | `landing-roll-landing-roll` |
| `ifly-737-max8.landing.mcp-speed-and-altitude` | `landing-before-landing-procedure` |
| `ifly-737-max8.memory.approach-and-go-around-airspeed-unreliable` | `memory-airspeed-unreliable` |
| `ifly-737-max8.preflight.fuel-valve-and-filter-lights` | `preflight-preliminary-preflight-procedure` |
| `ifly-737-max8.preflight.ground-proximity-panel` | `preflight-preliminary-preflight-procedure` |
| `ifly-737-max8.preflight.primary-engine-indications` | `preflight-preliminary-preflight-procedure` |
| `ifly-737-max8.preflight.takeoff-config-and-cabin-altitude-lights` | `preflight-preliminary-preflight-procedure` |
| `ifly-737-max8.pushback.towing-pin-inserted` | `pushback-pushback` |
| `ifly-737-max8.pushback.towing-pin-removed` | `pushback-pushback` |
| `ifly-737-max8.taxi.taxi-thrust` | `taxi-taxi` |

## Every 3.0 step

| 3.0 id | New id | Status | Reason |
| --- | --- | --- | --- |
| `cold-dark-0-0` | `ifly-737-max8.cold-dark.battery-switch` | moved |  |
| `cold-dark-0-1` | `ifly-737-max8.cold-dark.standby-power-switch` | moved |  |
| `cold-dark-0-2` | `ifly-737-max8.cold-dark.alternate-flaps-switches` | moved | Wording or setting changed during validation |
| `cold-dark-0-3` | `ifly-737-max8.cold-dark.wiper-selectors` | moved |  |
| `cold-dark-0-4` | `ifly-737-max8.cold-dark.hydraulic-pump-switches` | moved | Wording or setting changed during validation |
| `cold-dark-0-5` | `ifly-737-max8.cold-dark.landing-gear-lever` | moved |  |
| `gpu-power-0-0` | `ifly-737-max8.gpu-power.parking-brake` | moved |  |
| `gpu-power-0-1` | `ifly-737-max8.gpu-power.ground-power-cart-efb` | moved |  |
| `gpu-power-0-2` | `ifly-737-max8.gpu-power.ground-power-switch` | moved |  |
| `apu-power-0-0` | `ifly-737-max8.apu-power.fire-test-switch` | moved |  |
| `apu-power-0-f2` | `ifly-737-max8.apu-power.fire-test-switch-establishing-apu` | moved |  |
| `apu-power-0-f3` | `ifly-737-max8.apu-power.extinguisher-test-switch` | moved | Wording or setting changed during validation |
| `apu-power-0-p1` | `ifly-737-max8.apu-power.left-forward-fuel-pump` | moved | Wording or setting changed during validation |
| `apu-power-0-1` | `ifly-737-max8.apu-power.apu-switch` | moved |  |
| `apu-power-0-2` | `ifly-737-max8.apu-power.apu-generator-switches` | moved |  |
| `apu-power-0-p2` | `ifly-737-max8.apu-power.left-forward-fuel-pump-establishing-apu` | moved | Wording or setting changed during validation |
| `apu-power-0-3` | `ifly-737-max8.apu-power.ground-power-cart-efb` | moved |  |
| `apu-power-0-4` | `ifly-737-max8.apu-power.apu-bleed-switch` | moved |  |
| `apu-power-0-5` | `ifly-737-max8.apu-power.pack-switches` | moved |  |
| `apu-power-0-6` | `ifly-737-max8.apu-power.isolation-valve-switch` | moved |  |
| `apu-power-0-7` | `ifly-737-max8.apu-power.pack-lights` | moved |  |
| `exterior-0-0` | `ifly-737-max8.exterior.left-forward-fuselage` | moved |  |
| `exterior-0-1` | `ifly-737-max8.exterior.nose` | moved |  |
| `exterior-0-2` | `ifly-737-max8.exterior.right-forward-fuselage` | moved |  |
| `exterior-0-3` | `ifly-737-max8.exterior.right-wing-root` | moved |  |
| `exterior-0-4` | `ifly-737-max8.exterior.engine-2` | moved |  |
| `exterior-0-5` | `ifly-737-max8.exterior.right-wing-and-leading-edge` | moved |  |
| `exterior-0-6` | `ifly-737-max8.exterior.right-wing-tip` | moved |  |
| `exterior-0-7` | `ifly-737-max8.exterior.right-trailing-edge` | moved |  |
| `exterior-0-8` | `ifly-737-max8.exterior.right-main-gear` | moved |  |
| `exterior-0-9` | `ifly-737-max8.exterior.right-aft-fuselage` | moved |  |
| `exterior-0-10` | `ifly-737-max8.exterior.tail` | moved |  |
| `exterior-0-11` | `ifly-737-max8.exterior.left-aft-fuselage` | moved |  |
| `exterior-0-12` | `ifly-737-max8.exterior.left-main-gear` | moved |  |
| `exterior-0-13` | `ifly-737-max8.exterior.left-trailing-edge` | moved |  |
| `exterior-0-14` | `ifly-737-max8.exterior.left-wing-tip` | moved |  |
| `exterior-0-15` | `ifly-737-max8.exterior.left-wing-and-leading-edge` | moved |  |
| `exterior-0-16` | `ifly-737-max8.exterior.engine-1` | moved |  |
| `exterior-0-17` | `ifly-737-max8.exterior.left-wing-root` | moved |  |
| `preflight-0-l1` | `ifly-737-max8.preflight.panel-and-flood-lights` | moved |  |
| `preflight-0-0` | `ifly-737-max8.preflight.irs` | moved |  |
| `preflight-0-1` | `ifly-737-max8.preflight.elt-switch` | moved |  |
| `preflight-0-2` | `ifly-737-max8.preflight.elevator-jam-landing-assist` | moved |  |
| `preflight-0-3` | `ifly-737-max8.preflight.gps-ils-gls-and-maint-lights` | moved |  |
| `preflight-0-4` | `ifly-737-max8.preflight.eec-switches` | moved | Wording or setting changed during validation |
| `preflight-0-5` | `ifly-737-max8.preflight.passenger-oxygen-switch` | moved |  |
| `preflight-0-6` | `ifly-737-max8.preflight.crew-oxygen-pressure` | moved |  |
| `preflight-0-7` | `ifly-737-max8.preflight.landing-gear-indicator-lights-ovhd` | moved |  |
| `preflight-0-8` | `ifly-737-max8.preflight.flight-recorder-switch` | moved |  |
| `preflight-0-9` | `ifly-737-max8.preflight.mach-airspeed-warning-test` | moved |  |
| `preflight-0-10` | `ifly-737-max8.preflight.stall-warning-test` | moved |  |
| `preflight-0-11` | `ifly-737-max8.preflight.manual-gear-extension-access-door` | moved |  |
| `preflight-0-12` | `ifly-737-max8.preflight.cdu` | moved |  |
| `preflight-0-13` | `ifly-737-max8.preflight.flight-control-switches` | moved |  |
| `preflight-0-14` | `ifly-737-max8.preflight.spoiler-switches` | moved |  |
| `preflight-0-15` | `ifly-737-max8.preflight.yaw-damper-switch` | moved |  |
| `preflight-0-16` | `ifly-737-max8.preflight.standby-hydraulic-lights` | moved |  |
| `preflight-0-17` | `ifly-737-max8.preflight.alternate-flaps-switches` | moved | Wording or setting changed during validation |
| `preflight-0-18` | `ifly-737-max8.preflight.feel-diff-press-speed-trim-fail-mach-trim-fail` | moved |  |
| `preflight-0-19` | `ifly-737-max8.preflight.navigation-transfer-switches` | moved |  |
| `preflight-0-20` | `ifly-737-max8.preflight.display-source-switches` | moved |  |
| `preflight-0-21` | `ifly-737-max8.preflight.crossfeed-selector` | moved |  |
| `preflight-0-22` | `ifly-737-max8.preflight.fuel-pump-switches` | moved |  |
| `preflight-0-p1` | `ifly-737-max8.preflight.fuel-pump-switches-preliminary-prefligh` | moved | Wording or setting changed during validation |
| `preflight-0-23` | `ifly-737-max8.preflight.cab-util-and-ife-pass-seat-switches` | moved |  |
| `preflight-0-24` | `ifly-737-max8.preflight.bat-discharge-tr-unit-and-elec-lights` | moved |  |
| `preflight-0-25` | `ifly-737-max8.preflight.generator-drive-disconnect-switches` | moved |  |
| `preflight-0-26` | `ifly-737-max8.preflight.bus-transfer-switch` | moved |  |
| `preflight-0-27` | `ifly-737-max8.preflight.engine-gen-off-bus-lights` | moved |  |
| `preflight-0-28` | `ifly-737-max8.preflight.lavatory-smoke-light` | moved |  |
| `preflight-0-29` | `ifly-737-max8.preflight.equipment-cooling-switches` | moved |  |
| `preflight-0-30` | `ifly-737-max8.preflight.emergency-exit-lights-switch` | moved |  |
| `preflight-0-31` | `ifly-737-max8.preflight.passenger-signs` | moved | Wording or setting changed during validation |
| `preflight-0-32` | `ifly-737-max8.preflight.window-heat-switches` | moved |  |
| `preflight-0-33` | `ifly-737-max8.preflight.probe-heat` | moved |  |
| `preflight-0-34` | `ifly-737-max8.preflight.engine-and-wing-anti-ice-switches` | moved |  |
| `preflight-0-35` | `ifly-737-max8.preflight.electric-hydraulic-pump-switches` | moved | Wording or setting changed during validation |
| `preflight-0-a0` | `ifly-737-max8.preflight.high-alt-landing-switch` | moved | Wording or setting changed during validation |
| `preflight-0-36` | `ifly-737-max8.preflight.voice-recorder-switch` | moved |  |
| `preflight-0-37` | `ifly-737-max8.preflight.trim-air-switch` | moved |  |
| `preflight-0-38` | `ifly-737-max8.preflight.temperature-selectors` | moved |  |
| `preflight-0-39` | `ifly-737-max8.preflight.recirc-fan-switches` | moved |  |
| `preflight-0-40` | `ifly-737-max8.preflight.pack-switches` | moved |  |
| `preflight-0-41` | `ifly-737-max8.preflight.isolation-valve-switch` | moved |  |
| `preflight-0-42` | `ifly-737-max8.preflight.apu-bleed-switch` | moved |  |
| `preflight-0-43` | `ifly-737-max8.preflight.engine-bleed-switches` | moved |  |
| `preflight-0-44` | `ifly-737-max8.preflight.pack-lights` | moved |  |
| `preflight-0-45` | `ifly-737-max8.preflight.cabin-pressurization-panel` | moved |  |
| `preflight-0-46` | `ifly-737-max8.preflight.landing-lights-runway-turnoff-and-taxi-lights` | moved |  |
| `preflight-0-47` | `ifly-737-max8.preflight.engine-start-switches` | moved | Wording or setting changed during validation |
| `preflight-0-48` | `ifly-737-max8.preflight.logo-light` | moved |  |
| `preflight-0-49` | `ifly-737-max8.preflight.position-light` | moved |  |
| `preflight-0-50` | `ifly-737-max8.preflight.anti-collision-light-wing-light-and-wheel-well` | moved |  |
| `preflight-0-51` | `ifly-737-max8.preflight.flight-director-switches` | moved |  |
| `preflight-0-52` | `ifly-737-max8.preflight.bank-angle-selector` | moved |  |
| `preflight-0-53` | `ifly-737-max8.preflight.autopilot-disengage-bar` | moved |  |
| `preflight-0-54` | `ifly-737-max8.preflight.efis-control-panels` | moved |  |
| `preflight-0-55` | `ifly-737-max8.preflight.lights-test-autopilot-and-general` | moved |  |
| `preflight-0-56` | `ifly-737-max8.preflight.stab-out-of-trim-light` | moved |  |
| `preflight-0-57` | `ifly-737-max8.preflight.flight-instruments-pfd-and-nd` | moved |  |
| `preflight-0-58` | `ifly-737-max8.preflight.pfd-mfd-selectors` | moved |  |
| `preflight-0-59` | `ifly-737-max8.preflight.gpws-sys-test` | moved |  |
| `preflight-0-60` | `ifly-737-max8.preflight.integrated-standby-flight-display` | moved |  |
| `preflight-0-61` | `ifly-737-max8.preflight.landing-gear-lever` | moved | Wording or setting changed during validation |
| `preflight-0-62` | `ifly-737-max8.preflight.nose-wheel-steering-switch` | moved |  |
| `preflight-0-63` | `ifly-737-max8.preflight.fuel-flow-switch` | moved |  |
| `preflight-0-64` | `ifly-737-max8.preflight.autobrake-selector` | moved |  |
| `preflight-0-65` | `ifly-737-max8.preflight.c-r-switch` | moved | `expect` rewritten in our own words on import |
| `preflight-0-66` | `ifly-737-max8.preflight.display-selectors-l-c-r` | moved |  |
| `preflight-0-67` | `ifly-737-max8.preflight.speedbrake-lever` | moved |  |
| `preflight-0-68` | `ifly-737-max8.preflight.thrust-levers` | moved |  |
| `preflight-0-69` | `ifly-737-max8.preflight.flap-lever` | moved |  |
| `preflight-0-70` | `ifly-737-max8.preflight.parking-brake` | moved |  |
| `preflight-0-71` | `ifly-737-max8.preflight.engine-start-levers` | moved |  |
| `preflight-0-72` | `ifly-737-max8.preflight.stabilizer-trim-cutout-switches` | moved |  |
| `preflight-0-73` | `ifly-737-max8.preflight.cargo-fire-panel` | moved |  |
| `preflight-0-74` | `ifly-737-max8.preflight.radio-tuning-panels` | moved |  |
| `preflight-0-75` | `ifly-737-max8.preflight.audio-control-panel` | moved |  |
| `preflight-0-76` | `ifly-737-max8.preflight.multi-mode-receiver` | moved |  |
| `preflight-0-77` | `ifly-737-max8.preflight.adf-selector-panel` | moved |  |
| `preflight-0-78` | `ifly-737-max8.preflight.weather-radar-panel` | moved |  |
| `preflight-0-79` | `ifly-737-max8.preflight.transponder` | moved |  |
| `preflight-0-80` | `ifly-737-max8.preflight.stabilizer-trim-override-switch` | moved |  |
| `preflight-1-0` | `ifly-737-max8.preflight.oxygen` | moved |  |
| `preflight-1-1` | `ifly-737-max8.preflight.navigation-transfer-and-display-switches` | moved |  |
| `preflight-1-2` | `ifly-737-max8.preflight.window-heat` | moved |  |
| `preflight-1-3` | `ifly-737-max8.preflight.pressurization-mode-selector` | moved |  |
| `preflight-1-4` | `ifly-737-max8.preflight.flight-instruments` | moved |  |
| `preflight-1-5` | `ifly-737-max8.preflight.parking-brake-check` | moved |  |
| `preflight-1-6` | `ifly-737-max8.preflight.engine-start-levers-check` | moved |  |
| `before-start-0-0` | `ifly-737-max8.before-start.n1-bugs` | moved | Wording or setting changed during validation |
| `before-start-0-1` | `ifly-737-max8.before-start.mcp` | moved | Wording or setting changed during validation |
| `before-start-0-2` | `ifly-737-max8.before-start.door-lights` | moved |  |
| `before-start-0-3` | `ifly-737-max8.before-start.terr-efis-control-panel` | moved |  |
| `before-start-0-4` | `ifly-737-max8.before-start.fuel-pump-switches` | moved | Wording or setting changed during validation |
| `before-start-0-a0` | `ifly-737-max8.before-start.electric-hydraulic-pump-switches` | moved |  |
| `before-start-0-5` | `ifly-737-max8.before-start.anti-collision-light` | moved |  |
| `before-start-0-6` | `ifly-737-max8.before-start.stabilizer-trim` | moved |  |
| `before-start-1-0` | `ifly-737-max8.before-start.flight-deck-door` | moved |  |
| `before-start-1-1` | `ifly-737-max8.before-start.fuel` | moved | `expect` rewritten in our own words on import |
| `before-start-1-2` | `ifly-737-max8.before-start.passenger-signs` | moved | Wording or setting changed during validation |
| `before-start-1-3` | `ifly-737-max8.before-start.flight-deck-windows` | moved |  |
| `before-start-1-4` | `ifly-737-max8.before-start.mcp-check` | moved |  |
| `before-start-1-5` | `ifly-737-max8.before-start.takeoff-speeds` | moved |  |
| `before-start-1-6` | `ifly-737-max8.before-start.cdu-preflight` | moved |  |
| `before-start-1-7` | `ifly-737-max8.before-start.rudder-and-aileron-trim` | moved |  |
| `before-start-1-8` | `ifly-737-max8.before-start.taxi-and-takeoff-briefing` | moved |  |
| `before-start-1-9` | `ifly-737-max8.before-start.anti-collision-light-check` | moved |  |
| `pushback-0-0` | `ifly-737-max8.pushback.ice-check` | moved |  |
| `pushback-0-1` | `ifly-737-max8.pushback.maintenance-log` | moved |  |
| `pushback-0-2` | `ifly-737-max8.pushback.tcas-and-transponder` | moved |  |
| `pushback-0-3` | `ifly-737-max8.pushback.pushback-clearance` | moved |  |
| `pushback-0-c1` | `ifly-737-max8.pushback.wheel-chocks` | moved |  |
| `pushback-0-4` | `ifly-737-max8.pushback.parking-brake` | moved |  |
| `pushback-0-5` | `ifly-737-max8.pushback.pushback` | moved |  |
| `pushback-0-6` | `ifly-737-max8.pushback.parking-brake-pushback` | moved |  |
| `engine-start-0-0` | `ifly-737-max8.engine-start.start-clearance` | moved |  |
| `engine-start-0-1` | `ifly-737-max8.engine-start.pack-switches` | moved |  |
| `engine-start-0-2` | `ifly-737-max8.engine-start.apu-bleed-switch` | moved |  |
| `engine-start-0-3` | `ifly-737-max8.engine-start.ignition-selector` | moved | Wording or setting changed during validation |
| `engine-start-0-4` | `ifly-737-max8.engine-start.engine-2-start-switch` | moved |  |
| `engine-start-0-5` | `ifly-737-max8.engine-start.motoring-indication` | moved |  |
| `engine-start-0-6` | `ifly-737-max8.engine-start.engine-2-start-lever` | moved | Wording or setting changed during validation |
| `engine-start-0-7` | `ifly-737-max8.engine-start.fuel-flow-and-egt` | moved |  |
| `engine-start-0-8` | `ifly-737-max8.engine-start.engine-2-start-switch-engine-start` | moved | Wording or setting changed during validation |
| `engine-start-0-9` | `ifly-737-max8.engine-start.engine-2` | moved |  |
| `engine-start-0-10` | `ifly-737-max8.engine-start.engine-1-start-switch` | moved |  |
| `engine-start-0-11` | `ifly-737-max8.engine-start.motoring-indication-engine-start` | moved |  |
| `engine-start-0-12` | `ifly-737-max8.engine-start.engine-1-start-lever` | moved | Wording or setting changed during validation |
| `engine-start-0-13` | `ifly-737-max8.engine-start.fuel-flow-and-egt-engine-start` | moved |  |
| `engine-start-0-14` | `ifly-737-max8.engine-start.engine-1-start-switch-engine-start` | moved | Wording or setting changed during validation |
| `engine-start-0-15` | `ifly-737-max8.engine-start.engine-1` | moved |  |
| `before-taxi-0-0` | `ifly-737-max8.before-taxi.engine-generator-switches` | moved |  |
| `before-taxi-0-1` | `ifly-737-max8.before-taxi.probe-heat` | moved | Wording or setting changed during validation |
| `before-taxi-0-2` | `ifly-737-max8.before-taxi.anti-ice` | moved |  |
| `before-taxi-0-3` | `ifly-737-max8.before-taxi.pack-switches` | moved |  |
| `before-taxi-0-4` | `ifly-737-max8.before-taxi.isolation-valve-switch` | moved |  |
| `before-taxi-0-5` | `ifly-737-max8.before-taxi.engine-bleed-switches` | moved |  |
| `before-taxi-0-6` | `ifly-737-max8.before-taxi.engine-start-switches` | moved |  |
| `before-taxi-0-7` | `ifly-737-max8.before-taxi.apu-bleed-switch` | moved |  |
| `before-taxi-0-8` | `ifly-737-max8.before-taxi.apu-switch` | moved |  |
| `before-taxi-0-9` | `ifly-737-max8.before-taxi.flap-lever` | moved |  |
| `before-taxi-0-10` | `ifly-737-max8.before-taxi.sys-switch` | moved |  |
| `before-taxi-0-11` | `ifly-737-max8.before-taxi.flight-controls` | moved |  |
| `before-taxi-0-12` | `ifly-737-max8.before-taxi.recall` | moved | Wording or setting changed during validation |
| `before-taxi-1-0` | `ifly-737-max8.before-taxi.generators` | moved |  |
| `before-taxi-1-1` | `ifly-737-max8.before-taxi.probe-heat-check` | moved | Wording or setting changed during validation |
| `before-taxi-1-2` | `ifly-737-max8.before-taxi.anti-ice-check` | moved |  |
| `before-taxi-1-3` | `ifly-737-max8.before-taxi.isolation-valve-switch-check` | moved |  |
| `before-taxi-1-4` | `ifly-737-max8.before-taxi.engine-start-switches-check` | moved |  |
| `before-taxi-1-5` | `ifly-737-max8.before-taxi.recall-check` | moved |  |
| `before-taxi-1-6` | `ifly-737-max8.before-taxi.autobrake` | moved |  |
| `before-taxi-1-7` | `ifly-737-max8.before-taxi.engine-start-levers` | moved |  |
| `before-taxi-1-8` | `ifly-737-max8.before-taxi.flight-controls-check` | moved |  |
| `before-taxi-1-9` | `ifly-737-max8.before-taxi.ground-equipment` | moved | Wording or setting changed during validation |
| `taxi-0-0` | `ifly-737-max8.taxi.taxi-clearance` | moved |  |
| `taxi-0-1` | `ifly-737-max8.taxi.taxi-and-runway-turnoff-lights` | moved |  |
| `taxi-0-w1` | `ifly-737-max8.taxi.wing-light` | moved |  |
| `taxi-0-2` | `ifly-737-max8.taxi.transponder` | moved | Moved to list `before-taxi-before-taxi-procedure` to match vendor order |
| `taxi-0-3` | `ifly-737-max8.taxi.parking-brake` | moved |  |
| `before-takeoff-0-0` | `ifly-737-max8.before-takeoff.oil-temperature` | moved |  |
| `before-takeoff-0-1` | `ifly-737-max8.before-takeoff.efis-control-panels` | moved | Wording or setting changed during validation |
| `before-takeoff-0-pk` | `ifly-737-max8.before-takeoff.pack-switches` | moved |  |
| `before-takeoff-1-0` | `ifly-737-max8.before-takeoff.flaps` | moved |  |
| `before-takeoff-1-1` | `ifly-737-max8.before-takeoff.takeoff-data` | moved |  |
| `before-takeoff-1-2` | `ifly-737-max8.before-takeoff.v2` | moved |  |
| `before-takeoff-1-3` | `ifly-737-max8.before-takeoff.stabilizer-trim` | moved |  |
| `takeoff-0-0` | `ifly-737-max8.takeoff.engine-warm-up` | moved |  |
| `takeoff-0-1` | `ifly-737-max8.takeoff.runway-and-entry-point` | moved |  |
| `takeoff-0-2` | `ifly-737-max8.takeoff.engine-start-switches` | moved |  |
| `takeoff-0-3` | `ifly-737-max8.takeoff.position-light` | moved |  |
| `takeoff-0-4` | `ifly-737-max8.takeoff.transponder` | moved |  |
| `takeoff-0-5` | `ifly-737-max8.takeoff.heading` | moved |  |
| `takeoff-0-6` | `ifly-737-max8.takeoff.takeoff-clearance` | moved |  |
| `takeoff-0-7` | `ifly-737-max8.takeoff.landing-lights` | moved |  |
| `takeoff-1-0` | `ifly-737-max8.takeoff.thrust-levers` | moved |  |
| `takeoff-1-1` | `ifly-737-max8.takeoff.to-ga` | moved |  |
| `takeoff-1-2` | `ifly-737-max8.takeoff.takeoff-thrust` | moved | Wording or setting changed during validation |
| `takeoff-1-3` | `ifly-737-max8.takeoff.80-knots` | moved |  |
| `takeoff-1-4` | `ifly-737-max8.takeoff.v1` | moved |  |
| `takeoff-1-5` | `ifly-737-max8.takeoff.vr` | moved |  |
| `takeoff-1-6` | `ifly-737-max8.takeoff.positive-rate` | moved |  |
| `after-takeoff-0-0` | `ifly-737-max8.after-takeoff.landing-gear-lever` | moved |  |
| `after-takeoff-0-1` | `ifly-737-max8.after-takeoff.flap-lever` | moved |  |
| `after-takeoff-0-2` | `ifly-737-max8.after-takeoff.pack-switches` | moved |  |
| `after-takeoff-1-0` | `ifly-737-max8.after-takeoff.engine-bleeds` | moved |  |
| `after-takeoff-1-1` | `ifly-737-max8.after-takeoff.pack-switches-check` | moved |  |
| `after-takeoff-1-2` | `ifly-737-max8.after-takeoff.landing-gear` | moved |  |
| `after-takeoff-1-3` | `ifly-737-max8.after-takeoff.flaps` | moved |  |
| `after-takeoff-2-0` | `ifly-737-max8.after-takeoff.roll-mode` | moved |  |
| `after-takeoff-2-1` | `ifly-737-max8.after-takeoff.pressurization` | moved |  |
| `after-takeoff-2-2` | `ifly-737-max8.after-takeoff.climb-thrust` | moved |  |
| `after-takeoff-2-3` | `ifly-737-max8.after-takeoff.autopilot` | moved | Wording or setting changed during validation |
| `after-takeoff-2-4` | `ifly-737-max8.after-takeoff.autobrake` | moved |  |
| `after-takeoff-2-5` | `ifly-737-max8.after-takeoff.engine-start-switches` | moved |  |
| `climb-0-0` | `ifly-737-max8.climb.landing-lights` | moved |  |
| `climb-0-1` | `ifly-737-max8.climb.wing-logo-and-runway-turnoff-lights` | moved |  |
| `climb-0-2` | `ifly-737-max8.climb.fasten-belts-selector` | moved |  |
| `climb-1-0` | `ifly-737-max8.climb.exterior-lights` | moved |  |
| `climb-1-1` | `ifly-737-max8.climb.fasten-belts-selector-check` | moved |  |
| `climb-1-2` | `ifly-737-max8.climb.altimeters` | moved |  |
| `cruise-0-0` | `ifly-737-max8.cruise.cruise-altitude` | moved |  |
| `cruise-0-1` | `ifly-737-max8.cruise.fuel` | moved | Wording or setting changed during validation |
| `cruise-0-2` | `ifly-737-max8.cruise.center-fuel-pumps` | moved |  |
| `cruise-0-3` | `ifly-737-max8.cruise.anti-ice` | moved |  |
| `cruise-0-4` | `ifly-737-max8.cruise.weather` | moved |  |
| `cruise-0-5` | `ifly-737-max8.cruise.approach-preparation` | moved |  |
| `cruise-0-6` | `ifly-737-max8.cruise.descent-and-arrival-data` | moved |  |
| `cruise-0-7` | `ifly-737-max8.cruise.mcp-altitude` | moved | Wording or setting changed during validation |
| `descent-0-0` | `ifly-737-max8.descent.fmc-route` | moved |  |
| `descent-0-1` | `ifly-737-max8.descent.land-alt` | moved |  |
| `descent-0-2` | `ifly-737-max8.descent.cdu` | moved |  |
| `descent-0-3` | `ifly-737-max8.descent.recall` | moved |  |
| `descent-0-4` | `ifly-737-max8.descent.minimums` | moved |  |
| `descent-0-5` | `ifly-737-max8.descent.autobrake` | moved |  |
| `descent-1-0` | `ifly-737-max8.descent.pressurization` | moved |  |
| `descent-1-1` | `ifly-737-max8.descent.recall-check` | moved |  |
| `descent-1-2` | `ifly-737-max8.descent.autobrake-check` | moved |  |
| `descent-1-3` | `ifly-737-max8.descent.landing-data` | moved |  |
| `descent-1-4` | `ifly-737-max8.descent.approach-briefing` | moved |  |
| `approach-0-0` | `ifly-737-max8.approach.altimeters` | moved |  |
| `approach-0-1` | `ifly-737-max8.approach.landing-lights-wing-lights-logo-light` | moved |  |
| `approach-0-2` | `ifly-737-max8.approach.fasten-belts-selector` | moved | Wording or setting changed during validation |
| `approach-0-3` | `ifly-737-max8.approach.multi-mode-receiver` | moved | `expect` rewritten in our own words on import |
| `approach-0-a0` | `ifly-737-max8.approach.approach-in-the-cdu` | moved |  |
| `approach-0-a1` | `ifly-737-max8.approach.engine-start-switches` | moved | Moved to list `landing-before-landing-procedure` to match vendor order |
| `approach-0-4` | `ifly-737-max8.approach.course-selectors` | moved |  |
| `approach-0-a2` | `ifly-737-max8.approach.app-mcp` | moved |  |
| `approach-1-0` | `ifly-737-max8.approach.altimeters-check` | moved |  |
| `approach-1-1` | `ifly-737-max8.approach.fasten-belts-selector-check` | moved | Wording or setting changed during validation |
| `approach-1-2` | `ifly-737-max8.approach.exterior-lights` | moved |  |
| `landing-0-0` | `ifly-737-max8.landing.flap-lever` | moved |  |
| `landing-0-1` | `ifly-737-max8.landing.landing-gear-lever` | moved | Wording or setting changed during validation |
| `landing-0-3` | `ifly-737-max8.landing.speedbrake-lever` | moved |  |
| `landing-0-2` | `ifly-737-max8.landing.landing-flaps` | moved | Wording or setting changed during validation |
| `landing-0-a0` | `ifly-737-max8.landing.landing-gear-lever-before-landing` | moved |  |
| `landing-0-a5` | `ifly-737-max8.landing.speedbrake-lever-before-landing` | moved |  |
| `landing-0-a1` | `ifly-737-max8.landing.landing-flaps-before-landing` | moved |  |
| `landing-0-a2` | `ifly-737-max8.landing.landing-gear-lever-before-landing-2` | moved |  |
| `landing-0-a6` | `ifly-737-max8.landing.speedbrake-lever-before-landing-2` | moved |  |
| `landing-0-a3` | `ifly-737-max8.landing.landing-flaps-before-landing-2` | moved |  |
| `landing-0-4` | `ifly-737-max8.landing.landing-clearance` | moved |  |
| `landing-0-5` | `ifly-737-max8.landing.taxi-light` | moved |  |
| `landing-0-a4` | `ifly-737-max8.landing.autopilot-and-autothrottle` | moved | Wording or setting changed during validation |
| `landing-1-3` | `ifly-737-max8.landing.engine-start-switches` | moved |  |
| `landing-1-2` | `ifly-737-max8.landing.speedbrake-lever-check` | moved |  |
| `landing-1-1` | `ifly-737-max8.landing.gear` | moved |  |
| `landing-1-0` | `ifly-737-max8.landing.flaps` | moved |  |
| `landing-1-4` | `ifly-737-max8.landing.landing-clearance-check` | moved |  |
| `landing-roll-0-0` | `ifly-737-max8.landing-roll.thrust-levers` | moved |  |
| `landing-roll-0-1` | `ifly-737-max8.landing-roll.speedbrake` | moved |  |
| `landing-roll-0-2` | `ifly-737-max8.landing-roll.reverse-thrust` | moved |  |
| `landing-roll-0-3` | `ifly-737-max8.landing-roll.rev-indications` | moved |  |
| `landing-roll-0-4` | `ifly-737-max8.landing-roll.60-knots` | moved | Wording or setting changed during validation |
| `landing-roll-0-5` | `ifly-737-max8.landing-roll.reverse-levers` | moved |  |
| `landing-roll-0-6` | `ifly-737-max8.landing-roll.autobrake` | moved |  |
| `after-landing-0-0` | `ifly-737-max8.after-landing.autobrake` | moved |  |
| `after-landing-0-1` | `ifly-737-max8.after-landing.flaps` | moved |  |
| `after-landing-0-2` | `ifly-737-max8.after-landing.speedbrake-lever` | moved |  |
| `after-landing-0-3` | `ifly-737-max8.after-landing.probe-heat` | moved |  |
| `after-landing-0-4` | `ifly-737-max8.after-landing.engine-start-switches` | moved |  |
| `after-landing-0-5` | `ifly-737-max8.after-landing.efis` | moved |  |
| `after-landing-0-6` | `ifly-737-max8.after-landing.position-light` | moved |  |
| `after-landing-0-7` | `ifly-737-max8.after-landing.wing-light-landing-light-and-turnoff-lights` | moved |  |
| `after-landing-0-8` | `ifly-737-max8.after-landing.apu-switch` | moved |  |
| `after-landing-1-0` | `ifly-737-max8.after-landing.weather-radar-and-terrain` | moved |  |
| `after-landing-1-1` | `ifly-737-max8.after-landing.autobrake-check` | moved |  |
| `after-landing-1-2` | `ifly-737-max8.after-landing.flaps-check` | moved |  |
| `after-landing-1-3` | `ifly-737-max8.after-landing.speedbrake-lever-check` | moved |  |
| `after-landing-1-4` | `ifly-737-max8.after-landing.window-heat` | moved |  |
| `after-landing-1-5` | `ifly-737-max8.after-landing.probe-heat-check` | moved |  |
| `after-landing-1-6` | `ifly-737-max8.after-landing.engine-start-switches-check` | moved |  |
| `after-landing-1-7` | `ifly-737-max8.after-landing.apu-switch-check` | moved |  |
| `taxi-in-0-0` | `ifly-737-max8.taxi-in.taxi-clearance` | moved |  |
| `taxi-in-0-1` | `ifly-737-max8.taxi-in.transponder` | moved | Moved to list `shutdown-shutdown-procedure` to match vendor order; Wording or setting changed during validation |
| `taxi-in-0-2` | `ifly-737-max8.taxi-in.engine-cool-down` | moved |  |
| `shutdown-0-0` | `ifly-737-max8.shutdown.apu-generator-switches` | moved |  |
| `shutdown-0-1` | `ifly-737-max8.shutdown.parking-brake` | moved |  |
| `shutdown-0-2` | `ifly-737-max8.shutdown.engine-start-levers` | moved |  |
| `shutdown-0-3` | `ifly-737-max8.shutdown.fasten-belts-selector` | moved |  |
| `shutdown-0-4` | `ifly-737-max8.shutdown.anti-collision-light` | moved |  |
| `shutdown-0-a0` | `ifly-737-max8.shutdown.taxi-light` | moved |  |
| `shutdown-0-5` | `ifly-737-max8.shutdown.fuel-pump-switches` | moved |  |
| `shutdown-0-6` | `ifly-737-max8.shutdown.cab-util-and-ife-pass-seat-switches` | moved |  |
| `shutdown-0-7` | `ifly-737-max8.shutdown.anti-ice` | moved |  |
| `shutdown-0-8` | `ifly-737-max8.shutdown.electric-hydraulic-pump-switches` | moved |  |
| `shutdown-0-9` | `ifly-737-max8.shutdown.pack-switches` | moved |  |
| `shutdown-0-10` | `ifly-737-max8.shutdown.isolation-valve-switch` | moved |  |
| `shutdown-0-11` | `ifly-737-max8.shutdown.apu-bleed-switch` | moved | Wording or setting changed during validation |
| `shutdown-0-12` | `ifly-737-max8.shutdown.flight-directors` | moved |  |
| `shutdown-0-13` | `ifly-737-max8.shutdown.wheel-chocks-and-cones` | moved | Wording or setting changed during validation |
| `shutdown-0-14` | `ifly-737-max8.shutdown.ground-power` | moved |  |
| `shutdown-0-15` | `ifly-737-max8.shutdown.irs` | moved |  |
| `shutdown-1-0` | `ifly-737-max8.shutdown.fuel-pump-switches-check` | moved |  |
| `shutdown-1-1` | `ifly-737-max8.shutdown.probe-heat` | moved | Wording or setting changed during validation |
| `shutdown-1-2` | `ifly-737-max8.shutdown.hydraulic-pump-switches` | moved |  |
| `shutdown-1-3` | `ifly-737-max8.shutdown.flaps` | moved |  |
| `shutdown-1-4` | `ifly-737-max8.shutdown.parking-brake-check` | moved | Wording or setting changed during validation |
| `shutdown-1-5` | `ifly-737-max8.shutdown.engine-start-levers-check` | moved |  |
| `shutdown-1-6` | `ifly-737-max8.shutdown.irs-check` | moved |  |
| `shutdown-1-7` | `ifly-737-max8.shutdown.weather-radar` | moved |  |
| `shutdown-1-8` | `ifly-737-max8.shutdown.fuel-quantity` | moved |  |
| `shutdown-1-9` | `ifly-737-max8.shutdown.outflow-valve-below-10c` | moved |  |
| `secure-0-0` | `ifly-737-max8.secure.irs` | moved |  |
| `secure-0-1` | `ifly-737-max8.secure.emergency-exit-lights` | moved |  |
| `secure-0-a0` | `ifly-737-max8.secure.window-heat-switches` | moved |  |
| `secure-0-2` | `ifly-737-max8.secure.pack-switches` | moved |  |
| `secure-0-3` | `ifly-737-max8.secure.apu-or-ground-power` | moved |  |
| `secure-0-4` | `ifly-737-max8.secure.before-battery-off` | moved |  |
| `secure-0-5` | `ifly-737-max8.secure.battery` | moved |  |
| `secure-1-0` | `ifly-737-max8.secure.electric-hydraulic-pump-switches` | moved |  |
| `secure-1-1` | `ifly-737-max8.secure.irs-check` | moved |  |
| `secure-1-2` | `ifly-737-max8.secure.window-heat` | moved |  |
| `secure-1-3` | `ifly-737-max8.secure.emergency-exit-lights-check` | moved |  |
| `secure-1-4` | `ifly-737-max8.secure.pack-switches-check` | moved |  |
| `secure-1-5` | `ifly-737-max8.secure.apu-or-ground-power-check` | moved |  |
| `secure-1-6` | `ifly-737-max8.secure.battery-check` | moved |  |
| `memory-0-0` | `ifly-737-max8.memory.control-column` | moved |  |
| `memory-0-1` | `ifly-737-max8.memory.autopilot-if-engaged` | moved |  |
| `memory-0-2` | `ifly-737-max8.memory.autothrottle-if-engaged` | moved |  |
| `memory-0-3` | `ifly-737-max8.memory.column-and-thrust-levers` | moved |  |
| `memory-0-4` | `ifly-737-max8.memory.main-electric-stabilizer-trim` | moved |  |
| `memory-0-5` | `ifly-737-max8.memory.if-runaway-stops-after-autopilot-is-disengaged` | moved |  |
| `memory-0-6` | `ifly-737-max8.memory.if-runaway-continues` | moved |  |
| `memory-0-7` | `ifly-737-max8.memory.if-runaway-still-continues` | moved |  |
| `memory-0-8` | `ifly-737-max8.memory.stabilizer` | moved |  |
| `memory-0-9` | `ifly-737-max8.memory.manual-trim` | moved |  |
| `memory-0-10` | `ifly-737-max8.memory.final-approach` | moved |  |
| `memory-1-0` | `ifly-737-max8.memory.autopilot-if-engaged-airspeed-unreliable` | moved |  |
| `memory-1-1` | `ifly-737-max8.memory.autothrottle-if-engaged-airspeed-unreliable` | moved |  |
| `memory-1-2` | `ifly-737-max8.memory.f-d-switches-both` | moved |  |
| `memory-1-3` | `ifly-737-max8.memory.if-flaps-extended` | moved |  |
| `memory-1-4` | `ifly-737-max8.memory.if-flaps-up` | moved |  |
| `memory-1-5` | `ifly-737-max8.memory.probe-heat-switches` | moved | Wording or setting changed during validation |
| `memory-1-6` | `ifly-737-max8.memory.reliable-references` | moved |  |
| `memory-1-7` | `ifly-737-max8.memory.airspeed` | moved |  |
| `memory-1-8` | `ifly-737-max8.memory.if-only-standby-airspeed-is-reliable` | moved |  |
| `memory-1-9` | `ifly-737-max8.memory.if-captain-or-fo-airspeed-is-reliable` | moved |  |
| `memory-1-10` | `ifly-737-max8.memory.if-no-reliable-airspeed` | moved |  |
| `memory-1-11` | `ifly-737-max8.memory.transponder` | moved | Wording or setting changed during validation |
| `memory-2-0` | `ifly-737-max8.memory.oxygen-masks-and-regulators` | moved |  |
| `memory-2-1` | `ifly-737-max8.memory.crew-communications` | moved |  |
| `memory-2-2` | `ifly-737-max8.memory.pressurization-mode-selector` | moved |  |
| `memory-2-3` | `ifly-737-max8.memory.outflow-valve-switch` | moved | `expect` rewritten in our own words on import |
| `memory-2-4` | `ifly-737-max8.memory.if-cabin-altitude-is-uncontrollable` | moved |  |
| `memory-2-5` | `ifly-737-max8.memory.if-cabin-altitude-is-above-or-expected-to-go` | moved |  |
| `memory-2-6` | `ifly-737-max8.memory.if-cabin-altitude-is-controllable` | moved | `expect` rewritten in our own words on import; Wording or setting changed during validation |
| `memory-3-0` | `ifly-737-max8.memory.oxygen-masks-and-regulators-if-required` | moved |  |
| `memory-3-1` | `ifly-737-max8.memory.smoke-goggles-if-required` | moved |  |
| `memory-3-2` | `ifly-737-max8.memory.crew-communications-smoke-fire-or-fumes` | moved |  |
| `memory-3-3` | `ifly-737-max8.memory.recirc-fan-switches-both` | moved |  |
| `quick-reference-0-0` | `ifly-737-max8.quick-reference.before-start-lever-is-raised-to-idle` | moved |  |
| `quick-reference-0-1` | `ifly-737-max8.quick-reference.after-start-lever-is-raised-to-idle` | moved |  |
| `quick-reference-1-0` | `ifly-737-max8.quick-reference.autothrottle-if-engaged` | moved |  |
| `quick-reference-1-1` | `ifly-737-max8.quick-reference.thrust-lever-affected-engine` | moved |  |
| `quick-reference-1-2` | `ifly-737-max8.quick-reference.start-lever-affected-engine` | moved |  |
| `quick-reference-1-3` | `ifly-737-max8.quick-reference.engine-fire-switch-affected-engine` | moved | Wording or setting changed during validation |
| `quick-reference-1-4` | `ifly-737-max8.quick-reference.if-the-engine-fire-switch-or-eng-overheat-light` | moved | `text` rewritten in our own words on import |
| `quick-reference-1-5` | `ifly-737-max8.quick-reference.if-the-fire-switch-or-eng-overheat-light-is` | moved | Wording or setting changed during validation |
| `quick-reference-2-0` | `ifly-737-max8.quick-reference.autothrottle-if-engaged-engine-limit-surge` | moved |  |
| `quick-reference-2-1` | `ifly-737-max8.quick-reference.thrust-lever-affected-engine-engine-limit-surge` | moved |  |
| `quick-reference-3-0` | `ifly-737-max8.quick-reference.autothrottle-if-engaged-engine-overheat` | moved |  |
| `quick-reference-3-1` | `ifly-737-max8.quick-reference.thrust-lever-affected-engine-engine-overheat` | moved |  |
| `quick-reference-3-2` | `ifly-737-max8.quick-reference.if-eng-overheat-light-stays-illuminated` | moved | Wording or setting changed during validation |
| `quick-reference-4-0` | `ifly-737-max8.quick-reference.airspeed` | moved |  |
| `quick-reference-4-1` | `ifly-737-max8.quick-reference.start-switches-both` | moved |  |
| `quick-reference-4-2` | `ifly-737-max8.quick-reference.start-levers-both` | moved |  |
| `quick-reference-4-3` | `ifly-737-max8.quick-reference.when-egt-decreases` | moved | Wording or setting changed during validation |
| `quick-reference-4-4` | `ifly-737-max8.quick-reference.if-egt-reaches-a-redline-or-does-not-rise-in-30` | moved | Wording or setting changed during validation |
| `quick-reference-4-5` | `ifly-737-max8.quick-reference.engine-acceleration` | moved | Wording or setting changed during validation |
| `quick-reference-4-6` | `ifly-737-max8.quick-reference.apu-if-available` | moved |  |
| `quick-reference-5-0` | `ifly-737-max8.quick-reference.apu-fire-switch` | moved |  |
| `quick-reference-5-1` | `ifly-737-max8.quick-reference.apu-switch` | moved |  |
| `quick-reference-6-0` | `ifly-737-max8.quick-reference.emergency-descent` | moved | Wording or setting changed during validation |
| `quick-reference-6-1` | `ifly-737-max8.quick-reference.fasten-belts` | moved |  |
| `quick-reference-6-2` | `ifly-737-max8.quick-reference.descent` | moved | `expect` rewritten in our own words on import |
| `quick-reference-6-3` | `ifly-737-max8.quick-reference.start-switches` | moved |  |
| `quick-reference-6-4` | `ifly-737-max8.quick-reference.thrust-levers` | moved | `expect` rewritten in our own words on import |
| `quick-reference-6-5` | `ifly-737-max8.quick-reference.speed-brake` | moved |  |
| `quick-reference-6-6` | `ifly-737-max8.quick-reference.target-speed` | moved |  |
| `quick-reference-6-7` | `ifly-737-max8.quick-reference.atc` | moved |  |
| `quick-reference-6-8` | `ifly-737-max8.quick-reference.approaching-level-off` | moved |  |
| `quick-reference-6-9` | `ifly-737-max8.quick-reference.crew-oxygen-regulators` | moved |  |
| `quick-reference-7-0` | `ifly-737-max8.quick-reference.airspeed-overspeed` | moved |  |
| `quick-reference-8-0` | `ifly-737-max8.quick-reference.start-lever-affected-engine-engine-tailpipe-fire` | moved |  |
| `quick-reference-9-0` | `ifly-737-max8.quick-reference.cargo-fire-armed-switch-affected-compartment` | moved | Wording or setting changed during validation |
| `quick-reference-9-1` | `ifly-737-max8.quick-reference.cargo-fire-disch-switch` | moved |  |
| `quick-reference-9-2` | `ifly-737-max8.quick-reference.recirc-fan-switches-both` | moved |  |
| `quick-reference-10-0` | `ifly-737-max8.quick-reference.airspeed-wheel-well-fire` | moved |  |
| `quick-reference-10-1` | `ifly-737-max8.quick-reference.landing-gear-lever` | moved |  |
| `quick-reference-11-0` | `ifly-737-max8.quick-reference.configuration` | moved |  |
| `quick-reference-12-0` | `ifly-737-max8.quick-reference.parking-brake` | moved |  |
| `quick-reference-12-1` | `ifly-737-max8.quick-reference.speed-brake-passenger-evacuation` | moved |  |
| `quick-reference-12-2` | `ifly-737-max8.quick-reference.flap-lever` | moved |  |
| `quick-reference-12-3` | `ifly-737-max8.quick-reference.pressurization-mode-selector` | moved |  |
| `quick-reference-12-4` | `ifly-737-max8.quick-reference.outflow-valve-switch` | moved |  |
| `quick-reference-12-5` | `ifly-737-max8.quick-reference.flaps` | moved |  |
| `quick-reference-12-6` | `ifly-737-max8.quick-reference.start-levers-both-passenger-evacuation` | moved |  |
| `quick-reference-12-7` | `ifly-737-max8.quick-reference.cabin` | moved |  |
| `quick-reference-12-8` | `ifly-737-max8.quick-reference.tower` | moved |  |
| `quick-reference-12-9` | `ifly-737-max8.quick-reference.engine-and-apu-fire-switches-all` | moved |  |
| `quick-reference-12-10` | `ifly-737-max8.quick-reference.if-a-fire-warning-is-observed-or-indicated` | moved |  |
| `procedures-0-0` | `ifly-737-max8.procedures.stab-trim-cutout-switches-both` | moved |  |
| `procedures-0-1` | `ifly-737-max8.procedures.autopilot` | moved |  |
| `procedures-0-2` | `ifly-737-max8.procedures.stabilizer` | moved |  |
| `procedures-0-3` | `ifly-737-max8.procedures.airspeed` | moved |  |
| `procedures-0-4` | `ifly-737-max8.procedures.landing` | moved |  |
| `procedures-0-5` | `ifly-737-max8.procedures.go-around` | moved |  |
| `procedures-1-0` | `ifly-737-max8.procedures.stab-out-of-trim-light-on-the-ground` | moved | `text` rewritten in our own words on import |
| `procedures-1-1` | `ifly-737-max8.procedures.light-momentary-in-flight-stabilizer-trimming` | moved |  |
| `procedures-1-2` | `ifly-737-max8.procedures.in-flight-stabilizer-not-trimming` | moved |  |
| `procedures-1-3` | `ifly-737-max8.procedures.if-stabilizer-responds-to-electric-trim` | moved |  |
| `procedures-1-4` | `ifly-737-max8.procedures.if-the-stabilizer-does-not-respond` | moved |  |
| `procedures-1-5` | `ifly-737-max8.procedures.light-on-after-landing` | moved |  |
| `procedures-2-0` | `ifly-737-max8.procedures.speed-trim-and-mcas` | moved |  |
| `procedures-2-1` | `ifly-737-max8.procedures.operation` | moved |  |
| `procedures-3-0` | `ifly-737-max8.procedures.aoa-disagree` | moved |  |
| `procedures-3-1` | `ifly-737-max8.procedures.ias-disagree` | moved |  |
| `procedures-3-2` | `ifly-737-max8.procedures.alt-disagree-with-ias-disagree` | moved |  |
| `procedures-3-3` | `ifly-737-max8.procedures.alt-disagree-alone` | moved | Wording or setting changed during validation |
| `procedures-4-0` | `ifly-737-max8.procedures.below-80-knots` | moved | `expect` rewritten in our own words on import |
| `procedures-4-1` | `ifly-737-max8.procedures.80-knots-to-v1` | moved | `expect` rewritten in our own words on import |
| `procedures-4-2` | `ifly-737-max8.procedures.v1` | moved |  |
| `procedures-4-3` | `ifly-737-max8.procedures.thrust-levers-and-brakes` | moved |  |
| `procedures-4-4` | `ifly-737-max8.procedures.speedbrake` | moved |  |
| `procedures-4-5` | `ifly-737-max8.procedures.reverse-thrust` | moved |  |
| `procedures-4-6` | `ifly-737-max8.procedures.braking` | moved |  |
| `procedures-4-7` | `ifly-737-max8.procedures.reverse-levers` | moved |  |
| `procedures-4-8` | `ifly-737-max8.procedures.tower-and-cabin` | moved |  |
| `procedures-4-9` | `ifly-737-max8.procedures.brakes` | moved |  |
| `procedures-5-0` | `ifly-737-max8.procedures.autopilot-and-to-ga` | moved |  |
| `procedures-5-1` | `ifly-737-max8.procedures.thrust` | moved |  |
| `procedures-5-2` | `ifly-737-max8.procedures.attitude` | moved | Wording or setting changed during validation |
| `procedures-5-3` | `ifly-737-max8.procedures.speedbrakes` | moved |  |
| `procedures-5-4` | `ifly-737-max8.procedures.flight-director` | moved |  |
| `procedures-5-5` | `ifly-737-max8.procedures.flaps-and-gear` | moved | Wording or setting changed during validation |
| `procedures-5-6` | `ifly-737-max8.procedures.vertical-speed-and-altitude` | moved |  |
| `procedures-6-0` | `ifly-737-max8.procedures.autopilot-and-autothrottle` | moved |  |
| `procedures-6-1` | `ifly-737-max8.procedures.thrust-terrain-escape` | moved |  |
| `procedures-6-2` | `ifly-737-max8.procedures.attitude-terrain-escape` | moved | `expect` rewritten in our own words on import |
| `procedures-6-3` | `ifly-737-max8.procedures.speedbrakes-terrain-escape` | moved |  |
| `procedures-6-4` | `ifly-737-max8.procedures.if-terrain-remains-a-threat` | moved |  |
| `procedures-6-5` | `ifly-737-max8.procedures.gear-and-flaps` | moved |  |
| `procedures-6-6` | `ifly-737-max8.procedures.flight-director-terrain-escape` | moved |  |
| `procedures-7-0` | `ifly-737-max8.procedures.autopilot-and-autothrottle-stall-recovery` | moved |  |
| `procedures-7-1` | `ifly-737-max8.procedures.pitch` | moved | Wording or setting changed during validation |
| `procedures-7-2` | `ifly-737-max8.procedures.roll` | moved | Wording or setting changed during validation |
| `procedures-7-3` | `ifly-737-max8.procedures.thrust-and-speedbrakes` | moved |  |
| `procedures-7-4` | `ifly-737-max8.procedures.gear-and-flaps-stall-recovery` | moved |  |
| `procedures-7-5` | `ifly-737-max8.procedures.flight-path` | moved |  |
| `procedures-8-0` | `ifly-737-max8.procedures.if-stalled` | moved |  |
| `procedures-8-1` | `ifly-737-max8.procedures.autopilot-and-autothrottle-upset-recovery` | moved |  |
| `procedures-8-2` | `ifly-737-max8.procedures.nose-high` | moved |  |
| `procedures-8-3` | `ifly-737-max8.procedures.nose-low` | moved |  |
| `procedures-9-0` | `ifly-737-max8.procedures.autopilot-and-autothrottle-tcas-resolution` | moved |  |
| `procedures-9-1` | `ifly-737-max8.procedures.pitch-tcas-resolution` | moved |  |
| `procedures-9-2` | `ifly-737-max8.procedures.lateral-path` | moved |  |
| `procedures-9-3` | `ifly-737-max8.procedures.climb-ra-in-landing-configuration` | moved |  |
| `procedures-9-4` | `ifly-737-max8.procedures.near-the-ground` | moved |  |
| `procedures-9-5` | `ifly-737-max8.procedures.atc` | moved |  |
| `procedures-10-0` | `ifly-737-max8.procedures.to-ga` | moved |  |
| `procedures-10-1` | `ifly-737-max8.procedures.flaps` | moved |  |
| `procedures-10-2` | `ifly-737-max8.procedures.thrust-go-around` | moved |  |
| `procedures-10-3` | `ifly-737-max8.procedures.positive-rate` | moved |  |
| `procedures-10-4` | `ifly-737-max8.procedures.missed-approach-altitude` | moved |  |
| `procedures-10-5` | `ifly-737-max8.procedures.roll-mode` | moved |  |
| `procedures-10-6` | `ifly-737-max8.procedures.flaps-go-around` | moved |  |
| `procedures-10-7` | `ifly-737-max8.procedures.pitch-mode` | moved |  |
| `procedures-10-8` | `ifly-737-max8.procedures.missed-approach-altitude-go-around` | moved |  |
| `procedures-10-9` | `ifly-737-max8.procedures.after-takeoff-checklist` | moved |  |

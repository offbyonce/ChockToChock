// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Text written in our own words to replace imported wording that the overlap check flagged.
// Applied after ids are assigned, so a step's id does not change. The report lists each one.

const STEP = "ifly-737-max8";

export const REWRITES = {
  steps: {
    [`${STEP}.procedures.80-knots-to-v1`]: {
      expect:
        "STOP ONLY FOR: ENGINE FAILURE, A FIRE OR ITS WARNING, A WINDSHEAR ALERT, OR AN AIRPLANE THAT CANNOT FLY SAFELY",
    },
    [`${STEP}.procedures.below-80-knots`]: {
      expect:
        "STOP FOR ANY OF THE ABOVE, OR: MASTER CAUTION, A SYSTEM FAILURE, ODD NOISE OR SHAKING, A BLOWN TIRE, SLUGGISH ACCELERATION, A CONFIG WARNING, OR A WINDOW COMING OPEN",
    },
    [`${STEP}.quick-reference.descent`]: {
      expect: "NOW, TO THE GREATER OF 10,000 FT AND MINIMUM SAFE ALTITUDE",
    },
    [`${STEP}.quick-reference.thrust-levers`]: {
      expect: "CLOSE, OR AS FAR AS ANTI-ICE ALLOWS",
    },
    [`${STEP}.quick-reference.if-the-engine-fire-switch-or-eng-overheat-light`]: {
      text: "FIRE SWITCH OR OVERHEAT LIGHT STILL ON",
    },
    [`${STEP}.procedures.stab-out-of-trim-light-on-the-ground`]: {
      text: "ON THE GROUND: STAB OUT OF TRIM LIGHT",
    },
    [`${STEP}.procedures.attitude-terrain-escape`]: {
      expect: "LEVEL THE WINGS, THEN PITCH UP TO 20 DEGREES TO START",
    },
    [`${STEP}.memory.outflow-valve-switch`]: {
      expect: "HOLD TO CLOSE UNTIL THE INDICATOR SHOWS FULLY SHUT",
    },
    [`${STEP}.memory.if-cabin-altitude-is-controllable`]: {
      expect: "KEEP CONTROLLING IT MANUALLY. MASKS MAY COME OFF WITH CABIN ALTITUDE AT OR BELOW 10,000 FT",
    },
    [`${STEP}.preflight.c-r-switch`]: {
      expect: "PRESS, THEN CHECK THAT NO AUTOLAND FAULT SHOWS",
    },
    [`${STEP}.approach.multi-mode-receiver`]: {
      expect: "TUNE THE ILS FOR THE LANDING RUNWAY",
    },
    [`${STEP}.before-start.fuel`]: {
      expect:
        "REQUIRED ___ KG/LBS, ON BOARD ___ KG/LBS, FUEL PUMPS ON, CENTER PUMPS ON IF MORE THAN 1,000 LB (ABOUT 450 KG) IN THE CENTER TANK",
    },
  },
  lists: {
    "memory-airspeed-unreliable": {
      when: "You suspect one or more airspeed or Mach readings are wrong.",
    },
  },
  controls: {
    dispsw: { function: "SOURCE goes to AUTO, CONTROL PANEL to NORMAL." },
    motor: {
      function:
        "Shows while the engine is being motored by the starter. Keep the start lever in CUTOFF until this clears. It shows over the N2 readout, typically at about 18 to 24% N2.",
    },
  },
  briefingItems: {
    "takeoff-briefing-items-gate-stand.atis-weather": {
      detail:
        "Current ATIS: active runway, pressure setting, wind, runway state, any windshear reports, and limits for this airplane.",
    },
    "approach-briefing.atis-weather": {
      detail:
        "Current ATIS: active runway, pressure setting, wind, runway state, any windshear reports, and limits for this airplane.",
    },
    "takeoff-briefing-items-gate-stand.taxi-route": {
      detail:
        "How we expect to reach the runway and how long it will take, one or two engines, hot spots and crossings, APU plan.",
    },
    "approach-briefing.taxi-route": {
      detail:
        "How we expect to reach the gate and how long it will take, one or two engines, hot spots and crossings, when to start the APU.",
    },
    "takeoff-briefing-items-gate-stand.clearance": {
      detail:
        "What ATC gave us: runway and departure, takeoff flaps, first altitude, route and cruise level.",
    },
    "approach-briefing.clearance": {
      detail:
        "The expected STAR and approach, landing runway and flaps, lowest altitude on the arrival, expected level-offs, fuel for a missed approach or diversion.",
    },
    "takeoff-briefing-items-gate-stand.threats-and-abnormals": {
      detail: "Today's risks: terrain, weather, a high or short field, noise limits, an overweight return.",
    },
    "approach-briefing.threats-and-abnormals": {
      detail:
        "Today's risks: terrain, weather, a high or short field, noise limits, an overweight return, a late runway change.",
    },
    "takeoff-briefing-taxi-out.runway": {
      detail:
        "Has the runway or performance changed? If so, recompute and update the CDU before the hold-short line.",
    },
    "takeoff-briefing-taxi-out.departure": {
      detail:
        "RNAV departure: LNAV and VNAV armed for takeoff. Vectors: set the heading bug to the assigned heading and select HDG SEL after takeoff, normally from 400 ft.",
    },
    "takeoff-briefing-taxi-out.initial-altitude": {
      detail: "Compare the MCP with the clearance and the departure chart.",
    },
  },
};

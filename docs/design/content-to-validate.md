# Content to source and validate before it ships

The mockups contain wording and data written to show the format. None of it may ship until it is sourced and checked under the project's sourcing policy (decision 0007 and its amendments) and passes `pack check`.

## New step: Center tank fuel

- A decision step before Fuel pump switches in Before start: check the center tank quantity, answer "more than 1,000 lb (about 450 kg)" or "1,000 lb (450 kg) or less".
- Prefilled from the SimBrief center tank value when a plan is loaded, always changeable. With no plan, the pilot answers.
- The Fuel pump switches step then resolves to MAIN PUMPS ON with CENTER PUMPS ON or OFF, and its drawing shows the resolved center switch positions.
- This adds a pack step and changes how the fuel step's target is expressed. It needs: sourcing for where the pilot reads the quantity in the iFly, a pack format decision for decision steps and steps whose target depends on one, and validation of the threshold wording against the existing step.

## Wording written for the mockups

- Every If not line (for example "A main light stays on: check that pump's switch is ON"). The new rule is condition plus action; the action must come from sources.
- "Main pump LOW PRESSURE lights out" as the fuel step's Check.
- Flight page descriptions for zero fuel weight, block fuel, reserves, trip fuel, center tank, wind, visibility, clouds, temperature, altimeter, arrival runway, STAR, minimums, transition level and landing speed.
- Destination chips: CDU · PERF INIT · ZFW and RESERVES, CDU · TAKEOFF REF for wind and temperature, EFIS · BARO KNOB, CDU · DEP/ARR, EFIS · MINS, CDU · DESCENT, CDU · APPROACH REF.
- CDU walkthrough: page order, PERF INIT line labels and the line key for each entry (ZFW on 3L).
- Light meaning text for LOW PRESSURE.
- Briefing prompts beyond the ones filled from flight data.
- Emergency items are placeholders on purpose; titles and steps come from the pack.

## Panel icons and drawing quality (assigned to you)

The owner wants the icons and drawing detail in the panel views improved. Within the rules: geometry and arrangement stay as the pack defines them; improve switch, knob, selector, guard and light-lens drawing, label legibility and both themes' rendering through the shared drawing renderer and tokens. Show before and after for every panel type in the reviewer brief.

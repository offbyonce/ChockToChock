<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Weather report fixtures

Real METAR and TAF reports, US and international styles, for the decoder's tests
(`src/weather.rs`). They were taken on 2026-11-14 from the US National Weather Service's public
aviation weather data, which collects the reports airports publish; weather reports are facts.
They are kept exactly as received: `metar.txt` one report per line, `taf.txt` one forecast per
record with its change groups on indented lines.

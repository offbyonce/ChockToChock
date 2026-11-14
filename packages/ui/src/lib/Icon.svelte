<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One general interface icon. Only the Tabler icons used are copied in, as SVG files under
icons/tabler (MIT; decision 0010 adds no JavaScript dependency); icons/board holds the few drawn
on the approved boards. Icons are decorative: the
control around them carries the words.
-->
<script lang="ts" module>
  import alertTriangle from "../icons/tabler/alert-triangle.svg?raw";
  import arrowLeft from "../icons/tabler/arrow-left.svg?raw";
  import arrowRight from "../icons/tabler/arrow-right.svg?raw";
  import check from "../icons/tabler/check.svg?raw";
  import chevronDown from "../icons/tabler/chevron-down.svg?raw";
  import chevronLeft from "../icons/tabler/chevron-left.svg?raw";
  import chevronRight from "../icons/tabler/chevron-right.svg?raw";
  import mapPin from "../icons/tabler/map-pin.svg?raw";
  import menu from "../icons/tabler/menu-2.svg?raw";
  import plane from "../icons/tabler/plane.svg?raw";
  import skip from "../icons/tabler/player-skip-forward.svg?raw";
  import search from "../icons/tabler/search.svg?raw";
  import settings from "../icons/tabler/settings.svg?raw";
  import x from "../icons/tabler/x.svg?raw";
  import decision from "../icons/board/decision.svg?raw";
  import fieldAlert from "../icons/board/field-alert.svg?raw";
  import fileUpload from "../icons/board/file-upload.svg?raw";
  import globe from "../icons/board/globe.svg?raw";
  import notice from "../icons/board/notice.svg?raw";
  import panel from "../icons/board/panel.svg?raw";
  import pin from "../icons/board/pin.svg?raw";

  const ICONS = {
    "alert-triangle": alertTriangle,
    "arrow-left": arrowLeft,
    "arrow-right": arrowRight,
    check,
    "chevron-down": chevronDown,
    "chevron-left": chevronLeft,
    "chevron-right": chevronRight,
    decision,
    "field-alert": fieldAlert,
    "file-upload": fileUpload,
    globe,
    "map-pin": mapPin,
    notice,
    panel,
    pin,
    menu,
    plane,
    skip,
    search,
    settings,
    x,
  };

  export type IconName = keyof typeof ICONS;
</script>

<script lang="ts">
  let { name }: { name: IconName } = $props();
</script>

<span class="icon" aria-hidden="true">{@html ICONS[name]}</span>

<style>
  .icon {
    display: inline-flex;
    width: var(--size-icon);
    height: var(--size-icon);
    flex: none;
  }
  .icon :global(svg) {
    width: 100%;
    height: 100%;
  }
</style>

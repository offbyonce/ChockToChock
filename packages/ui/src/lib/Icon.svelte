<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One general interface icon. Only the Tabler icons used are copied in, as SVG files under
icons/tabler (MIT; decision 0010 adds no JavaScript dependency). Icons are decorative: the
control around them carries the words.
-->
<script lang="ts" module>
  import alertTriangle from "../icons/tabler/alert-triangle.svg?raw";
  import chevronRight from "../icons/tabler/chevron-right.svg?raw";
  import menu from "../icons/tabler/menu-2.svg?raw";
  import plane from "../icons/tabler/plane.svg?raw";
  import search from "../icons/tabler/search.svg?raw";
  import settings from "../icons/tabler/settings.svg?raw";
  import x from "../icons/tabler/x.svg?raw";

  const ICONS = {
    "alert-triangle": alertTriangle,
    "chevron-right": chevronRight,
    menu,
    plane,
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
    width: 1.375rem;
    height: 1.375rem;
    flex: none;
  }
  .icon :global(svg) {
    width: 100%;
    height: 100%;
  }
</style>

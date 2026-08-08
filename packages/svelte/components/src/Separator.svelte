<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/separator.css";
  import type { Orientation, SeparatorTone } from "./types";

  interface Props {
    orientation?: Orientation;
    decorative?: boolean;
    tone?: SeparatorTone;
  }

  let {
    orientation = "horizontal",
    decorative = true,
    tone = "subtle",
  }: Props = $props();
</script>

<div
  class="poodle-separator"
  data-orientation={orientation}
  data-tone={tone}
  role={decorative ? undefined : "separator"}
  aria-hidden={decorative ? "true" : undefined}
  aria-orientation={decorative ? undefined : orientation}
></div>


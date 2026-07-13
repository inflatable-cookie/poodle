<script lang="ts">
  import "@poodle/styles/status-bar.css";
  import type { Snippet } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    summary?: string | null;
    ariaLabel?: string | null;
    chrome?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    leading?: Snippet;
    trailing?: Snippet;
  }

  const uiPresentation = getUiPresentation();

  let {
    summary = null,
    ariaLabel = null,
    chrome = false,
    size = null,
    sizeRole = "chrome",
    density = null,
    leading,
    trailing,
  }: Props = $props();

  let resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  let resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<footer
  class="poodle-status-bar"
  class:poodle-status-bar--chrome={chrome}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  aria-label={ariaLabel ?? summary ?? "Status"}
>
  <div class="poodle-status-bar__leading">
    {#if leading}
      {@render leading()}
    {:else if summary}
      <span>{summary}</span>
    {/if}
  </div>

  {#if trailing}
    <div class="poodle-status-bar__trailing">
      {@render trailing()}
    </div>
  {/if}
</footer>


<script lang="ts">
  import "@poodle/styles/pill.css";
  import type { Snippet } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import { getPillContext, type PillTypography } from "./pill-context";
  import type { ControlDensity, SemanticControlSizeRole } from "./types";
  import type { PillAppearance, PillFont, PillSize, PillTone } from "./types";

  let {
    tone = "neutral",
    appearance = "solid",
    size = null,
    sizeRole = "chrome",
    density = null,
    font = "normal",
    typography = "label",
    accent = null,
    muted = false,
    adaptiveWidth = false,
    ariaLabel = null,
    children = undefined,
  }: {
    tone?: PillTone;
    appearance?: PillAppearance;
    size?: PillSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    font?: PillFont;
    typography?: PillTypography;
    accent?: string | null;
    muted?: boolean;
    adaptiveWidth?: boolean;
    ariaLabel?: string | null;
    children?: Snippet;
  } = $props();

  const uiPresentation = getUiPresentation();
  const pillContext = getPillContext();

  const resolvedSize = $derived(
    (pillContext?.size ?? size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole)) as PillSize,
  );
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedTypography = $derived(pillContext?.typography ?? typography);
</script>

<span
  class="poodle-pill"
  data-tone={tone}
  data-appearance={appearance}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-font={font}
  data-typography={resolvedTypography}
  data-muted={muted}
  data-adaptive-width={adaptiveWidth || undefined}
  data-accent={accent ? "custom" : undefined}
  aria-label={ariaLabel ?? undefined}
  style:--poodle-pill-accent={accent ?? undefined}
>
  {@render children?.()}
</span>


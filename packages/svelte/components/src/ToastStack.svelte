<script lang="ts">
  import "@poodle/styles/toast-stack.css";
  import { default as Button } from "./Button.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types.ts";

  import type { ToastItem } from "./types.ts";

  let {
    items = [],
    ariaLabel = "Notifications",
    size = null,
    sizeRole = "chrome",
    density = null,
    onDismiss = undefined,
    onAction = undefined,
  }: {
    items?: ToastItem[];
    ariaLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onDismiss?: ((id: string) => void) | undefined;
    onAction?: ((id: string) => void) | undefined;
  } = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<ul class="poodle-toast-stack" aria-label={ariaLabel} aria-live="polite" aria-atomic="false" data-size={resolvedSize} data-density={resolvedDensity}>
  {#each items as item (item.id)}
    <li
      class="poodle-toast"
      data-tone={item.tone ?? "info"}
      aria-live={item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
    >
      <button type="button" class="poodle-toast__dismiss" aria-label={`Dismiss ${item.title}`} onclick={() => onDismiss?.(item.id)}>
        <Icon name="x" />
      </button>

      <div class="poodle-toast__copy">
        <strong>{item.title}</strong>
        {#if item.message}
          <p>{item.message}</p>
        {/if}
      </div>

      {#if item.actionLabel}
        <div class="poodle-toast__actions">
          <Button variant="secondary" size={resolvedSize} density={resolvedDensity} onClick={() => onAction?.(item.id)}>
            {item.actionLabel}
          </Button>
        </div>
      {/if}
    </li>
  {/each}
</ul>


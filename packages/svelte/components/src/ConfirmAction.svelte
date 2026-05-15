<script lang="ts">
  import { type Snippet } from "svelte";

  import AlertDialog from "./AlertDialog.svelte";
  import Button from "./Button.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    title: string;
    description?: string | null;
    tone?: AlertDialogTone;
    triggerLabel?: string;
    confirmLabel?: string;
    cancelLabel?: string;
    onConfirm?: (() => void | Promise<void>) | null;
    onCancel?: (() => void) | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    trigger?: Snippet<[]>;
    children?: Snippet<[]>;
  }

  let {
    title,
    description = null,
    tone = "danger",
    triggerLabel = "Delete",
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    onConfirm = null,
    onCancel = null,
    size = null,
    sizeRole = "control",
    density = null,
    trigger,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let open = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const triggerTone = $derived(tone === "danger" ? "danger" as const : "default" as const);

  function handleTrigger(): void {
    open = true;
  }

  async function handleConfirm(): Promise<void> {
    await onConfirm?.();
    open = false;
  }

  function handleCancel(): void {
    onCancel?.();
    open = false;
  }

  function handleOpenChange(event: CustomEvent<{ open: boolean }>): void {
    open = event.detail.open;
  }
</script>

{#if trigger}
  <span
    class="poodle-confirm-action__trigger"
    data-size={resolvedSize}
    data-density={resolvedDensity}
    role="presentation"
    onclick={handleTrigger}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        handleTrigger();
      }
    }}
  >
    {@render trigger?.()}
  </span>
{:else}
  <Button variant="secondary" tone={triggerTone} size={resolvedSize} density={resolvedDensity} onclick={handleTrigger}>
    {triggerLabel}
  </Button>
{/if}

<AlertDialog
  open={open || null}
  {title}
  {description}
  {tone}
  {confirmLabel}
  {cancelLabel}
  size={resolvedSize}
  density={resolvedDensity}
  on:confirm={handleConfirm}
  on:cancel={handleCancel}
  on:openChange={handleOpenChange}
>
  {@render children?.()}
</AlertDialog>

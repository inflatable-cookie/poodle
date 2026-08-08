<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/alert-dialog.css";
  import type { Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as Dialog } from "./Dialog.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    open?: boolean | null | undefined;
    title: string;
    description?: string | null;
    itemLabel?: string | null;
    itemValue?: string | null;
    tone?: AlertDialogTone;
    confirmLabel?: string;
    cancelLabel?: string;
    ariaLabel?: string | null;
    workingLabel?: string;
    onConfirm?: (() => void | Promise<void>) | null;
    onCancel?: (() => void) | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    children?: Snippet<[]>;
  }

  let {
    open = $bindable<boolean | null | undefined>(undefined),
    title,
    description = null,
    itemLabel = null,
    itemValue = null,
    tone = "danger",
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    ariaLabel = null,
    workingLabel = "Working…",
    onConfirm = null,
    onCancel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onOpenChange = undefined,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let working = $state(false);
  let uncontrolledOpen = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const confirmTone = $derived(tone === "danger" ? "danger" as const : "default" as const);
  const isControlled = $derived(open !== undefined);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  async function handleConfirm(): Promise<void> {
    if (working) {
      return;
    }

    working = true;

    try {
      if (onConfirm) {
        await onConfirm();
      }

      setOpen(false);
    } catch {
      // Keep the dialog open so the caller can recover or retry.
    } finally {
      working = false;
    }
  }

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    } else {
      // Write back through the binding before notifying, so `bind:open` works
      // as it does on every other bindable Poodle component. A host that wants
      // to refuse the close re-asserts the value inside `onOpenChange`, which
      // lands last and renders no intermediate state.
      open = nextOpen;
    }

    onOpenChange?.(nextOpen);
  }

  function handleCancel(): void {
    onCancel?.();
    setOpen(false);
  }

  function handleDialogOpenChange(nextOpen: boolean): void {
    if (!nextOpen && !working) {
      onCancel?.();
    }

    setOpen(nextOpen);
  }
</script>

<div data-size={resolvedSize} data-density={resolvedDensity}>
  <Dialog
    open={isOpen}
    {title}
    {description}
    role="alertdialog"
    width="sm"
    {ariaLabel}
    size={resolvedSize}
    density={resolvedDensity}
    dismissOnEscape={!working}
    dismissOnBackdrop={!working}
    showCloseButton={!working}
    onOpenChange={handleDialogOpenChange}
  >
    {#snippet actions()}
      <Button
        variant="ghost"
        size={resolvedSize}
        density={resolvedDensity}
        onClick={handleCancel}
        disabled={working}
      >
        {cancelLabel}
      </Button>
      <Button
        variant="primary"
        tone={confirmTone}
        size={resolvedSize}
        density={resolvedDensity}
        onClick={handleConfirm}
        disabled={working}
      >
        {working ? workingLabel : confirmLabel}
      </Button>
    {/snippet}

    {#if itemLabel && itemValue}
      <p class="poodle-alert-dialog__item-detail">
        <strong>{itemLabel}:</strong> {itemValue}
      </p>
    {/if}

    {@render children?.()}
  </Dialog>
</div>


<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Button, Dialog, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  import FormLayout from "./FormLayout.svelte";

  export let open: boolean | null = null;
  export let title: string;
  export let subtitle: string | null = null;
  export let description: string | null = null;
  export let submitLabel = "Submit";
  export let cancelLabel = "Cancel";
  export let submitting = false;
  export let error: string | null = null;
  export let success: string | null = null;
  export let ariaLabel: string | null = null;
  export let width: string | null = null;
  export let columns = 6;
  export let showDefaultActions = true;
  /** When true, the Dialog body renders the default slot directly with no FormLayout wrapper. */
  export let bare = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    submit: void;
    cancel: void;
    openChange: { open: boolean };
  }>();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedShowActions = bare ? false : showDefaultActions;
  $: resolvedDescription = subtitle ?? description;
  $: contentStyle = width ? `--poodle-form-dialog-width: ${width};` : "";
  $: contentClassName = width ? "form-dialog__surface" : "";

  function handleSubmit(): void {
    dispatch("submit");
  }

  function notifyCancel(): void {
    dispatch("cancel");
  }

  function setOpen(nextOpen: boolean): void {
    if (open !== null) {
      dispatch("openChange", { open: nextOpen });
    }
  }

  function handleCancel(): void {
    notifyCancel();
    setOpen(false);
  }

  function handleOpenChange(event: CustomEvent<{ open: boolean }>): void {
    if (!event.detail.open && !submitting) {
      notifyCancel();
    }

    dispatch("openChange", event.detail);
  }
</script>

<Dialog
  {open}
  title={title}
  description={$$slots.subtitle ? null : resolvedDescription}
  kind="dialog"
  {ariaLabel}
  size={resolvedSize}
  density={resolvedDensity}
  dismissOnEscape={!submitting}
  dismissOnBackdrop={!submitting}
  showCloseButton={true}
  {contentClassName}
  {contentStyle}
  on:openChange={handleOpenChange}
>
  {#if $$slots.subtitle}
    <div class="form-dialog__header">
      <div class="form-dialog__subtitle">
        <slot name="subtitle" {submitting} />
      </div>
    </div>
  {/if}

  {#if bare}
    <slot {submitting} />
  {:else}
    <FormLayout {error} {success} {columns}>
      <slot {submitting} />
    </FormLayout>
  {/if}

  <svelte:fragment slot="actions">
    {#if $$slots.actions}
      <slot name="actions" {submitting} />
    {:else if resolvedShowActions}
      <Button
        variant="ghost"
        size={resolvedSize}
        density={resolvedDensity}
        on:click={handleCancel}
        disabled={submitting}
      >
        {cancelLabel}
      </Button>
      <Button
        variant="primary"
        size={resolvedSize}
        density={resolvedDensity}
        on:click={handleSubmit}
        disabled={submitting}
      >
        {submitting ? "Submitting..." : submitLabel}
      </Button>
    {/if}
  </svelte:fragment>
</Dialog>

<style>
  :global(.form-dialog__surface) {
    width: min(var(--poodle-form-dialog-width, 34rem), 100%);
  }

  .form-dialog__header {
    margin-bottom: var(--poodle-space-stack-md);
  }

  .form-dialog__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size, 0.875rem);
    line-height: var(--poodle-typography-body-lineHeight, 1.5);
  }
</style>

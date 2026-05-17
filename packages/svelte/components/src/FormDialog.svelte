<script lang="ts">
  import type { Snippet } from "svelte";

  import Button from "./Button.svelte";
  import Dialog from "./Dialog.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  import FormLayout from "./FormLayout.svelte";

  interface Props {
    open?: boolean | null | undefined;
    title: string;
    subtitle?: string | null;
    description?: string | null;
    submitLabel?: string;
    cancelLabel?: string;
    submitting?: boolean;
    error?: string | null;
    success?: string | null;
    ariaLabel?: string | null;
    width?: string | null;
    columns?: number;
    showDefaultActions?: boolean;
    bare?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onSubmit?: (() => void) | undefined;
    onCancel?: (() => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
    children?: Snippet<[]>;
    body?: Snippet<[boolean]>;
    actions?: Snippet<[boolean]>;
    subtitleContent?: Snippet<[boolean]>;
  }

  let {
    open = $bindable<boolean | null | undefined>(undefined),
    title,
    subtitle = null,
    description = null,
    submitLabel = "Submit",
    cancelLabel = "Cancel",
    submitting = false,
    error = null,
    success = null,
    ariaLabel = null,
    width = null,
    columns = 6,
    showDefaultActions = true,
    bare = false,
    size = null,
    sizeRole = "control",
    density = null,
    onSubmit = undefined,
    onCancel = undefined,
    onOpenChange = undefined,
    children,
    body,
    actions,
    subtitleContent,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledOpen = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedShowActions = $derived(bare ? false : showDefaultActions);
  const resolvedDescription = $derived(subtitle ?? description);
  const contentStyle = $derived(width ? `--poodle-form-dialog-width: ${width};` : "");
  const contentClassName = $derived(width ? "form-dialog__surface" : "");
  const isControlled = $derived(open !== undefined);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  function handleSubmit(): void {
    onSubmit?.();
  }

  function notifyCancel(): void {
    onCancel?.();
  }

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    onOpenChange?.(nextOpen);
  }

  function handleCancel(): void {
    notifyCancel();
    setOpen(false);
  }

  function handleDialogOpenChange(nextOpen: boolean): void {
    if (!nextOpen && !submitting) {
      notifyCancel();
    }

    setOpen(nextOpen);
  }
</script>

<Dialog
  open={isOpen}
  title={title}
  description={subtitleContent ? null : resolvedDescription}
  role="dialog"
  {ariaLabel}
  size={resolvedSize}
  density={resolvedDensity}
  dismissOnEscape={!submitting}
  dismissOnBackdrop={!submitting}
  showCloseButton={true}
  {contentClassName}
  {contentStyle}
  onOpenChange={handleDialogOpenChange}
>
  {#if subtitleContent}
    {#snippet header()}
      <div class="poodle-form-dialog__header">
        <div class="poodle-form-dialog__subtitle">
          {@render subtitleContent(submitting)}
        </div>
      </div>
    {/snippet}
  {/if}

  {#if bare}
    {#if body}
      {@render body(submitting)}
    {:else}
      {@render children?.()}
    {/if}
  {:else}
    <FormLayout {error} {success} {columns}>
      {#if body}
        {@render body(submitting)}
      {:else}
        {@render children?.()}
      {/if}
    </FormLayout>
  {/if}

  {#snippet actions()}
    {#if actions}
      {@render actions(submitting)}
    {:else if resolvedShowActions}
      <Button
        variant="ghost"
        size={resolvedSize}
        density={resolvedDensity}
        onClick={handleCancel}
        disabled={submitting}
      >
        {cancelLabel}
      </Button>
      <Button
        variant="primary"
        size={resolvedSize}
        density={resolvedDensity}
        onClick={handleSubmit}
        disabled={submitting}
      >
        {submitting ? "Submitting..." : submitLabel}
      </Button>
    {/if}
  {/snippet}
</Dialog>

<style>
  :global(.poodle-form-dialog__surface) {
    width: min(var(--poodle-form-dialog-width, 34rem), 100%);
  }

  .poodle-form-dialog__header {
    margin-bottom: var(--poodle-space-stack-md);
  }

  .poodle-form-dialog__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size, 0.875rem);
    line-height: var(--poodle-typography-body-lineHeight, 1.5);
  }
</style>

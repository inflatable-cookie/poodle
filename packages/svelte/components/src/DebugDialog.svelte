<script lang="ts">
  import { default as Button } from "./Button.svelte";
  import { default as Code } from "./Code.svelte";
  import { default as Dialog } from "./Dialog.svelte";

  import type { ButtonVariant, ControlSize } from "./types";

  let {
    value = null,
    title = "Debug data",
    triggerLabel = "View debug data",
    maxHeight = "min(60vh, 32rem)",
    triggerVariant = "ghost",
    triggerSize = "sm",
    showCloseButton = true,
    closeLabel = "Close debug dialog",
  }: {
    value?: unknown | null;
    title?: string;
    triggerLabel?: string;
    maxHeight?: string;
    triggerVariant?: ButtonVariant;
    triggerSize?: ControlSize | null;
    showCloseButton?: boolean;
    closeLabel?: string;
  } = $props();

  let open = $state(false);

  const hasValue = $derived(value !== null && value !== undefined);
  const source = $derived(stringifyValue(value));

  function stringifyValue(input: unknown): string {
    if (input === null || input === undefined) {
      return "";
    }

    try {
      return JSON.stringify(input, null, 2);
    } catch {
      return String(input);
    }
  }
</script>

{#if hasValue}
  <Button type="button" variant={triggerVariant} size={triggerSize} onClick={() => (open = true)}>
    {triggerLabel}
  </Button>

  <Dialog
    open={open}
    {title}
    width="lg"
    {showCloseButton}
    {closeLabel}
    onOpenChange={(nextOpen) => (open = nextOpen)}
  >
    <Code source={source} language="json" maxHeight={maxHeight} />
  </Dialog>
{/if}

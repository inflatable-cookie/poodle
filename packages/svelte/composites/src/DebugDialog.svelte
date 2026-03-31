<script lang="ts">
  import { Button, Code, Dialog } from "@poodle/svelte-primitives";

  import type { ButtonVariant, ControlSize } from "@poodle/svelte-primitives";

  export let value: unknown | null = null;
  export let title = "Debug data";
  export let triggerLabel = "View debug data";
  export let maxHeight = "min(60vh, 32rem)";
  export let triggerVariant: ButtonVariant = "ghost";
  export let triggerSize: ControlSize | null = "sm";
  export let showCloseButton = true;
  export let closeLabel = "Close debug dialog";

  let open = false;

  $: hasValue = value !== null && value !== undefined;
  $: source = stringifyValue(value);

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
  <Button type="button" variant={triggerVariant} size={triggerSize} on:click={() => (open = true)}>
    {triggerLabel}
  </Button>

  <Dialog
    open={open}
    {title}
    width="lg"
    {showCloseButton}
    {closeLabel}
    on:openChange={(event) => (open = event.detail.open)}
  >
    <Code source={source} language="json" maxHeight={maxHeight} />
  </Dialog>
{/if}

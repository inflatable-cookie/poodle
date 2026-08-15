<script lang="ts">
  import Popover from "../../../../packages/svelte/components/src/Popover.svelte";

  /**
   * Conformance fixture host: mounts the real Svelte Popover from the case
   * fixture and owns the controlled open state the same way a consumer host
   * does (spec 066 harness step 1). Region strings render as the trigger and
   * content snippets; `host.nested` composes a second Popover inside the
   * content (the nested dismiss-stack proof). No expected result is restated.
   */

  interface Props {
    fixture: {
      props: Record<string, unknown>;
      regions: Record<string, string>;
      host?: Record<string, unknown>;
    };
    onOpenChange: (open: boolean) => void;
  }

  let { fixture, onOpenChange }: Props = $props();

  let open = $state<boolean | null>(null);
  let seeded = $state(false);
  const props = $derived({ ...fixture.props, open });

  $effect.pre(() => {
    if (seeded) return;
    open = (fixture.props.open as boolean | null | undefined) ?? null;
    seeded = true;
  });

  function handleOpenChange(next: boolean): void {
    onOpenChange(next);
    if (open !== null) open = next;
  }

  const nested = $derived(
    fixture.host?.nested as { trigger?: string; children?: string } | undefined,
  );
</script>

<Popover {...props} onOpenChange={handleOpenChange}>
  {#snippet trigger()}
    {@html fixture.regions.trigger}
  {/snippet}
  {#snippet children()}
    {@html fixture.regions.children}
    {#if nested}
      <Popover defaultOpen onOpenChange={onOpenChange}>
        {#snippet trigger()}
          {@html nested.trigger ?? ""}
        {/snippet}
        {#snippet children()}
          {@html nested.children ?? ""}
        {/snippet}
      </Popover>
    {/if}
  {/snippet}
</Popover>

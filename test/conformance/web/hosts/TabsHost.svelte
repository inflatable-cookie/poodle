<script lang="ts">
  import Tabs from "../../../../packages/svelte/components/src/Tabs.svelte";

  interface Props {
    fixture: { props: Record<string, unknown>; regions: Record<string, string> };
    onValueChange: (value: string) => void;
  }

  let { fixture, onValueChange }: Props = $props();
  let value = $state<string | null>(null);
  let seeded = $state(false);
  const props = $derived({ ...fixture.props, value });

  $effect.pre(() => {
    if (seeded) return;
    value = (fixture.props.value as string | null | undefined) ?? null;
    seeded = true;
  });

  function handleValueChange(next: string): void {
    onValueChange(next);
    value = next;
  }
</script>

<Tabs {...props} onValueChange={handleValueChange}>
  {#snippet children(activeValue: string)}
    {fixture.regions.panel} · {activeValue}
  {/snippet}
</Tabs>

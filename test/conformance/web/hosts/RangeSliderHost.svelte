<script lang="ts">
  import RangeSlider from "../../../../packages/svelte/components/src/RangeSlider.svelte";

  /**
   * Conformance fixture host: mounts the real Svelte RangeSlider and owns
   * the controlled value the same way a consumer host does.
   */

  interface Props {
    fixture: { props: Record<string, unknown>; regions: Record<string, string> };
    onValueChange: (value: [number, number]) => void;
    onValueCommit: (value: [number, number]) => void;
  }

  let { fixture, onValueChange, onValueCommit }: Props = $props();

  const initialValue = $derived((fixture.props.value as [number, number] | undefined) ?? [0, 100]);
  let value = $state<[number, number]>([0, 100]);
  $effect.pre(() => {
    value = initialValue;
  });
  const props = $derived({ ...fixture.props, value });

  function handleValueChange(next: [number, number]): void {
    onValueChange(next);
    value = next;
  }

  function handleValueCommit(next: [number, number]): void {
    onValueCommit(next);
    value = next;
  }
</script>

<RangeSlider
  {...props}
  onValueChange={handleValueChange}
  onValueCommit={handleValueCommit}
/>

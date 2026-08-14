<script lang="ts">
  import Button from "../../../../packages/svelte/components/src/Button.svelte";

  /**
   * Conformance fixture host: mounts the real Svelte Button from the case
   * fixture and owns the controlled-pressed state the same way a consumer
   * host does (spec 066 harness step 1). No fixture content is restated —
   * props and regions pass through.
   */

  interface Props {
    fixture: { props: Record<string, unknown>; regions: Record<string, string> };
    onPress: () => void;
    onPressedChange: (pressed: boolean) => void;
  }

  let { fixture, onPress, onPressedChange }: Props = $props();

  const initialPressed = $derived((fixture.props.pressed as boolean | null) ?? null);
  let pressed = $state<boolean | null>(null);
  $effect.pre(() => {
    pressed = initialPressed;
  });
  const props = $derived({ ...fixture.props, pressed });

  function handlePressedChange(next: boolean): void {
    onPressedChange(next);
    if (pressed !== null) pressed = next;
  }
</script>

<Button
  {...props}
  leadingIcon={fixture.regions.leading ?? null}
  trailingIcon={fixture.regions.trailing ?? null}
  onClick={onPress}
  onPressedChange={handlePressedChange}
>
  {fixture.regions.label}
</Button>

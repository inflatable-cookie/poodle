<script lang="ts">
  import TextInput from "../../../../packages/svelte/components/src/TextInput.svelte";

  /**
   * Conformance fixture host: mounts the real Svelte TextInput and owns
   * the controlled value the same way a consumer host does.
   */

  interface Props {
    fixture: { props: Record<string, unknown>; regions: Record<string, string> };
    onValueChange: (value: string) => void;
    onSubmit: (value: string) => void;
    onCancel: () => void;
    onClear: () => void;
  }

  let { fixture, onValueChange, onSubmit, onCancel, onClear }: Props = $props();

  const initialValue = $derived((fixture.props.value as string | null | undefined) ?? null);
  let value = $state<string | null>(null);
  $effect.pre(() => {
    value = initialValue;
  });
  const props = $derived({
    ...fixture.props,
    value,
    leadingIcon: fixture.regions.leading ?? fixture.props.leadingIcon ?? null,
    trailingIcon: fixture.regions.trailing ?? fixture.props.trailingIcon ?? null,
  });

  function handleValueChange(next: string): void {
    onValueChange(next);
    value = next;
  }
</script>

<TextInput
  {...props}
  onValueChange={handleValueChange}
  onSubmit={onSubmit}
  onCancel={onCancel}
  onClear={onClear}
/>

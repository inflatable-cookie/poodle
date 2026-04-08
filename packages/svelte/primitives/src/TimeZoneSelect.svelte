<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Select from "./Select.svelte";
  import { defaultTimeZoneOptions } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole, TimeZoneOption } from "./types";

  export let id: string | undefined = undefined;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let placeholder: string | null = "Search time zones...";
  export let options: TimeZoneOption[] = [];
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  $: availableOptions = options.length > 0 ? options : defaultTimeZoneOptions();
  $: selectOptions = availableOptions.map((o) => ({
    value: o.value,
    label: o.label,
    disabled: o.disabled,
  }));
</script>

<Select
  {id}
  {name}
  {value}
  {defaultValue}
  options={selectOptions}
  {placeholder}
  {disabled}
  {ariaLabel}
  {describedBy}
  {size}
  {sizeRole}
  {density}
  searchable
  emptyMessage="No matching time zones"
  on:valueChange={(e) => dispatch("valueChange", e.detail)}
/>

<script lang="ts">
  import { default as Select } from "./Select.svelte";
  import { defaultTimeZoneOptions } from "./date";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, TimeZoneOption } from "./types";

  interface Props {
    id?: string;
    value?: string | null | undefined;
    defaultValue?: string | null;
    placeholder?: string | null;
    options?: TimeZoneOption[];
    disabled?: boolean;
    ariaLabel?: string | null;
    describedBy?: string | null;
    name?: string | undefined;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string) => void) | undefined;
    onQueryChange?: ((query: string) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    id = undefined,
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    placeholder = "Search time zones...",
    options = [],
    disabled = false,
    ariaLabel = null,
    describedBy = null,
    name = undefined,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    onQueryChange = undefined,
    onOpenChange = undefined,
  }: Props = $props();

  const availableOptions = $derived(options.length > 0 ? options : defaultTimeZoneOptions());
  const selectOptions = $derived(availableOptions.map((o) => ({
    value: o.value,
    label: o.label,
    disabled: o.disabled,
  })));
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
  {onValueChange}
  {onQueryChange}
  {onOpenChange}
/>

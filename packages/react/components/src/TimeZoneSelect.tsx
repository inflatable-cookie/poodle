import { defaultTimeZoneOptions } from "@inflatable-cookie/poodle-core";

import { Select } from "./Select";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, TimeZoneOption } from "./types";

export interface TimeZoneSelectProps {
  id?: string;
  value?: string | null;
  defaultValue?: string | null;
  placeholder?: string | null;
  options?: TimeZoneOption[];
  disabled?: boolean;
  ariaLabel?: string | null;
  describedBy?: string | null;
  name?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: string) => void;
  onQueryChange?: (query: string) => void;
  onOpenChange?: (open: boolean) => void;
}

export function TimeZoneSelect({
  id,
  value,
  defaultValue = null,
  placeholder = "Search time zones...",
  options = [],
  disabled = false,
  ariaLabel = null,
  describedBy = null,
  name,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  onQueryChange,
  onOpenChange,
}: TimeZoneSelectProps) {
  const availableOptions = options.length > 0 ? options : defaultTimeZoneOptions();
  const selectOptions = availableOptions.map((o) => ({ value: o.value, label: o.label, disabled: o.disabled }));

  return (
    <Select
      id={id}
      name={name}
      value={value}
      defaultValue={defaultValue}
      options={selectOptions}
      placeholder={placeholder}
      disabled={disabled}
      ariaLabel={ariaLabel}
      describedBy={describedBy}
      size={size}
      sizeRole={sizeRole}
      density={density}
      searchable
      emptyMessage="No matching time zones"
      onValueChange={onValueChange}
      onQueryChange={onQueryChange}
      onOpenChange={onOpenChange}
    />
  );
}

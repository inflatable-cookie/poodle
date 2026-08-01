<script lang="ts">
  import {
    Eyebrow,
    Slider,
    TextInput,
    ThemeSelect,
    ToggleGroup,
    type ToggleGroupOption,
  } from "@poodle/svelte";
  import {
    densityModes,
    controlSizes,
    themeOptions,
  } from "@poodle/svelte-tokens";
  let {
    theme,
    density,
    controlSize,
    search = "",
    onThemeChange = () => {},
    onDensityChange = () => {},
    onControlSizeChange = () => {},
    onSearchChange = () => {},
    contrast = 1,
    onContrastChange = () => {},
  }: {
    theme: string;
    density: string;
    controlSize: string;
    search?: string;
    onThemeChange?: (value: string) => void;
    onDensityChange?: (value: string) => void;
    onControlSizeChange?: (value: string) => void;
    onSearchChange?: (value: string) => void;
    contrast?: number;
    onContrastChange?: (value: number) => void;
  } = $props();
  type DensityName = keyof typeof densityModes;
  type ControlSizeName = keyof typeof controlSizes;

  const densityOrder: DensityName[] = ["compact", "default", "comfortable"];
  const controlSizeOrder: ControlSizeName[] = ["xs", "sm", "md", "lg", "xl"];

  const densityEntries = Object.entries(densityModes) as [DensityName, (typeof densityModes)[DensityName]][];
  const controlSizeEntries = Object.entries(controlSizes) as [ControlSizeName, (typeof controlSizes)[ControlSizeName]][];

  const themeCatalogue = themeOptions();
  const densityOptions: ToggleGroupOption[] = densityOrder
    .filter((name) => densityEntries.some(([entryName]) => entryName === name))
    .map((name) => ({ value: name, label: name }));
  const controlSizeOptions: ToggleGroupOption[] = controlSizeOrder
    .filter((name) => controlSizeEntries.some(([entryName]) => entryName === name))
    .map((name) => ({ value: name, label: name }));
</script>

<div class="poodle-display-controls">
  <div class="poodle-display-controls__group">
    <Eyebrow>Theme</Eyebrow>
    <ThemeSelect
      themes={themeCatalogue}
      value={theme}
      ariaLabel="Theme"
      onChange={(value) => onThemeChange(value)}
    />
  </div>

  <div class="poodle-display-controls__group">
    <Eyebrow>Density</Eyebrow>
    <ToggleGroup
      value={density}
      options={densityOptions}
      ariaLabel="Density"
      onValueChange={(value) => onDensityChange(value as string)}
    />
  </div>

  <div class="poodle-display-controls__group">
    <Eyebrow>Size</Eyebrow>
    <ToggleGroup
      value={controlSize}
      options={controlSizeOptions}
      ariaLabel="Control size"
      onValueChange={(value) => onControlSizeChange(value as string)}
    />
  </div>

  <div class="poodle-display-controls__group">
    <Eyebrow>Contrast</Eyebrow>
    <Slider
      value={contrast}
      min={0.4}
      max={1.6}
      step={0.05}
      ariaLabel="Neutral contrast"
      valueText={`${contrast.toFixed(2)}x`}
      onValueChange={(value) => onContrastChange(value)}
    />
  </div>

  <div class="poodle-display-controls__group poodle-display-controls__group--search">
    <Eyebrow>Search</Eyebrow>
    <TextInput
      type="search"
      placeholder="Find component..."
      value={search}
      ariaLabel="Search components"
      onValueChange={onSearchChange}
      onClear={() => onSearchChange("")}
    />
  </div>
</div>

<style>
  .poodle-display-controls {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 1.25rem 2rem;
    padding: 0.75rem 1rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-color-background-panel);
  }

  .poodle-display-controls__group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .poodle-display-controls__group--search {
    flex: 1;
    min-width: 10rem;
  }
</style>

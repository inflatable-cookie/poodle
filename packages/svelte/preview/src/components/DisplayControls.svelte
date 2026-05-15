<script lang="ts">
  import {
    Eyebrow,
    TextInput,
    ToggleGroup,
    type ToggleGroupOption,
  } from "@poodle/svelte";
  import {
    themes,
    densityModes,
    controlSizes,
  } from "@poodle/svelte-tokens";

  export let theme: string;
  export let density: string;
  export let controlSize: string;
  export let appearanceTreatment: string = "system";
  export let search: string = "";
  export let onThemeChange: (value: string) => void = () => {};
  export let onDensityChange: (value: string) => void = () => {};
  export let onControlSizeChange: (value: string) => void = () => {};
  export let onAppearanceTreatmentChange: (value: string) => void = () => {};
  export let onSearchChange: (value: string) => void = () => {};

  type ThemeName = keyof typeof themes;
  type DensityName = keyof typeof densityModes;
  type ControlSizeName = keyof typeof controlSizes;

  const densityOrder: DensityName[] = ["compact", "default", "comfortable"];
  const controlSizeOrder: ControlSizeName[] = ["xs", "sm", "md", "lg", "xl"];

  const themeEntries = Object.entries(themes) as [ThemeName, (typeof themes)[ThemeName]][];
  const densityEntries = Object.entries(densityModes) as [DensityName, (typeof densityModes)[DensityName]][];
  const controlSizeEntries = Object.entries(controlSizes) as [ControlSizeName, (typeof controlSizes)[ControlSizeName]][];

  const themeOptions: ToggleGroupOption[] = themeEntries.map(([name]) => ({ value: name, label: name }));
  const densityOptions: ToggleGroupOption[] = densityOrder
    .filter((name) => densityEntries.some(([entryName]) => entryName === name))
    .map((name) => ({ value: name, label: name }));
  const controlSizeOptions: ToggleGroupOption[] = controlSizeOrder
    .filter((name) => controlSizeEntries.some(([entryName]) => entryName === name))
    .map((name) => ({ value: name, label: name }));
  const appearanceTreatmentOptions: ToggleGroupOption[] = [
    { value: "system", label: "system" },
    { value: "brand-raised", label: "brand-raised" },
  ];
</script>

<div class="poodle-display-controls">
  <div class="poodle-display-controls__group">
    <Eyebrow>Theme</Eyebrow>
    <ToggleGroup
      value={theme}
      options={themeOptions}
      ariaLabel="Theme"
      onValueChange={(value) => onThemeChange(value as string)}
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
    <Eyebrow>Treatment</Eyebrow>
    <ToggleGroup
      value={appearanceTreatment}
      options={appearanceTreatmentOptions}
      ariaLabel="Appearance treatment"
      onValueChange={(value) => onAppearanceTreatmentChange(value as string)}
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

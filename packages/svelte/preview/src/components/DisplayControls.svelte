<script lang="ts">
  import {
    Checkbox,
    Eyebrow,
    ToggleGroup,
    type ToggleGroupOption,
  } from "@poodle/svelte-primitives";
  import {
    themes,
    densityModes,
    controlSizes,
  } from "@poodle/svelte-tokens";

  export let theme: string;
  export let density: string;
  export let controlSize: string;
  export let appearanceTreatment: string = "system";
  export let disabled = false;
  export let invalid = true;
  export let busy = false;
  export let onThemeChange: (value: string) => void = () => {};
  export let onDensityChange: (value: string) => void = () => {};
  export let onControlSizeChange: (value: string) => void = () => {};
  export let onAppearanceTreatmentChange: (value: string) => void = () => {};
  export let onDisabledChange: (checked: boolean) => void = () => {};
  export let onInvalidChange: (checked: boolean) => void = () => {};
  export let onBusyChange: (checked: boolean) => void = () => {};

  type ThemeName = keyof typeof themes;
  type DensityName = keyof typeof densityModes;
  type ControlSizeName = keyof typeof controlSizes;

  const themeEntries = Object.entries(themes) as [ThemeName, (typeof themes)[ThemeName]][];
  const densityEntries = Object.entries(densityModes) as [DensityName, (typeof densityModes)[DensityName]][];
  const controlSizeEntries = Object.entries(controlSizes) as [ControlSizeName, (typeof controlSizes)[ControlSizeName]][];

  const themeOptions: ToggleGroupOption[] = themeEntries.map(([name]) => ({ value: name, label: name }));
  const densityOptions: ToggleGroupOption[] = densityEntries.map(([name]) => ({ value: name, label: name }));
  const controlSizeOptions: ToggleGroupOption[] = controlSizeEntries.map(([name]) => ({ value: name, label: name }));
  const appearanceTreatmentOptions: ToggleGroupOption[] = [
    { value: "system", label: "system" },
    { value: "brand-raised", label: "brand-raised" },
  ];
</script>

<div class="display-controls">
  <div class="display-controls__group">
    <Eyebrow>Theme</Eyebrow>
    <ToggleGroup
      value={theme}
      options={themeOptions}
      ariaLabel="Theme"
      on:valueChange={(event) => onThemeChange(event.detail.value as string)}
    />
  </div>

  <div class="display-controls__group">
    <Eyebrow>Density</Eyebrow>
    <ToggleGroup
      value={density}
      options={densityOptions}
      ariaLabel="Density"
      on:valueChange={(event) => onDensityChange(event.detail.value as string)}
    />
  </div>

  <div class="display-controls__group">
    <Eyebrow>Size</Eyebrow>
    <ToggleGroup
      value={controlSize}
      options={controlSizeOptions}
      ariaLabel="Control size"
      on:valueChange={(event) => onControlSizeChange(event.detail.value as string)}
    />
  </div>

  <div class="display-controls__group">
    <Eyebrow>Treatment</Eyebrow>
    <ToggleGroup
      value={appearanceTreatment}
      options={appearanceTreatmentOptions}
      ariaLabel="Appearance treatment"
      on:valueChange={(event) => onAppearanceTreatmentChange(event.detail.value as string)}
    />
  </div>

  <div class="display-controls__group display-controls__group--probes">
    <Eyebrow>State probes</Eyebrow>
    <Checkbox
      checked={disabled}
      label="Disabled"
      on:checkedChange={(event) => onDisabledChange(event.detail.checked)}
    />
    <Checkbox
      checked={invalid}
      label="Invalid"
      on:checkedChange={(event) => onInvalidChange(event.detail.checked)}
    />
    <Checkbox
      checked={busy}
      label="Busy"
      on:checkedChange={(event) => onBusyChange(event.detail.checked)}
    />
  </div>
</div>

<style>
  .display-controls {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 1.25rem 2rem;
    padding: 0.75rem 1rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-color-background-panel);
  }

  .display-controls__group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .display-controls__group--probes {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }
</style>

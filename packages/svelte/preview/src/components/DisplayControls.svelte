<script lang="ts">
  import {
    Eyebrow,
    Slider,
    TextInput,
    ThemeSelect,
    ToggleGroup,
    type ToggleGroupOption,
  } from "@inflatable-cookie/poodle-svelte";
  import {
    densityModes,
    controlSizes,
    themeOptions,
  } from "@inflatable-cookie/poodle-core/tokens";
  import { previewShell } from "../generated/preview-shell";

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

  // The control surface is the scene's (card 035 R3/R4): the capability set
  // and label text come from the generated artifact, never authored here.
  // Widget mechanics (which control renders each kind) are this shell's
  // idiom. Kinds are compared as plain strings on purpose: deleting an axis
  // or search from the scene removes the control cleanly — a literal-typed
  // comparison would be a compile error, not a removal.
  type ShellControl = (typeof previewShell)["controls"][number];
  type NamedControl = ShellControl & { values: readonly string[] };
  type RangeControl = ShellControl & { min: number; max: number; default: number };
  type SearchControl = ShellControl & { placeholder: string };

  const themeControl = previewShell.controls.find(
    (control): control is NamedControl => (control.kind as string) === "theme",
  );
  const sizeControl = previewShell.controls.find(
    (control): control is NamedControl => (control.kind as string) === "size",
  );
  const densityControl = previewShell.controls.find(
    (control): control is NamedControl => (control.kind as string) === "density",
  );
  const contrastControl = previewShell.controls.find(
    (control): control is RangeControl => (control.kind as string) === "contrast",
  );
  const searchControl = previewShell.controls.find(
    (control): control is SearchControl => (control.kind as string) === "search",
  );

  const themeCatalogue = themeOptions();
  const themeList = themeControl
    ? themeCatalogue.filter((option) =>
        (themeControl.values as readonly string[]).includes(option.value),
      )
    : [];
  const densityOptions: ToggleGroupOption[] = densityControl
    ? densityControl.values
        .filter((name): name is DensityName => name in densityModes)
        .map((name) => ({ value: name, label: name }))
    : [];
  const controlSizeOptions: ToggleGroupOption[] = sizeControl
    ? sizeControl.values
        .filter((name): name is ControlSizeName => name in controlSizes)
        .map((name) => ({ value: name, label: name }))
    : [];
</script>

<div class="poodle-display-controls">
  {#if themeControl}
    <div class="poodle-display-controls__group">
      <Eyebrow>{themeControl.label}</Eyebrow>
      <ThemeSelect
        themes={themeList}
        value={theme}
        ariaLabel={themeControl.label}
        onChange={(value) => onThemeChange(value)}
      />
    </div>
  {/if}

  {#if densityControl}
    <div class="poodle-display-controls__group">
      <Eyebrow>{densityControl.label}</Eyebrow>
      <ToggleGroup
        value={density}
        options={densityOptions}
        ariaLabel={densityControl.label}
        onValueChange={(value) => onDensityChange(value as string)}
      />
    </div>
  {/if}

  {#if sizeControl}
    <div class="poodle-display-controls__group">
      <Eyebrow>{sizeControl.label}</Eyebrow>
      <ToggleGroup
        value={controlSize}
        options={controlSizeOptions}
        ariaLabel="Control size"
        onValueChange={(value) => onControlSizeChange(value as string)}
      />
    </div>
  {/if}

  {#if contrastControl}
    <div class="poodle-display-controls__group">
      <Eyebrow>{contrastControl.label}</Eyebrow>
      <Slider
        value={contrast}
        min={contrastControl.min}
        max={contrastControl.max}
        step={0.05}
        ariaLabel="Neutral contrast"
        valueText={`${contrast.toFixed(2)}x`}
        onValueChange={(value) => onContrastChange(value)}
      />
    </div>
  {/if}

  {#if searchControl}
    <div class="poodle-display-controls__group poodle-display-controls__group--search">
      <Eyebrow>{searchControl.label}</Eyebrow>
      <TextInput
        type="search"
        placeholder={searchControl.placeholder}
        value={search}
        ariaLabel="Search components"
        onValueChange={onSearchChange}
        onClear={() => onSearchChange("")}
      />
    </div>
  {/if}
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

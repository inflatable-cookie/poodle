import { Eyebrow, Slider, TextInput, ToggleGroup, type ToggleGroupOption } from "@inflatable-cookie/poodle-react";
import { themes, densityModes, controlSizes } from "@inflatable-cookie/poodle-core/tokens";
import { previewShell } from "../generated/preview-shell";

export interface DisplayControlsProps {
  theme: string;
  density: string;
  controlSize: string;
  search?: string;
  contrast?: number;
  onThemeChange?: (value: string) => void;
  onDensityChange?: (value: string) => void;
  onControlSizeChange?: (value: string) => void;
  onSearchChange?: (value: string) => void;
  onContrastChange?: (value: number) => void;
}

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

const themeControl = previewShell.controls.find((control): control is NamedControl => (control.kind as string) === "theme");
const sizeControl = previewShell.controls.find((control): control is NamedControl => (control.kind as string) === "size");
const densityControl = previewShell.controls.find((control): control is NamedControl => (control.kind as string) === "density");
const contrastControl = previewShell.controls.find((control): control is RangeControl => (control.kind as string) === "contrast");
const searchControl = previewShell.controls.find((control): control is SearchControl => (control.kind as string) === "search");

const themeOptions: ToggleGroupOption[] = themeControl
  ? Object.keys(themes)
      .filter((name) => (themeControl.values as readonly string[]).includes(name))
      .map((name) => ({ value: name, label: name }))
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

export function DisplayControls({
  theme,
  density,
  controlSize,
  search = "",
  contrast = 1,
  onThemeChange = () => {},
  onDensityChange = () => {},
  onControlSizeChange = () => {},
  onSearchChange = () => {},
  onContrastChange = () => {},
}: DisplayControlsProps) {
  return (
    <div className="poodle-display-controls">
      {themeControl && (
        <div className="poodle-display-controls__group">
          <Eyebrow>{themeControl.label}</Eyebrow>
          <ToggleGroup
            value={theme}
            options={themeOptions}
            ariaLabel={themeControl.label}
            onValueChange={(value) => onThemeChange(value as string)}
          />
        </div>
      )}

      {densityControl && (
        <div className="poodle-display-controls__group">
          <Eyebrow>{densityControl.label}</Eyebrow>
          <ToggleGroup
            value={density}
            options={densityOptions}
            ariaLabel={densityControl.label}
            onValueChange={(value) => onDensityChange(value as string)}
          />
        </div>
      )}

      {sizeControl && (
        <div className="poodle-display-controls__group">
          <Eyebrow>{sizeControl.label}</Eyebrow>
          <ToggleGroup
            value={controlSize}
            options={controlSizeOptions}
            ariaLabel="Control size"
            onValueChange={(value) => onControlSizeChange(value as string)}
          />
        </div>
      )}

      {contrastControl && (
        <div className="poodle-display-controls__group">
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
      )}

      {searchControl && (
        <div className="poodle-display-controls__group poodle-display-controls__group--search">
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
      )}
    </div>
  );
}

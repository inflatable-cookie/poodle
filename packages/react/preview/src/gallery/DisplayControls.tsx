import { Eyebrow, Slider, TextInput, ToggleGroup, type ToggleGroupOption } from "@inflatable-cookie/poodle-react";
import { themes, densityModes, controlSizes } from "@inflatable-cookie/poodle-svelte-tokens";

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

const densityOrder = ["compact", "default", "comfortable"];
const controlSizeOrder = ["xs", "sm", "md", "lg", "xl"];

const themeOptions: ToggleGroupOption[] = Object.keys(themes).map((name) => ({ value: name, label: name }));
const densityOptions: ToggleGroupOption[] = densityOrder
  .filter((name) => name in densityModes)
  .map((name) => ({ value: name, label: name }));
const controlSizeOptions: ToggleGroupOption[] = controlSizeOrder
  .filter((name) => name in controlSizes)
  .map((name) => ({ value: name, label: name }));

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
      <div className="poodle-display-controls__group">
        <Eyebrow>Theme</Eyebrow>
        <ToggleGroup value={theme} options={themeOptions} ariaLabel="Theme" onValueChange={(value) => onThemeChange(value as string)} />
      </div>

      <div className="poodle-display-controls__group">
        <Eyebrow>Density</Eyebrow>
        <ToggleGroup value={density} options={densityOptions} ariaLabel="Density" onValueChange={(value) => onDensityChange(value as string)} />
      </div>

      <div className="poodle-display-controls__group">
        <Eyebrow>Size</Eyebrow>
        <ToggleGroup value={controlSize} options={controlSizeOptions} ariaLabel="Control size" onValueChange={(value) => onControlSizeChange(value as string)} />
      </div>

      <div className="poodle-display-controls__group">
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

      <div className="poodle-display-controls__group poodle-display-controls__group--search">
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
  );
}

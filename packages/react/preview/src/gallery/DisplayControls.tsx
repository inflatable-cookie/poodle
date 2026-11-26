import {
  Eyebrow,
  Select,
  Slider,
  TextInput,
  ThemeSelect,
  ToggleGroup,
  UiPresentationProvider,
  type ToggleGroupOption,
} from "@inflatable-cookie/poodle-react";
import {
  themeOptions,
  densityModes,
  controlSizes,
  styles,
  type StyleName,
  type ThemeSwatch,
} from "@inflatable-cookie/poodle-core/tokens";
import { previewShell } from "../generated/preview-shell";

export interface DisplayControlsProps {
  theme: string;
  style?: StyleName;
  // The header follows the specimen `controlSize` selection (xs–xl), so every
  // painted control resolves the same shared ladder stop. Density stays
  // ambient too.
  density: DensityName;
  controlSize: ControlSizeName;
  search?: string;
  contrast?: number;
  // Set while the live theme is applied; its swatch then shows the live colours.
  liveSwatch?: ThemeSwatch;
  onLiveTheme?: () => void;
  onThemeChange?: (value: string) => void;
  onStyleChange?: (value: string) => void;
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

const themeCatalogue = themeOptions();
const styleOptions = Object.keys(styles).map((name) => ({ value: name, label: name }));
const themeList = themeControl
  ? themeCatalogue.filter((option) => (themeControl.values as readonly string[]).includes(option.value))
  : [];
const liveThemeChoice = "live";
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
  style = "default",
  density,
  controlSize,
  search = "",
  contrast = 1,
  liveSwatch,
  onLiveTheme,
  onThemeChange = () => {},
  onStyleChange = () => {},
  onDensityChange = () => {},
  onControlSizeChange = () => {},
  onSearchChange = () => {},
  onContrastChange = () => {},
}: DisplayControlsProps) {
  const currentTheme = themeList.find((option) => option.value === theme);
  const themeChoices =
    onLiveTheme && currentTheme
      ? [
          ...themeList,
          {
            value: liveThemeChoice,
            label: "Live theme",
            description: "Edit the live theme settings",
            swatch: liveSwatch ?? currentTheme.swatch,
          },
        ]
      : themeList;

  return (
    <UiPresentationProvider density={density} sizeScale={controlSize}>
      <div className="poodle-display-controls">
        {themeControl && (
          <div className="poodle-display-controls__group">
            <Eyebrow>{themeControl.label}</Eyebrow>
            <ThemeSelect
              value={liveSwatch ? liveThemeChoice : theme}
              themes={themeChoices}
              ariaLabel={themeControl.label}
              onChange={(value) => (value === liveThemeChoice ? onLiveTheme?.() : onThemeChange(value))}
            />
          </div>
        )}

        {styleOptions.length > 1 && (
          <div className="poodle-display-controls__group">
            <Eyebrow>Style</Eyebrow>
            <Select native value={style} options={styleOptions} ariaLabel="Style" onValueChange={onStyleChange} />
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
              ariaLabel={sizeControl.label}
              onValueChange={(value) => onControlSizeChange(value as string)}
            />
          </div>
        )}

        {contrastControl && !liveSwatch && (
          <div className="poodle-display-controls__group">
            <Eyebrow>{contrastControl.label}</Eyebrow>
            <Slider
              value={contrast}
              min={contrastControl.min}
              max={contrastControl.max}
              step={0.05}
              ariaLabel={contrastControl.label}
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
              ariaLabel={searchControl.label}
              onValueChange={onSearchChange}
              onClear={() => onSearchChange("")}
            />
          </div>
        )}
      </div>
    </UiPresentationProvider>
  );
}

import { Fragment, useEffect, useRef } from "react";
import { themePresets } from "@inflatable-cookie/poodle-core";
import { Button, Checkbox, Slider, ToggleGroup } from "@inflatable-cookie/poodle-react";

import { liveThemePreset, type LiveThemeSettings } from "@poodle/svelte-preview/live-theme";

// Mirrors the Svelte preview's LiveThemePanel. The settings and URL names come
// from the shared live-theme module, so both previews read the same URL.
export interface LiveThemePanelProps {
  settings: LiveThemeSettings;
  onChange: (settings: LiveThemeSettings) => void;
  // Changes each time the picker opens the panel; the heading takes focus then.
  focusRequest?: number;
}

const schemes = [
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

const tints = [
  { label: "Base", color: "base", blend: "baseBlend", sample: "#73809a" },
  { label: "Accent", color: "accent", blend: "accentBlend", sample: "#5b9dff" },
] as const;

export function LiveThemePanel({ settings, onChange, focusRequest = 0 }: LiveThemePanelProps) {
  const headingRef = useRef<HTMLHeadingElement | null>(null);

  useEffect(() => {
    if (focusRequest) headingRef.current?.focus();
  }, [focusRequest]);

  function update(patch: Partial<LiveThemeSettings>): void {
    onChange({ ...settings, ...patch });
  }

  return (
    <section className="live-theme-panel" aria-labelledby="live-theme-settings">
      <h2 id="live-theme-settings" tabIndex={-1} ref={headingRef}>
        Live theme settings
      </h2>

      <div className="settings">
        <div className="setting">
          <span>Scheme</span>
          <ToggleGroup
            value={settings.scheme}
            options={schemes}
            ariaLabel="Scheme"
            onValueChange={(scheme) => {
              if (scheme === "light" || scheme === "dark") update({ scheme });
            }}
          />
        </div>

        {tints.map(({ label, color, blend, sample }) => (
          <Fragment key={color}>
            <div className="setting">
              <label htmlFor={`live-theme-${color}`}>{label} colour</label>
              <div className="colour-choice">
                <input
                  id={`live-theme-${color}`}
                  type="color"
                  value={settings[color] === "mono" ? "#808080" : settings[color]}
                  onChange={(event) => update({ [color]: event.currentTarget.value })}
                />
                <Button
                  fit="content"
                  ariaLabel={`${label} mono`}
                  pressed={settings[color] === "mono"}
                  onPressedChange={(mono) => update({ [color]: mono ? "mono" : sample })}
                >
                  Mono
                </Button>
              </div>
            </div>
            <div className="setting range">
              <span className="range-label">
                {label} blend <span aria-hidden="true">{Math.round(settings[blend] * 100)}%</span>
              </span>
              <Slider
                variant="embedded"
                value={settings[blend]}
                min={0}
                max={1}
                step={0.01}
                ariaLabel={`${label} blend`}
                valueText={`${Math.round(settings[blend] * 100)}%`}
                onValueChange={(value) => update({ [blend]: value })}
              />
            </div>
          </Fragment>
        ))}

        <div className="setting range">
          <span className="range-label">
            Contrast <span aria-hidden="true">{settings.contrast.toFixed(2)}x</span>
          </span>
          <Slider
            variant="embedded"
            value={settings.contrast}
            min={0.5}
            max={2}
            step={0.01}
            ariaLabel="Contrast"
            valueText={`${settings.contrast.toFixed(2)}x`}
            onValueChange={(contrast) => update({ contrast })}
          />
        </div>

        <Checkbox
          checked={settings.evenTint}
          label="Even tint"
          onCheckedChange={(evenTint) => update({ evenTint })}
        />
      </div>

      <div className="presets" role="group" aria-label="Live theme presets">
        <span>Presets</span>
        {themePresets.map((preset) => (
          <Button
            key={preset[0]}
            fit="content"
            variant="ghost"
            onClick={() => onChange(liveThemePreset(preset, settings.evenTint))}
          >
            {preset[1]}
          </Button>
        ))}
      </div>
    </section>
  );
}

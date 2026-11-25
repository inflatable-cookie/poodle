<script lang="ts">
  import { themePresets } from "@inflatable-cookie/poodle-core";
  import { Button, Checkbox, Slider, ToggleGroup } from "@inflatable-cookie/poodle-svelte";

  import { liveThemePreset, type LiveThemeSettings } from "../live-theme";

  let { settings, onChange }: {
    settings: LiveThemeSettings;
    onChange: (settings: LiveThemeSettings) => void;
  } = $props();

  const schemes = [
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" },
  ];

  const tints = [
    { label: "Base", color: "base", blend: "baseBlend", sample: "#73809a" },
    { label: "Accent", color: "accent", blend: "accentBlend", sample: "#5b9dff" },
  ] as const;

  function update(patch: Partial<LiveThemeSettings>): void {
    onChange({ ...settings, ...patch });
  }
</script>

<section class="live-theme-panel" aria-labelledby="live-theme-settings">
  <h2 id="live-theme-settings" tabindex="-1">Live theme settings</h2>

  <div class="settings">
    <div class="setting">
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

    {#each tints as { label, color, blend, sample } (color)}
      <div class="setting">
        <label for={`live-theme-${color}`}>{label} colour</label>
        <div class="colour-choice">
          <input
            id={`live-theme-${color}`}
            type="color"
            value={settings[color] === "mono" ? "#808080" : settings[color]}
            oninput={(event) => update({ [color]: event.currentTarget.value })}
          />
          <Button
            fit="content"
            ariaLabel={`${label} mono`}
            pressed={settings[color] === "mono"}
            onPressedChange={(mono) => update({ [color]: mono ? "mono" : sample })}
          >Mono</Button>
        </div>
      </div>
      <div class="setting range">
        <span class="range-label">
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
    {/each}

    <div class="setting range">
      <span class="range-label">
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

  <div class="presets" role="group" aria-label="Live theme presets">
    <span>Presets</span>
    {#each themePresets as preset (preset[0])}
      <Button fit="content" variant="ghost" onClick={() => onChange(liveThemePreset(preset, settings.evenTint))}>
        {preset[1]}
      </Button>
    {/each}
  </div>
</section>

<style>
  .live-theme-panel {
    display: grid;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-color-background-panel);
  }

  h2 {
    margin: 0;
    font-size: var(--poodle-typography-heading-size);
    font-weight: var(--poodle-typography-heading-weight);
  }

  .settings, .presets {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    align-items: end;
  }

  .presets, .colour-choice {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .presets > span {
    margin-right: 0.5rem;
  }

  .setting {
    display: grid;
    gap: 0.25rem;
  }

  .setting, .presets > span {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-label-size);
  }

  .range {
    min-width: 8rem;
  }

  .range-label {
    display: flex;
    justify-content: space-between;
    gap: 0.5rem;
    font-variant-numeric: tabular-nums;
  }

  input[type="color"] {
    box-sizing: border-box;
    width: var(--poodle-size-control-height);
    height: var(--poodle-size-control-height);
    padding: 0.125rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-fill-rest);
    cursor: pointer;
  }

  input[type="color"]:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: var(--poodle-focus-outline-offset);
  }
</style>

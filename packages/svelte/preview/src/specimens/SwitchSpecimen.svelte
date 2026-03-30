<script lang="ts">
  import { Switch, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let darkMode = true;
  let autoSave = false;
  let compactView = true;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Switch
      label="Dark mode"
      checked={darkMode}
      on:checkedChange={(e) => (darkMode = e.detail.checked)}
    />
    <Switch
      label="Auto-save drafts"
      checked={autoSave}
      on:checkedChange={(e) => (autoSave = e.detail.checked)}
    />
    <Switch
      label="Compact view"
      checked={compactView}
      on:checkedChange={(e) => (compactView = e.detail.checked)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <Switch label="Enabled" {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Switch id={"density-" + density} label="Toggle" {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <Switch label="Disabled off" disabled />
    <Switch label="Disabled on" checked={true} disabled />
    <Switch label="Read-only on" checked={true} readOnly />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom colors</Eyebrow>
    <Switch
      label="Billing alerts"
      checked={true}
      onColor="#22c55e"
      offColor="#cbd5e1"
    />
    <Switch
      label="Quiet mode"
      checked={false}
      onColor="#f59e0b"
      offColor="#94a3b8"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Dual labels and tones</Eyebrow>
    <Switch
      checked={true}
      leftLabel="Draft"
      rightLabel="Live"
      leftTone="danger"
      rightTone="success"
      ariaLabel="Publication status"
    />
    <Switch
      checked={false}
      leftLabel="Restricted"
      rightLabel="Free"
      leftTone="warning"
      rightTone="success"
      ariaLabel="Access status"
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>

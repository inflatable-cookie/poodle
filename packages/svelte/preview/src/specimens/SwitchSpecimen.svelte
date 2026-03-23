<script lang="ts">
  import { Switch, Eyebrow } from "@poodle/svelte-primitives";

  let darkMode = true;
  let autoSave = false;
  let compactView = true;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Switch
      label="Dark mode"
      isChecked={darkMode}
      on:checkedChange={(e) => (darkMode = e.detail.checked)}
    />
    <Switch
      label="Auto-save drafts"
      isChecked={autoSave}
      on:checkedChange={(e) => (autoSave = e.detail.checked)}
    />
    <Switch
      label="Compact view"
      isChecked={compactView}
      on:checkedChange={(e) => (compactView = e.detail.checked)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <Switch label="Disabled off" isDisabled />
    <Switch label="Disabled on" isChecked={true} isDisabled />
    <Switch label="Read-only on" isChecked={true} isReadOnly />
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
</style>

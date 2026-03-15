<script lang="ts">
  import { EditableLabel, Eyebrow } from "@pug/svelte-primitives";

  let title = "Untitled project";
  let subtitle = "Click to rename";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Double-click to edit (default)</Eyebrow>
    <EditableLabel
      value={title}
      ariaLabel="Project title"
      on:commit={(e) => (title = e.detail.value)}
    />
    <p>Current value: <strong>{title}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Enter/Space activation</Eyebrow>
    <EditableLabel
      value={subtitle}
      activationMode="enterOrSpace"
      ariaLabel="Subtitle"
      on:commit={(e) => (subtitle = e.detail.value)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <EditableLabel value="Locked label" isDisabled ariaLabel="Disabled label" />
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
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>

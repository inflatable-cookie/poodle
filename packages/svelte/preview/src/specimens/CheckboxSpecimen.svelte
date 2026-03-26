<script lang="ts">
  import { Checkbox, Eyebrow } from "@poodle/svelte-primitives";

  let notifications = true;
  let marketing = false;
  let terms = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Checkbox
      label="Enable email notifications"
      checked={notifications}
      on:checkedChange={(e) => (notifications = e.detail.checked)}
    />
    <Checkbox
      label="Subscribe to marketing emails"
      checked={marketing}
      on:checkedChange={(e) => (marketing = e.detail.checked)}
    />
    <Checkbox
      label="I agree to the terms and conditions"
      checked={terms}
      on:checkedChange={(e) => (terms = e.detail.checked)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <Checkbox label="Disabled unchecked" disabled />
    <Checkbox label="Disabled checked" checked={true} disabled />
    <Checkbox label="Mixed / indeterminate" mixed />
    <Checkbox label="Read-only checked" checked={true} readOnly />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom selected color</Eyebrow>
    <Checkbox label="Billable feature" checked={true} selectedColor="#22c55e" />
    <Checkbox label="Requires moderation" checked={true} selectedColor="#f59e0b" />
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

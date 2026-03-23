<script lang="ts">
  import { Checkbox, Eyebrow } from "@flint/svelte-primitives";

  let notifications = true;
  let marketing = false;
  let terms = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Checkbox
      label="Enable email notifications"
      isChecked={notifications}
      on:checkedChange={(e) => (notifications = e.detail.checked)}
    />
    <Checkbox
      label="Subscribe to marketing emails"
      isChecked={marketing}
      on:checkedChange={(e) => (marketing = e.detail.checked)}
    />
    <Checkbox
      label="I agree to the terms and conditions"
      isChecked={terms}
      on:checkedChange={(e) => (terms = e.detail.checked)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <Checkbox label="Disabled unchecked" isDisabled />
    <Checkbox label="Disabled checked" isChecked={true} isDisabled />
    <Checkbox label="Mixed / indeterminate" isMixed />
    <Checkbox label="Read-only checked" isChecked={true} isReadOnly />
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

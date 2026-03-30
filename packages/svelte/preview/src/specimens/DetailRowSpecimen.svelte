<script lang="ts">
  import { DetailRow, Eyebrow, Button } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic label-value pairs</Eyebrow>
    <DetailRow label="Name" value="Poodle Design System" />
    <DetailRow label="Version" value="2.1.0" />
    <DetailRow label="License" value="MIT" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With description</Eyebrow>
    <DetailRow label="API endpoint" value="https://api.example.com/v2" description="Base URL for all API requests." truncateValue />
  </div>

  <div class="specimen__group">
    <Eyebrow>With action slot</Eyebrow>
    <DetailRow label="Email" value="clay@example.com">
      <svelte:fragment slot="action">
        <Button variant="secondary">Change</Button>
      </svelte:fragment>
    </DetailRow>
  </div>

  <div class="specimen__group">
    <Eyebrow>With value slot</Eyebrow>
    <DetailRow label="Status">
      <svelte:fragment slot="value">
        <span class="status-badge">Active</span>
      </svelte:fragment>
    </DetailRow>
  </div>

  <div class="specimen__group">
    <Eyebrow>Stacked layout</Eyebrow>
    <DetailRow
      label="Arrangement"
      value="2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A..."
      truncateValue
      layout="stacked"
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
    gap: 0.375rem;
  }

  .status-badge {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    border-radius: 999px;
    background: var(--poodle-color-bg-success-subtle, #2d4a2d);
    color: var(--poodle-color-text-success, #8fbc8f);
    font-size: 0.75rem;
    font-weight: 500;
  }
</style>

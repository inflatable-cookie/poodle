<script lang="ts">
  import { Eyebrow, Surface } from "@poodle/svelte";
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="specimen">
    <div class="specimen__example">
      <Eyebrow>Section label</Eyebrow>
      <h3 class="demo-heading">Page Title</h3>
      <p>Eyebrow renders small uppercase text used for categorizing content above headings.</p>
    </div>

    <div class="specimen__example">
      <Eyebrow>Primitive</Eyebrow>
      <h3 class="demo-heading">Button</h3>
      <p>Primary interactive control for triggering actions.</p>
    </div>

    <div class="specimen__example">
      <Eyebrow>Status</Eyebrow>
      <h3 class="demo-heading">Active deployment</h3>
      <p>Last deployed 3 minutes ago.</p>
    </div>
  </div>
</Surface>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__example {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .demo-heading {
    margin: 0;
    font-size: 1.25rem;
    color: var(--poodle-color-text-primary);
  }

  p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
    line-height: 1.5;
  }
</style>

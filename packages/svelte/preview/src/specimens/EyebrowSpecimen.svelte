<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Section label</Eyebrow>
    <p>Eyebrow renders small uppercase text used for categorizing content above headings.</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Primitive</Eyebrow>
    <h3 class="demo-heading">Button</h3>
    <p>Primary interactive control for triggering actions.</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Composite</Eyebrow>
    <h3 class="demo-heading">DataTable</h3>
    <p>Feature-rich table with sorting, selection, and pagination.</p>
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
    gap: 0.25rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }

  .demo-heading {
    margin: 0;
    font-size: 1.25rem;
    color: var(--pug-color-text-primary);
  }
</style>

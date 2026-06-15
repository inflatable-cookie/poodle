<script lang="ts">
  import { Eyebrow, Surface } from "@poodle/svelte";
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="poodle-specimen">
    <div class="poodle-specimen__example">
      <Eyebrow>Section label</Eyebrow>
      <h3 class="poodle-demo-heading">Page Title</h3>
      <p>Eyebrow renders small uppercase text used for categorizing content above headings.</p>
    </div>

    <div class="poodle-specimen__example">
      <Eyebrow>Primitive</Eyebrow>
      <h3 class="poodle-demo-heading">Button</h3>
      <p>Primary interactive control for triggering actions.</p>
    </div>

    <div class="poodle-specimen__example">
      <Eyebrow>Status</Eyebrow>
      <h3 class="poodle-demo-heading">Active deployment</h3>
      <p>Last deployed 3 minutes ago.</p>
    </div>

    <div class="poodle-specimen__example">
      <Eyebrow as="h3" size="md" spacing="bottom">Semantic section heading</Eyebrow>
      <p>Eyebrow can render as a heading when it labels a real subsection.</p>
    </div>
  </div>
</Surface>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .poodle-specimen__example {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .poodle-demo-heading {
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

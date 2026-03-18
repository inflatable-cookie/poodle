<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { PanelHeader } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <div class="specimen__frame">
      <PanelHeader title="Explorer" isActive />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With collapse</Eyebrow>
    <div class="specimen__frame">
      <PanelHeader title="Search Results" isActive isCollapsible />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Inactive</Eyebrow>
    <div class="specimen__frame">
      <PanelHeader title="Output" isActive={false} />
    </div>
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

  .specimen__frame {
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }
</style>

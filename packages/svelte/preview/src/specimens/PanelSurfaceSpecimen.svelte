<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { PanelSurface } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Standard variant</Eyebrow>
    <div class="specimen__frame">
      <PanelSurface title="Standard Panel" variant="standard" bodyPadding="md">
        <p class="specimen__text">Standard panel content with medium padding.</p>
      </PanelSurface>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Utility variant</Eyebrow>
    <div class="specimen__frame">
      <PanelSurface title="Utility Panel" variant="utility" bodyPadding="sm">
        <p class="specimen__text">Compact utility panel with reduced chrome.</p>
      </PanelSurface>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Focused variant</Eyebrow>
    <div class="specimen__frame">
      <PanelSurface title="Focused Panel" variant="focused" isActive bodyPadding="md">
        <p class="specimen__text">Focused panel with accent border and active state.</p>
      </PanelSurface>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Elevated</Eyebrow>
    <div class="specimen__frame">
      <PanelSurface title="Elevated Panel" isElevated bodyPadding="md">
        <p class="specimen__text">Elevated panel with shadow.</p>
      </PanelSurface>
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
    height: 8rem;
  }

  .specimen__text {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
  }
</style>

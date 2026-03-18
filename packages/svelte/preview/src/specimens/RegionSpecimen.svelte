<script lang="ts">
  import { Region, Eyebrow, Stack } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Region label="Content area" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom colors</Eyebrow>
    <Stack gap="sm">
      <Region label="Header" color="#5b9bd5" minHeight="3rem" />
      <Region label="Sidebar" color="#70ad47" minHeight="6rem" />
      <Region label="Main content" color="#ed7d31" minHeight="8rem" />
      <Region label="Footer" color="#a855f7" minHeight="3rem" />
    </Stack>
  </div>

  <div class="specimen__group">
    <Eyebrow>Layout composition</Eyebrow>
    <div class="specimen__layout">
      <Region label="Nav" color="#5b9bd5" minHeight="100%" />
      <div class="specimen__layout-main">
        <Region label="Toolbar" color="#70ad47" minHeight="2.5rem" />
        <Region label="Content" color="#ed7d31" minHeight="10rem" />
      </div>
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

  .specimen__layout {
    display: grid;
    grid-template-columns: 10rem 1fr;
    gap: 0.5rem;
    min-height: 14rem;
  }

  .specimen__layout-main {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>

<script lang="ts">
  import { Region, Stack } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <Region label="Content area" />
  </SpecimenGroup>

  <SpecimenGroup label="Custom colors">
    <Stack gap="sm">
      <Region label="Header" color="#5b9bd5" minHeight="3rem" />
      <Region label="Sidebar" color="#70ad47" minHeight="6rem" />
      <Region label="Main content" color="#ed7d31" minHeight="8rem" />
      <Region label="Footer" color="#a855f7" minHeight="3rem" />
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Layout composition">
    <div class="specimen__layout">
      <Region label="Nav" color="#5b9bd5" minHeight="100%" />
      <div class="specimen__layout-main">
        <Region label="Toolbar" color="#70ad47" minHeight="2.5rem" />
        <Region label="Content" color="#ed7d31" minHeight="10rem" />
      </div>
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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

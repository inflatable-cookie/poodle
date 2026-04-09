<script lang="ts">
  import { Surface } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Panel tone (default)">
    <Surface padding="md" border="subtle">
      <p>Panel surface with subtle border — the standard container.</p>
    </Surface>
  </SpecimenGroup>

  <SpecimenGroup label="Canvas tone">
    <Surface tone="canvas" padding="md" border="subtle">
      <p>Canvas surface sits behind panels as a background layer.</p>
    </Surface>
  </SpecimenGroup>

  <SpecimenGroup label="Elevated tone">
    <Surface tone="elevated" padding="md" border="subtle" elevated>
      <p>Elevated surface with shadow for overlays and cards.</p>
    </Surface>
  </SpecimenGroup>

  <SpecimenGroup label="No border">
    <Surface padding="md" border="none">
      <p>Surface with no border — just padding and background.</p>
    </Surface>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>

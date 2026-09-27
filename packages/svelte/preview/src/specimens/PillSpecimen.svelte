<script lang="ts">
  import { Pill } from "@inflatable-cookie/poodle-svelte";
  import SceneSpecimen from "../components/SceneSpecimen.svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let { slug = "pill" }: { slug?: string } = $props();

  const dismissRowStyle = "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;";
</script>

<SceneSpecimen {slug}>
  <SpecimenGroup label="Dismissible">
    <div style={dismissRowStyle}>
      <Pill dismissible dismissLabel="Remove filter: Videos">Videos</Pill>
      <Pill dismissible>Audio</Pill>
      <Pill tone="info" appearance="subtle" dismissible dismissLabel="Remove filter: Published">
        Published
      </Pill>
    </div>
  </SpecimenGroup>
</SceneSpecimen>

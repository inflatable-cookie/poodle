<script lang="ts">
  import SceneSpecimen from "../src/components/SceneSpecimen.svelte";

  // Drives the "one SceneSpecimen instance, changing slug" scenario the preview
  // gallery uses: Svelte patches the same component (and its SpecimenLayout)
  // with the new slug prop rather than remounting.
  let slug = $state("callout");
</script>

<button class="scene-switch" onclick={() => (slug = "avatar")}>navigate to avatar</button>
<SceneSpecimen {slug} />
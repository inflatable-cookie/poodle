<script lang="ts">
  import SceneSpecimen from "../src/components/SceneSpecimen.svelte";

  let { slug }: { slug: string } = $props();
</script>

<SceneSpecimen {slug} />
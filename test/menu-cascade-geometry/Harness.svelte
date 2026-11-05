<script lang="ts">
  import Menu from "../../packages/svelte/components/src/Menu.svelte";
  import { items, geometry, action } from "./items";
</script>
<section data-framework="svelte"><div class="scroller"><div class="spacer"></div><div class="host">
<Menu {items} placement="top-end" triggerAriaLabel="Options svelte" ariaLabel="Root svelte" onAction={action} onSurfaceGeometryChange={geometry}>
{#snippet trigger()}Options{/snippet}
</Menu></div><div class="tail"></div></div></section>

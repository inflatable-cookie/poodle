<script lang="ts">
  import AppHeader from "../src/AppHeader.svelte";

  // Renders the bound element's identity into the DOM so the test can observe
  // what `bind:element` actually resolved to.
  let element = $state<HTMLElement | null>(null);
</script>

<AppHeader bind:element={element} title="Finch" />
<output data-testid="bound-element">{element?.tagName} {element?.className}</output>

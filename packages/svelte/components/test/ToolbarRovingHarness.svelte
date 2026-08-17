<script lang="ts">
  import Toolbar from "../src/Toolbar.svelte";
  import type { Orientation } from "../src/types";

  let { orientation = "horizontal" }: { orientation?: Orientation } = $props();
</script>

<!-- Real focusable children so roving focus has elements to move between. -->
<Toolbar ariaLabel="Formatting toolbar" {orientation}>
  <button>One</button>
  <button>Two</button>
  <button>Three</button>
</Toolbar>
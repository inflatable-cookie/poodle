<script lang="ts">
  import type { IconRegistry } from "./icon-registry";

  import { setIconRegistry } from "./icon-registry";

  export let registry: IconRegistry;

  setIconRegistry(registry);
</script>

<slot />

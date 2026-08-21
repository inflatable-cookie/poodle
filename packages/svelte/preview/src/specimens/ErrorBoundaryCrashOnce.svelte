<script module lang="ts">
  const thrownTokens = new WeakSet<object>();
</script>

<script lang="ts">
  import { Text } from "@inflatable-cookie/poodle-svelte";

  let { token }: { token: object } = $props();

  if (!thrownTokens.has(token)) {
    thrownTokens.add(token);
    throw new Error("Preview child failed during render.");
  }
</script>

<Text>Recovered child content.</Text>

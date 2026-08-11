<script lang="ts">
  import AppHeader from "../src/AppHeader.svelte";

  let { center = false, actions = true, utility = true }: { center?: boolean; actions?: boolean; utility?: boolean } = $props();
</script>

<!-- Renders AppHeader with idiomatic snippet syntax so the tests can assert
     the exact region markup. Snippet props are only supplied when enabled. -->
<AppHeader
  title="Finch"
  center={center ? centerContent : undefined}
  actions={actions ? actionsContent : undefined}
  utility={utility ? utilityContent : undefined}
>
</AppHeader>

{#snippet centerContent()}Centre{/snippet}
{#snippet actionsContent()}New{/snippet}
{#snippet utilityContent()}Settings{/snippet}

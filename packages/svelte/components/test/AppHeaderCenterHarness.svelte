<script lang="ts">
  import AppHeader from "../src/AppHeader.svelte";

  let { center = false, actions = true, utility = true }: { center?: boolean; actions?: boolean; utility?: boolean } = $props();
</script>

<!-- Renders AppHeader with idiomatic snippet syntax so the tests can assert
     the exact region markup. Snippet props are only supplied when enabled.
     `never` crosses the duplicate-Svelte type identity created by the packed
     install smoke; snippets have no runtime conversion. -->
<AppHeader
  title="Finch"
  center={center ? (centerContent as never) : undefined}
  actions={actions ? (actionsContent as never) : undefined}
  utility={utility ? (utilityContent as never) : undefined}
>
</AppHeader>

{#snippet centerContent()}Centre{/snippet}
{#snippet actionsContent()}New{/snippet}
{#snippet utilityContent()}Settings{/snippet}

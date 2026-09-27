<script lang="ts">
  import ListCard from "../src/ListCard.svelte";

  // Renders an `eyebrowContent` snippet so tests can prove it takes
  // precedence over the plain `eyebrow` string.
</script>

<ListCard title="msa-2026.pdf" eyebrow="Ignored fallback">
  {#snippet eyebrowContent()}
    <span class="harness-eyebrow">Contracts · rich</span>
  {/snippet}
</ListCard>

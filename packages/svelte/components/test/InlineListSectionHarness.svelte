<script lang="ts">
  import InlineListSection from "../src/InlineListSection.svelte";

  type Version = { id: string; label: string };

  let {
    title = "Versions",
    items = [] as Version[],
    count = null as number | string | null,
    framed = true,
  } = $props<{
    title?: string;
    items?: Version[];
    count?: number | string | null;
    framed?: boolean;
  }>();
</script>

<InlineListSection {title} {items} {count} {framed}>
  {#snippet item(entry: Version)}
    <span class="harness-version">{entry.label}</span>
  {/snippet}
</InlineListSection>

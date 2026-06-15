<script lang="ts">
  import { IconButton, InlineListSection, Pill, Text } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const versions = [
    { id: "v3", name: "Version 3", meta: "Ready" },
    { id: "v2", name: "Version 2", meta: "Archived" },
    { id: "v1", name: "Version 1", meta: "Archived" },
  ];
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Framed related list">
    <InlineListSection title="Versions" items={versions} count={versions.length}>
      {#snippet actions()}
        <IconButton icon="plus" ariaLabel="Add version" variant="secondary" />
      {/snippet}

      {#snippet item(version)}
        <div class="poodle-row">
          <Text as="span" weight="medium">{version.name}</Text>
          <Pill size="sm" tone={version.meta === "Ready" ? "success" : "neutral"}>{version.meta}</Pill>
        </div>
      {/snippet}
    </InlineListSection>
  </SpecimenGroup>

  <SpecimenGroup label="Empty">
    <InlineListSection
      title="Aliases"
      items={[]}
      emptyMessage="No aliases yet."
    >
      {#snippet item(alias)}
        <Text>{alias}</Text>
      {/snippet}
    </InlineListSection>
  </SpecimenGroup>

  <SpecimenGroup label="Unframed">
    <InlineListSection title="References" items={versions.slice(0, 2)} framed={false}>
      {#snippet item(version)}
        <Text as="span">{version.name}</Text>
      {/snippet}
    </InlineListSection>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    width: 100%;
    min-width: 0;
  }
</style>

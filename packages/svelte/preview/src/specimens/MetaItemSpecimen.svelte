<script lang="ts">
  import { Code, MetaBar, MetaItem, Pill, TextLink } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let linkAction = $state("No link clicked yet.");
</script>

<SpecimenGroup label="Labelled metadata">
  <MetaBar ariaLabel="Project metadata">
    <MetaItem label="ID">
      <Code inline source="proj_01JX9G9NVV1W3M4P6K8Q8T2D5A" showCopyButton />
    </MetaItem>
    <MetaItem label="Owner">Clay</MetaItem>
    <MetaItem label="Updated">2 hours ago</MetaItem>
  </MetaBar>
</SpecimenGroup>

<SpecimenGroup label="Unlabelled and rich value content">
  <MetaBar showSeparators={false}>
    <MetaItem label="Type">Media</MetaItem>
    <Pill tone="neutral">Public</Pill>
    <MetaItem>1920 × 1080</MetaItem>
  </MetaBar>
</SpecimenGroup>

<SpecimenGroup label="Interactive child content">
  <MetaBar>
    <MetaItem label="Docs">
      <TextLink href="#components/meta-item" onClick={() => (linkAction = "Opened MetaItem docs")}>
        View contract
      </TextLink>
    </MetaItem>
    <MetaItem label="Status">
      <Pill tone="success">Stable</Pill>
    </MetaItem>
  </MetaBar>
  <p class="poodle-specimen__hint">{linkAction}</p>
</SpecimenGroup>

<style>
  .poodle-specimen__hint {
    margin: 0.75rem 0 0;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

<script lang="ts">
  import { Code } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const tsExample = `import { Button } from "@poodle/svelte-primitives";

function handleClick(event: MouseEvent): void {
  console.log("Button clicked", event);
}`;

  const cssExample = `.button {
  display: inline-flex;
  align-items: center;
  border-radius: var(--poodle-radius-control);
  background: var(--poodle-color-accent-base);
}`;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Block with language label">
    <Code source={tsExample} language="typescript" />
  </SpecimenGroup>

  <SpecimenGroup label="With line numbers and highlight">
    <Code source={tsExample} language="ts" showLineNumbers highlightLines={[3, 4]} />
  </SpecimenGroup>

  <SpecimenGroup label="CSS with max height">
    <Code source={cssExample} language="css" maxHeight="6rem" />
  </SpecimenGroup>

  <SpecimenGroup label="Inline code">
    <p>Use <Code source="npm install" inline /> to install dependencies.</p>
  </SpecimenGroup>

  <SpecimenGroup label="No copy button">
    <Code source="echo 'hello world'" language="bash" showCopyButton={false} />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Code source="const x = 1;" />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Code source="const x = 1;" />
  </svelte:fragment>
</SpecimenLayout>

<style>
  p { margin: 0; color: var(--poodle-color-text-primary); }
</style>

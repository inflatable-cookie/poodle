<script lang="ts">
  import { Code } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

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

<div class="specimen">
  <SpecimenGroup label="Block with language label">
    <Code source={tsExample} language="typescript" />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Code source="const x = 1;" {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Code source="const x = 1;" {density} />
        </div>
      {/each}
    </div>
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
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__row { display: flex; align-items: center; gap: 0.5rem; }
  .specimen__label { font-size: 0.75rem; font-family: var(--poodle-typography-code-family); color: var(--poodle-color-text-muted); min-width: 6rem; }
  p { margin: 0; color: var(--poodle-color-text-primary); }
</style>

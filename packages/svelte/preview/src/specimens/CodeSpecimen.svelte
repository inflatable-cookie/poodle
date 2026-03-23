<script lang="ts">
  import { Code, Eyebrow } from "@poodle/svelte-primitives";

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
  <div class="specimen__group">
    <Eyebrow>Block with language label</Eyebrow>
    <Code source={tsExample} language="typescript" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With line numbers and highlight</Eyebrow>
    <Code source={tsExample} language="ts" showLineNumbers highlightLines={[3, 4]} />
  </div>

  <div class="specimen__group">
    <Eyebrow>CSS with max height</Eyebrow>
    <Code source={cssExample} language="css" maxHeight="6rem" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Inline code</Eyebrow>
    <p>Use <Code source="npm install" inline /> to install dependencies.</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>No copy button</Eyebrow>
    <Code source="echo 'hello world'" language="bash" showCopyButton={false} />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  p { margin: 0; color: var(--poodle-color-text-primary); }
</style>

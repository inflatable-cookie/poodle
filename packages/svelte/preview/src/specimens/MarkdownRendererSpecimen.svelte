<script lang="ts">
  import { MarkdownRenderer } from "@inflatable-cookie/poodle-svelte/markdown";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const safeDocument = `# Release notes

Rendered as ordinary document content with **bold**, *italic*, and \`inline code\`.

## Highlights

- Safe output is the default
- Markdown links keep their \`href\`
- Lists, quotes, and code blocks keep their prose styling

> Policy never changes the visual treatment. Safe and trusted look the same.

See the [component contract](/docs/contracts/components/markdown-editor) for more.`;

  // Only the explicit `htmlPolicy="trusted"` prop keeps this markup. The
  // specimen labels that policy visibly so it cannot be mistaken for the
  // default rendering posture.
  const trustedDocument = `<div class="trusted-document" data-trusted-document=""><h2>Trusted HTML</h2><p>This <mark>markup</mark> survives only through the explicit trusted policy.</p></div>`;

  const customSource = "Custom parser section";

  const constrainedDocument =
    "A very long unbroken token: https://example.com/an/extremely/long/unbroken/path/that/would/otherwise/overflow/a/constrained/container\n\nParagraph text still wraps normally.";

  function customRender(markdown: string): string {
    return `<h2>${markdown}</h2><p>Custom <strong>renderHtml</strong> output still passes through the safe policy.</p>`;
  }
</script>

<SpecimenLayout>
  {#snippet children()}
    <SpecimenGroup label="Safe document (default policy)">
      <MarkdownRenderer value={safeDocument} />
    </SpecimenGroup>

    <SpecimenGroup label="Trusted document — explicit htmlPolicy=trusted">
      <p class="markdown-renderer-policy-warning" data-policy-label="trusted">
        <code>htmlPolicy="trusted"</code> renders unsanitized HTML. Use it only for fully trusted,
        caller-owned content.
      </p>
      <MarkdownRenderer value={trustedDocument} htmlPolicy="trusted" />
    </SpecimenGroup>

    <SpecimenGroup label="Custom parser (still safe)">
      <MarkdownRenderer value={customSource} renderHtml={customRender} />
    </SpecimenGroup>

    <SpecimenGroup label="Empty document">
      <MarkdownRenderer value="" />
    </SpecimenGroup>

    <SpecimenGroup label="Constrained width">
      <div style="max-width: 18rem">
        <MarkdownRenderer value={constrainedDocument} />
      </div>
    </SpecimenGroup>
  {/snippet}

  {#snippet densities(density)}
    <SpecimenGroup label={density}>
      <MarkdownRenderer value={safeDocument} {density} />
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>

<style>
  .markdown-renderer-policy-warning {
    margin: 0 0 0.5rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }
</style>

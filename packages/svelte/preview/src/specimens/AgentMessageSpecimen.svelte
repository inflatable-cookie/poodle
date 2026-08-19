<script lang="ts">
  import { AgentMessage } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const inline = "The `lexer` is **strict** but *forgiving*, see [the docs](https://example.com/md). Also ~~gone~~ kept.";
  const headings = "# One\n\n## Two\n\n### Three\n\n#### Four\n\n##### Five\n\n###### Six";
  const fenced = "```rust\nfn main() {\n    println!(\"hi\");\n}\n```";
  const unfenced = "```\nno language given\n```";
  const tight = "- alpha\n- beta\n- gamma";
  const loose = "- alpha\n\n- beta";
  const ordered = "3. three\n4. four";
  const nested = "- outer\n  - inner one\n  - inner two\n- second";
  const itemWithFence = "1. run this:\n\n   ```sh\n   bun test\n   ```";
  const quote = "> quoted **line**\n> continued";
  const rule = "before\n\n---\n\nafter";
  // Outside the supported subset. It must degrade to text, not vanish — an
  // agent explaining HTML has to keep the explanation.
  const unsupported = "A table:\n\n| a | b |\n| - | - |\n| 1 | 2 |\n\nAnd raw <div>markup</div>.";
  const long =
    "The latest fixes hold: 41 parser tests pass. AO415 now matches in text and structure; its only remaining delta is an extra CSS class on `<sup>`. RO418 is down to one space inserted after inline math at a source line break. Both are narrow compatibility rules, not parsing failures.";
</script>

<SpecimenLayout>
  <SpecimenGroup
    label="Assistant and user messages"
    description="A user turn sits on the subtle surface; an assistant turn has no container chrome. Long answers stay at the prose measure."
  >
    <AgentMessage markdown="The latest parser fixes hold." />
    <AgentMessage markdown="Can you run the parity sweep again?" role="user" />
    <AgentMessage markdown={long} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Inline formatting and headings"
    description="Structure, not text: code spans, emphasis, links and strikethrough all survive the block model. Headings are real heading elements, so the message is navigable by heading."
  >
    <AgentMessage markdown={inline} />
    <AgentMessage markdown={headings} />
  </SpecimenGroup>

  <SpecimenGroup label="Code blocks" description="An unannotated fence reports no language rather than an empty one.">
    <AgentMessage markdown={fenced} />
    <AgentMessage markdown={unfenced} />
  </SpecimenGroup>

  <SpecimenGroup
    label="List structures"
    description="Tight and loose both normalise to paragraph-wrapped items — that is what removes tight-vs-loose as a source of native divergence."
  >
    <AgentMessage markdown={tight} />
    <AgentMessage markdown={loose} />
    <AgentMessage markdown={ordered} />
    <AgentMessage markdown={nested} />
    <AgentMessage markdown={itemWithFence} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Quotes, rules and fallback"
    description="Tables and raw HTML degrade to text. Silently losing content is the worst available failure for a transcript."
  >
    <AgentMessage markdown={quote} />
    <AgentMessage markdown={rule} />
    <AgentMessage markdown={unsupported} />
  </SpecimenGroup>

  <SpecimenGroup label="Streaming" description="The caret is aria-hidden: it is a progress hint, not content.">
    <AgentMessage markdown="Regenerating the corpus against the cached oracle" isStreaming />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentMessage markdown={`Size ${size}: prose measure and type scale move together.`} {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentMessage markdown={`Density ${density}\n\n- one\n- two`} {density} />
  {/snippet}
</SpecimenLayout>

import { AgentMessage } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const inline =
  "The `lexer` is **strict** but *forgiving*, see [the docs](https://example.com/md). Also ~~gone~~ kept.";
const headings = "# One\n\n### Three\n\n###### Six";
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

export function AgentMessageSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <AgentMessage markdown={`Size ${size}: prose measure and type scale move together.`} size={size} />
      )}
      densities={(density) => (
        <AgentMessage markdown={`Density ${density}\n\n- one\n- two`} density={density} />
      )}
    >
      <SpecimenGroup label="Inline markup">
        <AgentMessage markdown={inline} />
      </SpecimenGroup>

      <SpecimenGroup label="Headings">
        <AgentMessage markdown={headings} />
      </SpecimenGroup>

      <SpecimenGroup label="Code blocks">
        <AgentMessage markdown={fenced} />
        <AgentMessage markdown={unfenced} />
      </SpecimenGroup>

      <SpecimenGroup label="Lists">
        <AgentMessage markdown={tight} />
        <AgentMessage markdown={loose} />
        <AgentMessage markdown={ordered} />
        <AgentMessage markdown={nested} />
        <AgentMessage markdown={itemWithFence} />
      </SpecimenGroup>

      <SpecimenGroup label="Quotes and rules">
        <AgentMessage markdown={quote} />
        <AgentMessage markdown={rule} />
      </SpecimenGroup>

      <SpecimenGroup label="Outside the subset">
        <AgentMessage markdown={unsupported} />
      </SpecimenGroup>

      <SpecimenGroup label="Streaming">
        <AgentMessage markdown="Regenerating the corpus against the cached oracle" isStreaming />
      </SpecimenGroup>

      <SpecimenGroup label="Roles">
        <AgentMessage markdown="Can you run the parity sweep again?" role="user" />
        <AgentMessage markdown={long} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
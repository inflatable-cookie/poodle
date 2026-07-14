import type { CSSProperties } from "react";
import { Code } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const tsExample = `import { Button } from "@poodle/svelte";

function handleClick(event: MouseEvent): void {
  console.log("Button clicked", event);
}`;

const cssExample = `.button {
  display: inline-flex;
  align-items: center;
  border-radius: var(--poodle-radius-control);
  background: var(--poodle-color-accent-base);
}`;

const paragraphStyle: CSSProperties = { margin: 0, color: "var(--poodle-color-text-primary)" };

export function CodeSpecimen() {
  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => <Code source="const x = 1;" size={size} />}
      densities={(density) => <Code source="const x = 1;" density={density} />}
    >
      <SpecimenGroup bare label="Block with language label">
        <Code source={tsExample} language="typescript" />
      </SpecimenGroup>

      <SpecimenGroup bare label="With line numbers and highlight">
        <Code source={tsExample} language="ts" showLineNumbers highlightLines={[3, 4]} />
      </SpecimenGroup>

      <SpecimenGroup bare label="CSS with max height">
        <Code source={cssExample} language="css" maxHeight="6rem" />
      </SpecimenGroup>

      <SpecimenGroup label="Inline code">
        <p style={paragraphStyle}>Use <Code source="npm install" inline /> to install dependencies.</p>
      </SpecimenGroup>

      <SpecimenGroup bare label="No copy button">
        <Code source="echo 'hello world'" language="bash" showCopyButton={false} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

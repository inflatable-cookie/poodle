import type { CSSProperties } from "react";
import { Eyebrow } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const example: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.25rem",
};

const heading: CSSProperties = {
  margin: 0,
  fontSize: "1.25rem",
  color: "var(--poodle-color-text-primary)",
};

const paragraph: CSSProperties = {
  margin: 0,
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.875rem",
  lineHeight: 1.5,
};

export function EyebrowSpecimen() {
  const sizes = (size: string) => <Eyebrow size={size as "xs" | "sm" | "md"}>Section label</Eyebrow>;

  return (
    <SpecimenLayout sizeValues={["xs", "sm", "md"]} sizes={sizes}>
      <SpecimenGroup label="Above a page title">
        <div style={example}>
          <Eyebrow>Section label</Eyebrow>
          <h3 style={heading}>Page Title</h3>
          <p style={paragraph}>Eyebrow renders small uppercase text used for categorizing content above headings.</p>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Primitive category">
        <div style={example}>
          <Eyebrow>Primitive</Eyebrow>
          <h3 style={heading}>Button</h3>
          <p style={paragraph}>Primary interactive control for triggering actions.</p>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Composite category">
        <div style={example}>
          <Eyebrow>Composite</Eyebrow>
          <h3 style={heading}>DataTable</h3>
          <p style={paragraph}>Feature-rich table with sorting, selection, and pagination.</p>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Semantic heading">
        <div style={example}>
          <Eyebrow as="h3" size="md" spacing="bottom">
            Semantic section heading
          </Eyebrow>
          <p style={paragraph}>Eyebrow can render as a heading when it labels a real subsection.</p>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

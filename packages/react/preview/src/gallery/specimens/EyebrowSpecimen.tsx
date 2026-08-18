import type { CSSProperties } from "react";
import { Eyebrow, type ControlSize } from "@inflatable-cookie/poodle-react";
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
  // Eyebrow's typographic scale stops at `md`; only its own steps render.
  const sizes = (size: ControlSize) =>
    size === "xs" || size === "sm" || size === "md" ? <Eyebrow size={size}>Section label</Eyebrow> : null;

  return (
    <SpecimenLayout sizes={sizes}>
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

      <SpecimenGroup label="Status ribbon">
        <div style={example}>
          <Eyebrow>Status</Eyebrow>
          <h3 style={heading}>Active deployment</h3>
          <p style={paragraph}>Last deployed 3 minutes ago.</p>
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

import type { CSSProperties } from "react";
import { HoverCard } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const triggerLink: CSSProperties = {
  color: "var(--poodle-color-text-accent)",
  textDecoration: "underline",
  cursor: "pointer",
  fontSize: "0.875rem",
};

const cardContent: CSSProperties = { padding: "0.5rem", maxWidth: "16rem" };
const cardTitle: CSSProperties = { display: "block", marginBottom: "0.25rem" };
const cardBody: CSSProperties = {
  margin: 0,
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
};

export function HoverCardSpecimen() {
  return (
    <div className="poodle-specimen" style={{ gap: "2rem" }}>
      <SpecimenGroup label="Default (top placement)">
        <HoverCard ariaLabel="User preview" trigger={<span style={triggerLink}>@clay</span>}>
          <div style={cardContent}>
            <strong style={cardTitle}>Clay</strong>
            <p style={cardBody}>
              Design systems engineer working on Poodle. Loves component architecture and accessibility.
            </p>
          </div>
        </HoverCard>
      </SpecimenGroup>

      <SpecimenGroup label="Bottom placement">
        <HoverCard
          placement="bottom"
          ariaLabel="Repository info"
          trigger={<span style={triggerLink}>poodle/svelte-primitives</span>}
        >
          <div style={cardContent}>
            <strong style={cardTitle}>svelte-primitives</strong>
            <p style={cardBody}>
              Core primitive components for the Poodle design system. 64 components, 94% coverage.
            </p>
          </div>
        </HoverCard>
      </SpecimenGroup>
    </div>
  );
}

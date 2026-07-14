import type { CSSProperties } from "react";
import { Text, TextLink } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const rowStyle: CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  gap: "1rem",
};

export function TextLinkSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Inline prose">
        <Text>
          Review the <TextLink href="#contract">component contract</TextLink> before wiring the production route.
        </Text>
      </SpecimenGroup>

      <SpecimenGroup label="Tones">
        <div style={rowStyle}>
          <TextLink href="#accent">Accent link</TextLink>
          <TextLink href="#secondary" tone="secondary">Secondary link</TextLink>
          <TextLink href="#inherit" tone="inherit">Inherited link</TextLink>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Button action">
        <TextLink onClick={() => undefined}>Open inline action</TextLink>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={rowStyle}>
          <TextLink href="#disabled" disabled>Disabled anchor</TextLink>
          <TextLink disabled>Disabled action</TextLink>
        </div>
      </SpecimenGroup>
    </div>
  );
}

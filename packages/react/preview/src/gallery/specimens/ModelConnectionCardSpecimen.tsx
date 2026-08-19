import type { CSSProperties } from "react";
import {
  Icon,
  IconButton,
  ModelCatalogueEditor,
  ModelConnectionCard,
  Pill,
  UpdateCenter,
} from "@inflatable-cookie/poodle-react";
import {
  MODEL_CATALOGUE_FIXTURES,
  MODEL_CONNECTION_CARD_FIXTURES,
} from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const [work, personal, codex, anthropic, ollama] = MODEL_CONNECTION_CARD_FIXTURES;

const updateOffer = {
  status: { kind: "ready" },
  availability: {
    state: "offer",
    version: "1.4.0",
    reason: "staged",
    notes: "Faster renders, a rebuilt automation pass, and two crash fixes.",
  },
} as const;

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const cardStackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "0.75rem" };
const narrowStyle: CSSProperties = { width: "min(18rem, 100%)" };
const noteStyle: CSSProperties = { margin: "0 0 0.75rem", fontSize: "0.875rem", opacity: 0.75 };

function cardProps(fixture: (typeof MODEL_CONNECTION_CARD_FIXTURES)[number]) {
  return {
    id: fixture.id,
    title: fixture.title,
    providerLabel: fixture.providerLabel,
    routeLabel: fixture.routeLabel,
    version: fixture.version,
    accessSummary: fixture.accessSummary,
    readiness: fixture.readiness,
    readinessLabel: fixture.readinessLabel,
    isEnabled: fixture.enabled,
  };
}

export function ModelConnectionCardSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Ready and enabled">
          <p style={noteStyle}>
            Two configured connections of one provider. They differ only by instance
            label and opaque id; the second is switched off by host preference, not
            by readiness.
          </p>
          <div style={cardStackStyle}>
            <ModelConnectionCard {...cardProps(work)} />
            <ModelConnectionCard {...cardProps(personal)} />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Readiness and preference states">
          <div style={cardStackStyle}>
            <ModelConnectionCard {...cardProps(codex)} />
            <ModelConnectionCard {...cardProps(anthropic)} />
            <ModelConnectionCard {...cardProps(ollama)} />
            {/* The whole card is inert; readiness copy stays readable. */}
            <ModelConnectionCard {...cardProps(work)} isDisabled />
            {/* Only the enable Switch is locked; the card still opens. */}
            <ModelConnectionCard {...cardProps(codex)} isEnableDisabled />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Host mark, badges, actions, and closed accessory">
          <div style={cardStackStyle}>
            <ModelConnectionCard
              {...cardProps(work)}
              leading={() => <Icon name="star" />}
              badges={() => (
                <Pill tone="info" appearance="subtle">
                  Preview
                </Pill>
              )}
              actions={() => (
                <IconButton
                  icon="ellipsis"
                  variant="ghost"
                  ariaLabel="More actions for OpenAI · Work"
                />
              )}
            />
            <ModelConnectionCard
              {...cardProps(work)}
              closedAccessory={() => <UpdateCenter presence="attention" {...updateOffer} />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Open details with catalogue">
          <ModelConnectionCard
            {...cardProps(work)}
            defaultOpen
            details={() => <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} />}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Narrow summary wrapping">
          <div style={narrowStyle}>
            <ModelConnectionCard {...cardProps(anthropic)} />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

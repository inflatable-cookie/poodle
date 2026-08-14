import type { CSSProperties } from "react";
import {
  ModelCatalogueEditor,
  ModelConnectionCard,
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
          <ModelConnectionCard {...cardProps(work)} />
        </SpecimenGroup>

        <SpecimenGroup label="Ready and disabled">
          <ModelConnectionCard {...cardProps(personal)} />
        </SpecimenGroup>

        <SpecimenGroup label="Checking">
          <ModelConnectionCard {...cardProps(codex)} />
        </SpecimenGroup>

        <SpecimenGroup label="Needs attention">
          <ModelConnectionCard {...cardProps(anthropic)} />
        </SpecimenGroup>

        <SpecimenGroup label="Unavailable">
          <ModelConnectionCard {...cardProps(ollama)} />
        </SpecimenGroup>

        <SpecimenGroup label="Two OpenAI instances">
          <div style={cardStackStyle}>
            <ModelConnectionCard {...cardProps(work)} />
            <ModelConnectionCard {...cardProps(personal)} />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Closed UpdateCenter accessory">
          <ModelConnectionCard
            {...cardProps(work)}
            closedAccessory={() => <UpdateCenter presence="attention" {...updateOffer} />}
          />
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

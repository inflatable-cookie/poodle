import type { CSSProperties } from "react";
import { ModelConnectionPicker } from "@inflatable-cookie/poodle-react";
import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const options = MODEL_CONNECTION_PICKER_FIXTURES;

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const panelStyle: CSSProperties = { width: "min(42rem, 100%)" };
const narrowStyle: CSSProperties = { width: "min(20rem, 100%)" };
const noteStyle: CSSProperties = { margin: 0, fontSize: "0.875rem", opacity: 0.75 };

export function ModelConnectionPickerSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Grouped catalogue (many providers, one provider with several routes)">
          <div style={panelStyle}>
            <ModelConnectionPicker options={options} defaultValue="openai-responses" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Availability: available, checking, unavailable, unsupported">
          <p style={noteStyle}>
            Each posture appears on its matching option in the grouped catalogue above.
          </p>
        </SpecimenGroup>

        <SpecimenGroup label="Query with results">
          <div style={panelStyle}>
            <ModelConnectionPicker
              options={options}
              defaultQuery="anthropic"
              defaultValue="anthropic-messages"
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Query with no results">
          <div style={panelStyle}>
            <ModelConnectionPicker options={options} defaultQuery="zzzznothing" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Loading">
          <div style={panelStyle}>
            <ModelConnectionPicker options={options} state="loading" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Error">
          <div style={panelStyle}>
            <ModelConnectionPicker options={options} state="error" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Empty catalogue">
          <div style={panelStyle}>
            <ModelConnectionPicker options={[]} state="empty" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Narrow layout">
          <div style={narrowStyle}>
            <ModelConnectionPicker options={options} defaultValue="ollama-local" />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

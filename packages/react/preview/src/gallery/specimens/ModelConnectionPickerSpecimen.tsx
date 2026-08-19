import type { CSSProperties } from "react";
import { Icon, ModelConnectionPicker } from "@inflatable-cookie/poodle-react";
import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const options = MODEL_CONNECTION_PICKER_FIXTURES;

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const panelStyle: CSSProperties = { width: "min(42rem, 100%)" };
const groupStackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "1rem" };
const narrowStyle: CSSProperties = { width: "min(20rem, 100%)" };
const noteStyle: CSSProperties = { margin: "0 0 0.75rem", fontSize: "0.875rem", opacity: 0.75 };

export function ModelConnectionPickerSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Grouped catalogue">
          <p style={noteStyle}>
            Many providers, one of them with several routes. Every availability
            posture — available, checking, unavailable, unsupported — appears on its
            matching option here.
          </p>
          <div style={panelStyle}>
            <ModelConnectionPicker options={options} defaultValue="openai-responses" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Search results">
          <div style={groupStackStyle}>
            <div style={panelStyle}>
              <ModelConnectionPicker
                options={options}
                defaultQuery="anthropic"
                defaultValue="anthropic-messages"
              />
            </div>
            <div style={panelStyle}>
              <ModelConnectionPicker options={options} defaultQuery="zzzznothing" />
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Catalogue states and host lock">
          <div style={groupStackStyle}>
            <div style={panelStyle}>
              <ModelConnectionPicker options={options} state="loading" />
            </div>
            <div style={panelStyle}>
              <ModelConnectionPicker options={options} state="error" />
            </div>
            <div style={panelStyle}>
              <ModelConnectionPicker options={[]} state="empty" />
            </div>
            {/* The catalogue is fine; the host has locked search and options. */}
            <div style={panelStyle}>
              <ModelConnectionPicker
                options={options}
                defaultValue="openai-responses"
                isDisabled
              />
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Host provider marks and footer">
          <p style={noteStyle}>
            A host mark keyed by option id. The generic connection icon stays on
            every option the host did not name.
          </p>
          <div style={panelStyle}>
            <ModelConnectionPicker
              options={options}
              defaultValue="ollama-local"
              leading={({ option }) =>
                option.id === "ollama-local" ? <Icon name="terminal" /> : null
              }
              footer={() => "Connections are managed by the host application."}
            />
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

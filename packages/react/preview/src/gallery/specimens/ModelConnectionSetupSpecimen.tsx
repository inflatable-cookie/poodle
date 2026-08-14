import { useState, type CSSProperties } from "react";
import { Button, Field, ModelConnectionSetup, TextInput } from "@inflatable-cookie/poodle-react";
import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const options = MODEL_CONNECTION_PICKER_FIXTURES;
const interactiveOptions = options.map((option) =>
  option.id === "codex-app"
    ? { ...option, availability: "available" as const, availabilityLabel: "Available", isDisabled: false }
    : option,
);

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const panelStyle: CSSProperties = { width: "min(42rem, 100%)" };

function ApiKeyConfigure() {
  const [apiKey, setApiKey] = useState("");

  return (
    <Field id="mcs-api-key" label="API key">
      <TextInput
        id="mcs-api-key"
        type="password"
        value={apiKey}
        placeholder="sk-demo-placeholder"
        onValueChange={setApiKey}
      />
    </Field>
  );
}

function LocalEndpointConfigure() {
  const [endpoint, setEndpoint] = useState("http://127.0.0.1:11434");

  return (
    <Field id="mcs-endpoint" label="Endpoint URL">
      <TextInput
        id="mcs-endpoint"
        value={endpoint}
        placeholder="http://127.0.0.1:11434"
        onValueChange={setEndpoint}
      />
    </Field>
  );
}

export function ModelConnectionSetupSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Choose stage">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={interactiveOptions}
              defaultValue="openai-responses"
              canSubmit={true}
              configuration={({ option }) => {
                if (option.id === "codex-app") return null;
                if (option.id === "ollama-local") return <LocalEndpointConfigure />;
                if (option.id === "anthropic-messages") {
                  return <Button variant="secondary">Sign in with browser</Button>;
                }
                return <ApiKeyConfigure />;
              }}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Configure: API key">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="openai-responses"
              canSubmit={true}
              configuration={() => <ApiKeyConfigure />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Auto-detect: found">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="codex-app"
              canSubmit={true}
              success="Local harness detected."
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Auto-detect: missing">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="codex-app"
              error="Codex app not found on this machine."
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="OAuth pending">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="anthropic-messages"
              isPending={true}
              pendingLabel="Waiting for browser sign-in"
              configuration={() => <Button variant="secondary">Sign in with browser</Button>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Local endpoint">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="ollama-local"
              canSubmit={true}
              configuration={() => <LocalEndpointConfigure />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Validation failure">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="openai-responses"
              canSubmit={false}
              error="API key format is invalid."
              configuration={() => (
                <Field id="mcs-invalid-key" label="API key">
                  <TextInput
                    id="mcs-invalid-key"
                    type="password"
                    value="••••••••"
                    placeholder="sk-demo-placeholder"
                    readOnly
                  />
                </Field>
              )}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Pending submit">
          <div style={panelStyle}>
            <ModelConnectionSetup
              options={options}
              defaultStage="configure"
              defaultValue="openai-responses"
              canSubmit={true}
              isPending={true}
              pendingLabel="Checking connection"
              configuration={() => (
                <Field id="mcs-pending-key" label="API key">
                  <TextInput
                    id="mcs-pending-key"
                    type="password"
                    value="••••••••"
                    placeholder="sk-demo-placeholder"
                    readOnly
                  />
                </Field>
              )}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

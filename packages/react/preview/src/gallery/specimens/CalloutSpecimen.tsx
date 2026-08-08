import { useState, type CSSProperties } from "react";
import { Callout } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const actionStyle: CSSProperties = {
  minHeight: 0,
  padding: "0.375rem 0.625rem",
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-control)",
  background: "transparent",
  color: "var(--poodle-color-text-primary)",
  font: "inherit",
  cursor: "pointer",
};

export function CalloutSpecimen() {
  const [dismissed, setDismissed] = useState(false);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <Callout tone="info" size={size} title={`Callout at ${size}`}>
          Text and icon chrome scale with the size prop.
        </Callout>
      )}
      densities={(density) => (
        <Callout tone="info" density={density} title={`Callout at ${density} density`}>
          Internal spacing adjusts with the density prop.
        </Callout>
      )}
    >
      <SpecimenGroup label="Tones" bare>
        <Callout tone="neutral" title="Neutral callout">
          This is a general informational message with no specific severity.
        </Callout>
        <Callout tone="info" title="Info">
          Your changes have been saved and will take effect on next deploy.
        </Callout>
        <Callout tone="success" title="Success">
          All tests passed. The build is ready for production.
        </Callout>
        <Callout tone="warning" title="Warning">
          This API key expires in 7 days. Rotate it to avoid service interruption.
        </Callout>
        <Callout tone="danger" title="Error">
          Unable to connect to the database. Check your credentials and try again.
        </Callout>
      </SpecimenGroup>

      <SpecimenGroup label="Message prop" bare>
        <Callout tone="info" title="Information" message="This is an informational callout using the message prop instead of slot content." />
      </SpecimenGroup>

      <SpecimenGroup label="Dismissible" bare>
        {!dismissed ? (
          <Callout
            tone="info"
            title="Dismissible callout"
            message="This callout can be dismissed by the user."
            dismissible
            onDismiss={() => setDismissed(true)}
          />
        ) : (
          <Callout tone="success" message="Dismiss callback fired." />
        )}
      </SpecimenGroup>

      <SpecimenGroup label="Without title" bare>
        <Callout tone="info">
          A simple inline callout without a title for brief contextual notes.
        </Callout>
      </SpecimenGroup>

      <SpecimenGroup label="With actions" bare>
        <Callout
          tone="warning"
          title="Quota warning"
          message="API usage is approaching the current workspace limit."
          actions={<button type="button" style={actionStyle}>Review limits</button>}
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { useState } from "react";
import { Button, ErrorBoundary, Surface, Text } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import {
  armErrorBoundaryCrash,
  ErrorBoundaryCrashOnce,
  getErrorBoundaryCrashEpoch,
} from "./ErrorBoundaryCrashOnce";

export function ErrorBoundarySpecimen() {
  const [crashEpoch, setCrashEpoch] = useState(getErrorBoundaryCrashEpoch());

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Normal children">
        <ErrorBoundary>
          <Surface border="subtle" padding="md">
            <Text>Stable child content renders without boundary chrome.</Text>
          </Surface>
        </ErrorBoundary>
      </SpecimenGroup>

      <SpecimenGroup label="Caught render error">
        <div style={{ marginBottom: "0.75rem" }}>
          <Button
            variant="secondary"
            size="sm"
            onClick={() => {
              armErrorBoundaryCrash();
              setCrashEpoch(getErrorBoundaryCrashEpoch());
            }}
          >
            Throw again
          </Button>
        </div>
        <ErrorBoundary title="Preview failed" retryLabel="Reset boundary">
          <ErrorBoundaryCrashOnce key={crashEpoch} />
        </ErrorBoundary>
      </SpecimenGroup>
    </div>
  );
}

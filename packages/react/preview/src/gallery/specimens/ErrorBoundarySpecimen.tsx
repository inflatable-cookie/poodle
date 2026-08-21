import { useMemo, useState } from "react";
import { Button, ErrorBoundary, Surface, Text } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { ErrorBoundaryCrashOnce } from "./ErrorBoundaryCrashOnce";

export function ErrorBoundarySpecimen() {
  const [crashKey, setCrashKey] = useState(0);
  const crashToken = useMemo(() => ({}), [crashKey]);

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
          <Button variant="secondary" size="sm" onClick={() => setCrashKey((key) => key + 1)}>
            Throw again
          </Button>
        </div>
        <ErrorBoundary title="Preview failed" retryLabel="Reset boundary">
          <ErrorBoundaryCrashOnce key={crashKey} token={crashToken} />
        </ErrorBoundary>
      </SpecimenGroup>
    </div>
  );
}

import { useState, type ReactNode } from "react";
import { Button, ErrorBoundary, Surface, Text } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

// React error boundaries only catch errors thrown by a descendant component
// during its own render, so the Svelte inline `{throwRenderError()}` becomes a
// tiny child component that throws when rendered.
function ThrowingChild(): ReactNode {
  throw new Error("Preview child failed during render.");
}

export function ErrorBoundarySpecimen() {
  const [shouldThrow, setShouldThrow] = useState(true);

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
          <Button variant="secondary" size="sm" onClick={() => setShouldThrow(true)}>
            Throw again
          </Button>
        </div>
        <ErrorBoundary title="Preview failed" retryLabel="Reset boundary">
          {shouldThrow ? <ThrowingChild /> : <Text>Recovered child content.</Text>}
        </ErrorBoundary>
      </SpecimenGroup>
    </div>
  );
}

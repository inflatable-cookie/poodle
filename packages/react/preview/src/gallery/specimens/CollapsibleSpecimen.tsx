import { Collapsible } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function CollapsibleSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <Collapsible title={`Collapsible at ${size}`} size={size}>
          <p>Content at <strong>{size}</strong> size.</p>
        </Collapsible>
      )}
      densities={(density) => (
        <Collapsible title={`Collapsible at ${density} density`} density={density}>
          <p>Content at <strong>{density}</strong> density.</p>
        </Collapsible>
      )}
    >
      <SpecimenGroup bare label="Default (closed)">
        <Collapsible title="Project settings" description="Configure build options and deploy targets.">
          <p>Build target: production</p>
          <p>Output directory: dist/</p>
          <p>Source maps: enabled</p>
        </Collapsible>
      </SpecimenGroup>

      <SpecimenGroup bare label="Default open">
        <Collapsible title="Advanced options" defaultOpen>
          <p>Cache TTL: 3600s</p>
          <p>Retry count: 3</p>
          <p>Timeout: 30s</p>
        </Collapsible>
      </SpecimenGroup>

      <SpecimenGroup bare label="Disabled">
        <Collapsible title="Locked section" description="Requires admin access." disabled>
          <p>This content is hidden behind a disabled collapsible.</p>
        </Collapsible>
      </SpecimenGroup>

      <SpecimenGroup bare label="Highlighted">
        <Collapsible title="Focused section" highlighted defaultOpen>
          <p>Highlighted collapsibles can draw attention to a matched or focused section.</p>
        </Collapsible>
      </SpecimenGroup>

      <SpecimenGroup bare label="Custom trigger">
        <Collapsible
          defaultOpen
          ariaLabel="Toggle custom section"
          trigger={({ isOpen }) => (
            <div>
              <strong>Deployment details</strong>
              <p>{isOpen ? "Expanded" : "Collapsed"} state with custom heading content.</p>
            </div>
          )}
        >
          <p>Environment: production</p>
          <p>Region: eu-west-1</p>
        </Collapsible>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

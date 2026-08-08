import { EmptyState, Button, Icon } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

export function EmptyStateSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Neutral">
        <EmptyState
          title="No projects yet"
          message="Create your first project to get started."
          actions={<Button>Create project</Button>}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Search">
        <EmptyState
          variant="search"
          title="No results found"
          message="Try adjusting your search terms or clearing filters."
          actions={<Button variant="secondary">Clear filters</Button>}
        />
      </SpecimenGroup>

      <SpecimenGroup label="First run">
        <EmptyState
          variant="firstRun"
          title="Welcome to your workspace"
          message="This is where your team's components will appear once you start building."
        />
      </SpecimenGroup>

      <SpecimenGroup label="Compact custom visual">
        <EmptyState
          size="compact"
          title="No captured emails found"
          message="Emails will appear here when sent in development mode."
          visual={<Icon name="mail" />}
        />
      </SpecimenGroup>
    </div>
  );
}

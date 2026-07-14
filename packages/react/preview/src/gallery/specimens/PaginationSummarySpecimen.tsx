import { PaginationSummary } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function PaginationSummarySpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Default">
        <PaginationSummary currentPage={1} totalPages={8} totalItems={156} pageSize={20} />
      </SpecimenGroup>

      <SpecimenGroup label="Single page">
        <PaginationSummary currentPage={1} totalPages={1} totalItems={12} pageSize={20} />
      </SpecimenGroup>

      <SpecimenGroup label="Large dataset">
        <PaginationSummary currentPage={5} totalPages={50} totalItems={1000} pageSize={20} />
      </SpecimenGroup>
    </div>
  );
}

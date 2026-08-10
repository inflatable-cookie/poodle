import { RemediationBanner } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const retry = { id: "retry", label: "Try again", variant: "primary" as const, isDisabled: false };
const details = { id: "details", label: "View details", variant: "secondary" as const, isDisabled: false };

export function RemediationBannerSpecimen() {
  return <div className="poodle-specimen">
    <SpecimenGroup label="Recovery actions">
      <RemediationBanner title="We could not save your changes" message="Your edits are still local. Retry the save or inspect the error details." tone="danger" primaryAction={retry} secondaryAction={details} isDismissible />
    </SpecimenGroup>
    <SpecimenGroup label="Recovery in progress"><RemediationBanner title="Reconnecting" message="This should only take a moment." tone="pending" /></SpecimenGroup>
  </div>;
}

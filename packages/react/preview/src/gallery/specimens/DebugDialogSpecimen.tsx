import { DebugDialog } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

export function DebugDialogSpecimen() {
  const value = {
    id: "asset_42",
    status: "ready",
    checks: ["metadata", "thumbnail", "permissions"],
  };

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="With debug value">
        <DebugDialog value={value} title="Asset payload" triggerLabel="Inspect payload" />
      </SpecimenGroup>

      <SpecimenGroup label="Custom trigger">
        <DebugDialog
          value={value}
          title="Compact payload"
          triggerLabel="Debug"
          triggerVariant="secondary"
          triggerSize="xs"
          maxHeight="18rem"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Hidden when null">
        <DebugDialog value={null} />
      </SpecimenGroup>
    </div>
  );
}

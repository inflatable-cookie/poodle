import type { CSSProperties } from "react";
import { UpdateStatus } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const ready = { kind: "ready" } as const;

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };

export function UpdateStatusSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <UpdateStatus
          status={ready}
          availability={{ state: "upToDate" }}
          installedVersion="1.3.0"
          channel="production"
          size={size}
        />
      )}
      densities={(density) => (
        <UpdateStatus
          status={ready}
          availability={{ state: "upToDate" }}
          installedVersion="1.3.0"
          channel="production"
          density={density}
        />
      )}
    >
      <div style={stackStyle}>
        <SpecimenGroup label="Availability">
          <UpdateStatus
            status={ready}
            availability={{
              state: "offer",
              version: "1.4.0",
              reason: "staged",
              notes: "Bug fixes and performance improvements across the board.",
            }}
          />
          <UpdateStatus
            status={ready}
            availability={{ state: "upToDate" }}
            installedVersion="1.3.0"
            channel="production"
          />
          <UpdateStatus
            status={ready}
            availability={{
              state: "aheadOfChannel",
              installed: "1.3.0-nightly.4",
              channel: "1.2.9",
            }}
            aheadOfChannel={{ installed: "1.3.0-nightly.4", channel: "1.2.9" }}
          />
          <UpdateStatus
            status={ready}
            availability={{ state: "withheldByRollout", version: "2.0.0" }}
          />
          <UpdateStatus
            status={ready}
            availability={{ state: "managedElsewhere", version: "1.4.0", manager: "homebrewCask" }}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Progress">
          <UpdateStatus status={ready} progress={{ state: "downloading", fraction: 0.42 }} />
          <UpdateStatus status={ready} progress={{ state: "downloading", fraction: null }} />
          <UpdateStatus status={ready} progress={{ state: "verifying" }} />
          <UpdateStatus status={ready} progress={{ state: "readyToInstall", version: "1.4.0" }} />
          <UpdateStatus status={ready} progress={{ state: "installing", version: "1.4.0" }} />
        </SpecimenGroup>

        <SpecimenGroup label="Deferral and rejection">
          <UpdateStatus
            status={ready}
            availability={{ state: "offer", version: "1.4.0", reason: "staged", notes: null }}
            deferral={{
              version: "1.4.0",
              cause: { cause: "workInFlight", detail: "A transfer is running." },
            }}
          />
          <UpdateStatus
            status={ready}
            availability={{ state: "managedElsewhere", version: "1.4.0", manager: "homebrewCask" }}
            deferral={{
              version: "1.4.0",
              cause: {
                cause: "externallyManaged",
                manager: "homebrewCask",
                command: "brew upgrade finch",
              },
            }}
          />
          <UpdateStatus status={ready} lastRejection="unreachable" />
          <UpdateStatus status={ready} lastRejection="signatureRejected" />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

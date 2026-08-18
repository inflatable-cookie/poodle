import type { CSSProperties } from "react";
import { LicenceStatus } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const now = Math.floor(Date.now() / 1_000);
const soon = now + 12 * 86_400;
const later = now + 240 * 86_400;
const past = now - 9 * 86_400;
const checked = now - 3_600;

const offline = { kind: "offlineSignature" } as const;
const remote = { kind: "remoteAssertion", checked } as const;

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };

export function LicenceStatusSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={later}
          updateUntil={later}
          usable={true}
          attention="none"
          size={size}
        />
      )}
      densities={(density) => (
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={later}
          updateUntil={later}
          usable={true}
          attention="none"
          density={density}
        />
      )}
    >
      <div style={stackStyle}>
        <SpecimenGroup label="Active">
          <LicenceStatus
            usability={{ state: "active" }}
            trustBasis={offline}
            useUntil={later}
            updateUntil={later}
            usable={true}
            attention="none"
          />
        </SpecimenGroup>

        <SpecimenGroup label="In grace">
          {/* A pending renewal is the seller's outstanding work. It stays calm. */}
          <LicenceStatus
            usability={{ state: "inGrace", until: soon }}
            trustBasis={remote}
            useUntil={soon}
            updateUntil={later}
            usable={true}
            attention="none"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Use window expired">
          <LicenceStatus
            usability={{ state: "useWindowExpired", at: past }}
            trustBasis={offline}
            useUntil={past}
            updateUntil={later}
            usable={false}
            attention="actionable"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Lease lapsed">
          <LicenceStatus
            usability={{ state: "leaseLapsed", at: past }}
            trustBasis={remote}
            useUntil={later}
            updateUntil={later}
            usable={false}
            attention="actionable"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Clock refused">
          {/* The remedy is the machine's clock, never a purchase. */}
          <LicenceStatus
            usability={{ state: "clockRefused" }}
            trustBasis={remote}
            useUntil={later}
            updateUntil={later}
            usable={false}
            attention="actionable"
          />
        </SpecimenGroup>

        <SpecimenGroup label="No coverage expiry">
          <LicenceStatus
            usability={{ state: "active" }}
            trustBasis={offline}
            useUntil={null}
            updateUntil={null}
            usable={true}
            attention="none"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Updates expired">
          {/* Perpetual use, lapsed updates. Two windows, two rows: collapsing
              them is how an owner is told they have lost what they bought. */}
          <LicenceStatus
            usability={{ state: "active" }}
            trustBasis={offline}
            useUntil={null}
            updateUntil={past}
            usable={true}
            attention="informational"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Use window only">
          <LicenceStatus
            usability={{ state: "active" }}
            trustBasis={offline}
            useUntil={later}
            updateUntil={null}
            usable={true}
            attention="none"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Offline verification">
          <LicenceStatus
            usability={{ state: "active" }}
            trustBasis={offline}
            useUntil={later}
            updateUntil={later}
            usable={true}
            attention="none"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Remote verification">
          <LicenceStatus
            usability={{ state: "active" }}
            trustBasis={remote}
            useUntil={later}
            updateUntil={later}
            usable={true}
            attention="none"
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

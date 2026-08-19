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
const pairStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "1rem" };

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
          <div style={pairStyle}>
            {/* Covered use and updates, verified on this machine. */}
            <LicenceStatus
              usability={{ state: "active" }}
              trustBasis={offline}
              useUntil={later}
              updateUntil={later}
              usable={true}
              attention="none"
            />
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
          </div>
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
          {/* Lifetime updates, lapsed lease: the licence is not expired. */}
          <LicenceStatus
            usability={{ state: "leaseLapsed", at: past }}
            trustBasis={remote}
            useUntil={later}
            updateUntil={null}
            usable={false}
            attention="actionable"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Clock refused">
          {/* The remedy is the machine's clock, never a purchase. */}
          <LicenceStatus
            usability={{ state: "clockRefused" }}
            trustBasis={remote}
            useUntil={null}
            updateUntil={null}
            usable={false}
            attention="actionable"
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

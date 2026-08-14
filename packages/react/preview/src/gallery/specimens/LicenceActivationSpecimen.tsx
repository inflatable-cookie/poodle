import type { CSSProperties } from "react";
import { LicenceActivation } from "@inflatable-cookie/poodle-react";
import type { LicenceKeyProblem, LicenceKeyResult } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

/* Stand-ins for the host's behaviour. The real parser and account journey
   belong to the authority — the specimen shows that Poodle works against any
   pair that satisfies the interface, and imports neither. */
const keyFormat = {
  parse(input: string): LicenceKeyResult {
    const stripped = input.replace(/[-\s]/g, "");
    if (/[^A-Za-z0-9]/.test(stripped)) {
      return { ok: false, problem: { kind: "unexpectedSymbol", symbol: stripped[0] ?? "?" } };
    }
    if (stripped.length < 20) {
      return { ok: false, problem: { kind: "tooShort", minimum: 20, actual: stripped.length } };
    }
    return { ok: true, key: stripped.toUpperCase(), grouped: stripped.toUpperCase() };
  },
  isProbablyATypo(problem: LicenceKeyProblem): boolean {
    return problem.kind === "checkFailed" || problem.kind === "unexpectedSymbol";
  },
};

const accountTokenProvider = { acquire: async () => null };

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };

export function LicenceActivationSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        {/* Three routes, one row, equal weight. The default selection does not
            make Key primary; the other two are peers, not fallbacks. */}
        <SpecimenGroup label="Routes">
          <LicenceActivation keyFormat={keyFormat} accountTokenProvider={accountTokenProvider} />
          <LicenceActivation
            keyFormat={keyFormat}
            accountTokenProvider={accountTokenProvider}
            defaultRoute="accountToken"
          />
          <LicenceActivation
            keyFormat={keyFormat}
            accountTokenProvider={accountTokenProvider}
            defaultRoute="licenceFile"
            fileAccept=".licence"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Pending and disabled">
          {/* Pending blocks a duplicate submit. Every route stays on screen. */}
          <LicenceActivation
            keyFormat={keyFormat}
            accountTokenProvider={accountTokenProvider}
            pending
          />
          <LicenceActivation
            keyFormat={keyFormat}
            accountTokenProvider={accountTokenProvider}
            disabled
          />
        </SpecimenGroup>

        <SpecimenGroup label="Host copy">
          <LicenceActivation
            keyFormat={keyFormat}
            accountTokenProvider={accountTokenProvider}
            title="Activate Finch"
            machineLabelLabel="Name this machine (optional)"
            activateLabel="Activate Finch"
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

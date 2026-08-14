import { useState, type CSSProperties } from "react";
import { Field, LicenceActivation, TextInput } from "@inflatable-cookie/poodle-react";
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

function EmbeddedAccountActivation() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <LicenceActivation
      mode="account"
      accountTokenProvider={{ acquire: async () => null }}
      activateLabel="Activate"
      fileAccept=".licence"
      machineLabel="Studio Mac"
      accountContent={(disabled) => (
        <>
          <Field id="licence-account-email" label="Email address">
            <TextInput
              id="licence-account-email"
              type="email"
              value={email}
              disabled={disabled}
              onValueChange={setEmail}
            />
          </Field>
          <Field id="licence-account-password" label="Password">
            <TextInput
              id="licence-account-password"
              type="password"
              value={password}
              disabled={disabled}
              onValueChange={setPassword}
            />
          </Field>
        </>
      )}
    />
  );
}

export function LicenceActivationSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Embedded account activation">
          <EmbeddedAccountActivation />
        </SpecimenGroup>

        <SpecimenGroup label="External account activation">
          <LicenceActivation
            mode="account"
            accountTokenProvider={accountTokenProvider}
            fileAccept=".licence"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Key activation">
          <LicenceActivation
            mode="key"
            keyFormat={keyFormat}
            keyCodeInput={{ length: 20, groups: [5, 5, 5, 5] }}
            size="xs"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Pending and disabled">
          {/* Pending blocks a duplicate submit. Every route stays on screen. */}
          <LicenceActivation
            mode="account"
            accountTokenProvider={accountTokenProvider}
            pending
          />
          <LicenceActivation
            mode="key"
            keyFormat={keyFormat}
            keyCodeInput={{ length: 20, groups: [5, 5, 5, 5] }}
            disabled
          />
        </SpecimenGroup>

        <SpecimenGroup label="Host copy">
          <LicenceActivation
            mode="account"
            accountTokenProvider={accountTokenProvider}
            title="Activate Finch"
            machineLabel={null}
            activateLabel="Activate Finch"
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

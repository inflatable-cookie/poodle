import { useState } from "react";
import { PasswordRequirements, TextInput, type PasswordRequirementsPolicy } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const requirements: PasswordRequirementsPolicy = {
  minLength: 12,
  requireMixedCase: true,
  requireDigit: true,
  requireSpecial: true,
  minStrengthScore: 3,
  description: "Use a long password with varied character types.",
};

const stackStyle = {
  display: "flex",
  flexDirection: "column",
  gap: "1rem",
  maxWidth: "28rem",
} as const;

const fieldStyle = {
  display: "flex",
  flexDirection: "column",
  gap: "0.375rem",
} as const;

const fieldLabelStyle = {
  fontSize: "0.875rem",
  color: "var(--poodle-color-text-secondary)",
} as const;

const variantStyle = { width: "min(100%, 28rem)" } as const;

export function PasswordRequirementsSpecimen() {
  const [password, setPassword] = useState("");

  return (
    <SpecimenLayout
      showSizes
      showDensities={false}
      sizes={(size) => (
        <div style={variantStyle}>
          <PasswordRequirements password="Example123!" requirements={requirements} size={size} />
        </div>
      )}
    >
      <div style={stackStyle}>
        <SpecimenGroup label="Default">
          <div style={fieldStyle}>
            <label htmlFor="password-requirements-specimen-password" style={fieldLabelStyle}>
              Password
            </label>
            <TextInput
              id="password-requirements-specimen-password"
              value={password}
              type="password"
              onValueChange={setPassword}
            />
          </div>
          <PasswordRequirements password={password} requirements={requirements} />
        </SpecimenGroup>

        <SpecimenGroup label="Loading">
          <PasswordRequirements password="" requirements={null} loading />
        </SpecimenGroup>

        <SpecimenGroup label="Error">
          <PasswordRequirements password="" requirements={null} error="Could not load password requirements." />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

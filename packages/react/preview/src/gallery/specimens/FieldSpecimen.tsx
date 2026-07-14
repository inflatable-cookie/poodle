import type { CSSProperties } from "react";
import { Field, TextInput } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const variantBlockStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.5rem",
  width: "min(24rem, 100%)",
};

const variantStyle: CSSProperties = {
  width: "min(24rem, 100%)",
};

const variantLabelStyle: CSSProperties = {
  color: "var(--poodle-color-text-secondary)",
  fontFamily: "var(--poodle-typography-label-family)",
  fontSize: "0.75rem",
  fontWeight: "var(--poodle-typography-label-weight)" as CSSProperties["fontWeight"],
  lineHeight: "var(--poodle-typography-label-lineHeight)",
  letterSpacing: "0.08em",
  textTransform: "uppercase",
};

export function FieldSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={variantBlockStyle}>
          <div style={variantLabelStyle}>{size.toUpperCase()}</div>
          <div style={variantStyle}>
            <Field
              id={`field-size-${size}`}
              label="Display name"
              description="This is how your name appears to other users."
              size={size}
            >
              <TextInput id={`field-size-${size}`} placeholder="Enter your name" ariaLabel="Display name" />
            </Field>
          </div>
        </div>
      )}
      densities={(density) => (
        <div style={variantBlockStyle}>
          <div style={variantLabelStyle}>{density.toUpperCase()}</div>
          <div style={variantStyle}>
            <Field
              id={`field-density-${density}`}
              label="Display name"
              description="This is how your name appears to other users."
              density={density}
            >
              <TextInput id={`field-density-${density}`} placeholder="Enter your name" ariaLabel="Display name" />
            </Field>
          </div>
        </div>
      )}
    >
      <div className="poodle-specimen" style={{ maxWidth: "24rem" }}>
        <SpecimenGroup label="Default with description">
          <Field
            label="Display name"
            description="This is how your name appears to other users."
            id="field-name"
            control={({ describedBy }) => (
              <TextInput
                id="field-name"
                placeholder="Enter your name"
                ariaLabel="Display name"
                describedBy={describedBy}
              />
            )}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Required">
          <Field label="Email address" required id="field-email">
            <TextInput id="field-email" placeholder="you@example.com" ariaLabel="Email address" />
          </Field>
        </SpecimenGroup>

        <SpecimenGroup label="With error">
          <Field
            label="Username"
            error="This username is already taken."
            validationState="invalid"
            id="field-user"
            control={({ describedBy, validationState }) => (
              <TextInput
                id="field-user"
                value="admin"
                ariaLabel="Username"
                describedBy={describedBy}
                validationState={validationState}
              />
            )}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Optional">
          <Field label="Phone number" optionalLabel="optional" id="field-phone">
            <TextInput id="field-phone" placeholder="+1 (555) 000-0000" ariaLabel="Phone number" />
          </Field>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

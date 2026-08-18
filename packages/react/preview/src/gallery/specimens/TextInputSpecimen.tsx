import { useState, type CSSProperties } from "react";
import { TextInput, Field, Eyebrow, Surface } from "@inflatable-cookie/poodle-react";
import type { InputValidationStatus, ValidationResult, ValidationState } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const specimenStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "1.25rem",
};

const itemStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.375rem",
};

const controlStyle: CSSProperties = {
  maxWidth: "20rem",
};

export function TextInputSpecimen() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("invalid-email");
  const [validationState, setValidationState] = useState<ValidationState>("invalid");
  const [slug, setSlug] = useState("");
  const [slugStatus, setSlugStatus] = useState<InputValidationStatus>("idle");
  const [searchQuery, setSearchQuery] = useState("");

  async function validateSlug(value: string): Promise<ValidationResult> {
    await new Promise((resolve) => setTimeout(resolve, 250));
    if (value === "northstar") return { valid: false, message: "That slug is already in use." };
    return { valid: true };
  }

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={controlStyle}>
          <TextInput id={"size-" + size} size={size} placeholder={size.toUpperCase()} />
        </div>
      )}
      densities={(density) => (
        <div style={controlStyle}>
          <TextInput id={"density-" + density} density={density} placeholder="Type here" />
        </div>
      )}
    >
            <SpecimenGroup label="Default">
        <div style={controlStyle}>
                      <Field id="name-field" label="Name" description="Enter your full name.">
                        <TextInput id="name-field" placeholder="Jane Doe" onValueChange={(nextValue) => setName(nextValue)} />
                      </Field>
                    </div>
      </SpecimenGroup>

                <SpecimenGroup label="With validation">
        <div style={controlStyle}>
                      <Field
                        id="email-field"
                        label="Email"
                        description="A valid email address is required."
                        validationState={validationState}
                        error={validationState === "invalid" ? "Please enter a valid email address." : null}
                      >
                        <TextInput
                          id="email-field"
                          value={email}
                          validationState={validationState}
                          onValueChange={(nextValue) => {
                            setEmail(nextValue);
                            setValidationState(nextValue.includes("@") ? "valid" : "invalid");
                          }}
                        />
                      </Field>
                    </div>
      </SpecimenGroup>

                <SpecimenGroup label="Slug">
        <div style={controlStyle}>
                      <Field
                        id="slug-field"
                        label="Slug"
                        description="Generates from the title until the user edits it."
                        validationState={
                          slugStatus === "validating"
                            ? "pending"
                            : slugStatus === "invalid"
                              ? "invalid"
                              : slugStatus === "valid"
                                ? "valid"
                                : "none"
                        }
                        error={slugStatus === "invalid" ? "That slug is not available." : null}
                      >
                        <TextInput
                          id="slug-field"
                          type="slug"
                          value={slug}
                          source="Northstar Launch Plan"
                          prefix="/projects/"
                          maxLength={64}
                          validate={validateSlug}
                          onValueChange={(nextValue) => setSlug(nextValue)}
                          onValidationChange={(detail) => setSlugStatus(detail.status)}
                        />
                      </Field>
                    </div>
      </SpecimenGroup>

                <SpecimenGroup label="Search">
        <div style={controlStyle}>
                      <TextInput
                        id="search-field"
                        type="search"
                        placeholder="Search..."
                        value={searchQuery}
                        onValueChange={(nextValue) => setSearchQuery(nextValue)}
                        onClear={() => setSearchQuery("")}
                      />
                    </div>
      </SpecimenGroup>

                <SpecimenGroup label="Prefix and suffix">
        <div style={controlStyle}>
                      <TextInput id="price-field" prefix="$" suffix="USD" placeholder="0.00" inputMode="decimal" />
                    </div>
      </SpecimenGroup>

                <SpecimenGroup label="Disabled">
        <div style={controlStyle}>
                      <Field id="disabled-field" label="API key">
                        <TextInput id="disabled-field" value="sk-xxxx-xxxx-xxxx" disabled />
                      </Field>
                    </div>
      </SpecimenGroup>

      <SpecimenGroup label="Multiline">
        <div style={controlStyle}>
          <Field id="multiline-field" label="Description">
            <TextInput id="multiline-field" type="multiline" rows={3} maxLength={280} showCharCount placeholder="Enter a description..." />
          </Field>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

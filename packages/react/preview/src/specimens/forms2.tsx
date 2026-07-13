import { useState } from "react";
import { FieldSet, PasswordRequirements, Select, TextInput } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

registerSpecimen({
  slug: "field-set",
  title: "FieldSet",
  render: () => (
    <SpecimenSection title="FieldSet">
      <FieldSet legend="Profile" description="Public information" columns={2}>
        <TextInput placeholder="First name" ariaLabel="First name" />
        <TextInput placeholder="Last name" ariaLabel="Last name" />
      </FieldSet>
    </SpecimenSection>
  ),
});

function PasswordReqSpecimen() {
  const [pw, setPw] = useState("");
  return (
    <SpecimenSection title="PasswordRequirements">
      <TextInput type="password" value={pw} onValueChange={setPw} placeholder="Password" ariaLabel="Password" />
      <PasswordRequirements
        password={pw}
        requirements={{ minLength: 8, requireMixedCase: true, requireDigit: true, requireSpecial: false }}
      />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "password-requirements", title: "PasswordRequirements", render: () => <PasswordReqSpecimen /> });

function SelectSpecimen() {
  const [plan, setPlan] = useState<string | null>("");
  const [fruit, setFruit] = useState<string | null>("");
  return (
    <SpecimenSection title="Select">
      <Select
        native={false}
        value={plan}
        onValueChange={setPlan}
        placeholder="Choose a plan"
        ariaLabel="Plan"
        options={[
          { value: "starter", label: "Starter", description: "For small teams" },
          { value: "growth", label: "Growth" },
          { value: "enterprise", label: "Enterprise", disabled: true },
        ]}
      />
      <span data-testid="select-value">plan: {plan}</span>
      <Select
        searchable
        value={fruit}
        onValueChange={setFruit}
        placeholder="Search fruit"
        ariaLabel="Fruit"
        options={[
          { value: "apple", label: "Apple" },
          { value: "banana", label: "Banana" },
          { value: "cherry", label: "Cherry" },
        ]}
      />
      <span data-testid="search-value">fruit: {fruit}</span>
      <Select
        ariaLabel="Native"
        placeholder="Native select"
        options={[
          { value: "a", label: "Option A" },
          { value: "b", label: "Option B" },
        ]}
      />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "select", title: "Select", render: () => <SelectSpecimen /> });

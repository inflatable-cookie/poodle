import { useState } from "react";
import { Radio } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function RadioSpecimen() {
  const [shipping, setShipping] = useState("standard");

  return (
    <SpecimenLayout
      sizes={(size) => <Radio name={`specimen-size-${size}`} label="Option" checked size={size} />}
      densities={(density) => <Radio name={`specimen-density-${density}`} label="Option" density={density} />}
    >
      <SpecimenGroup label="Default">
        <Radio
          name="specimen-shipping"
          value="standard"
          label="Standard shipping"
          checked={shipping === "standard"}
          onCheckedChange={(checked) => checked && setShipping("standard")}
        />
        <Radio
          name="specimen-shipping"
          value="express"
          label="Express shipping"
          checked={shipping === "express"}
          onCheckedChange={(checked) => checked && setShipping("express")}
        />
        <Radio
          name="specimen-shipping"
          value="overnight"
          label="Overnight shipping"
          checked={shipping === "overnight"}
          onCheckedChange={(checked) => checked && setShipping("overnight")}
        />
      </SpecimenGroup>

      <SpecimenGroup label="States">
        <Radio name="specimen-states-a" label="Disabled unchecked" disabled />
        <Radio name="specimen-states-b" label="Disabled checked" checked disabled />
        <Radio name="specimen-states-c" label="Read-only checked" checked readOnly />
        <Radio name="specimen-states-d" label="Custom selected color" checked selectedColor="#22c55e" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

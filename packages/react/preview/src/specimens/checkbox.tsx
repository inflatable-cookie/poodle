import { useState } from "react";
import { Checkbox } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function CheckboxSpecimen() {
  const [agreed, setAgreed] = useState(false);
  return (
    <SpecimenSection title="Checkbox">
      <Row>
        <Checkbox label={`Controlled (${agreed ? "on" : "off"})`} checked={agreed} onCheckedChange={setAgreed} />
        <Checkbox label="Uncontrolled" defaultChecked />
        <Checkbox label="Mixed" mixed />
        <Checkbox label="Disabled" disabled />
        <Checkbox label="Read-only" readOnly defaultChecked />
      </Row>
    </SpecimenSection>
  );
}

registerSpecimen({ slug: "checkbox", title: "Checkbox", render: () => <CheckboxSpecimen /> });

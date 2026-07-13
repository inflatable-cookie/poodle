import { useState } from "react";
import { Button } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function ButtonSpecimen() {
  const [clicks, setClicks] = useState(0);
  return (
    <SpecimenSection title="Button">
      <Row>
        <Button variant="primary" onClick={() => setClicks((n) => n + 1)}>
          Primary ({clicks})
        </Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="ghost">Ghost</Button>
        <Button variant="primary" disabled>
          Disabled
        </Button>
      </Row>
    </SpecimenSection>
  );
}

registerSpecimen({ slug: "button", title: "Button", render: () => <ButtonSpecimen /> });

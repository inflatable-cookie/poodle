import { useState } from "react";
import { Tabs } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function TabsSpecimen() {
  const [tab, setTab] = useState("overview");
  return (
    <SpecimenSection title="Tabs">
      <Tabs
        value={tab}
        onValueChange={setTab}
        items={[
          { value: "overview", label: "Overview" },
          { value: "activity", label: "Activity" },
          { value: "settings", label: "Settings", disabled: true },
          { value: "billing", label: "Billing" },
        ]}
      />
      <Row>
        <span data-testid="active-tab">active: {tab}</span>
      </Row>
    </SpecimenSection>
  );
}

registerSpecimen({ slug: "tabs", title: "Tabs", render: () => <TabsSpecimen /> });

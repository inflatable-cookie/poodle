import { useState } from "react";
import { IconButton, Radio, RadioGroup, Switch } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function SwitchSpecimen() {
  const [on, setOn] = useState(false);
  return (
    <SpecimenSection title="Switch">
      <Row>
        <Switch label={`Controlled (${on ? "on" : "off"})`} checked={on} onCheckedChange={setOn} />
        <Switch label="Uncontrolled" defaultChecked />
        <Switch label="Disabled" disabled />
        <Switch label="Read-only" readOnly defaultChecked />
        <Switch leftLabel="Off" rightLabel="On" defaultChecked />
      </Row>
    </SpecimenSection>
  );
}

registerSpecimen({ slug: "switch", title: "Switch", render: () => <SwitchSpecimen /> });

function RadioGroupSpecimen() {
  const [choice, setChoice] = useState<string | null>("b");
  return (
    <SpecimenSection title="RadioGroup">
      <RadioGroup
        value={choice}
        onValueChange={setChoice}
        ariaLabel="Plan"
        options={[
          { value: "a", label: "Starter" },
          { value: "b", label: "Growth" },
          { value: "c", label: "Enterprise", disabled: true },
        ]}
      />
      <span data-testid="radio-value">value: {choice}</span>
    </SpecimenSection>
  );
}

registerSpecimen({ slug: "radio-group", title: "RadioGroup", render: () => <RadioGroupSpecimen /> });

registerSpecimen({
  slug: "radio",
  title: "Radio",
  render: () => (
    <SpecimenSection title="Radio">
      <Row>
        <Radio label="Standalone" defaultChecked />
        <Radio label="Disabled" disabled />
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "icon-button",
  title: "IconButton",
  render: () => (
    <SpecimenSection title="IconButton">
      <Row>
        <IconButton icon="check" ariaLabel="Confirm" />
        <IconButton icon="pencil" ariaLabel="Edit" variant="secondary" tooltip="Edit item" />
        <IconButton icon="trash-2" ariaLabel="Delete" variant="ghost" tone="danger" />
        <IconButton icon="check" ariaLabel="Busy" loading />
        <IconButton icon="star" ariaLabel="Favourite" defaultPressed={false} />
      </Row>
    </SpecimenSection>
  ),
});

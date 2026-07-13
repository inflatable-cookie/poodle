import { useState } from "react";
import { Accordion, Button, Collapsible, Menu, Text } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

registerSpecimen({
  slug: "collapsible",
  title: "Collapsible",
  render: () => (
    <SpecimenSection title="Collapsible">
      <Collapsible title="Advanced settings" description="Rarely needed">
        <Text>Hidden content</Text>
      </Collapsible>
      <Collapsible title="Open by default" defaultOpen>
        <Text>Visible content</Text>
      </Collapsible>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "accordion",
  title: "Accordion",
  render: () => (
    <SpecimenSection title="Accordion">
      <Accordion
        defaultValue="a"
        items={[
          { value: "a", label: "First section", description: "Summary" },
          { value: "b", label: "Second section" },
          { value: "c", label: "Disabled", disabled: true },
        ]}
      >
        {(item) => <Text>Panel for {item.label}</Text>}
      </Accordion>
    </SpecimenSection>
  ),
});

function MenuSpecimen() {
  const [last, setLast] = useState("");
  return (
    <SpecimenSection title="Menu">
      <Menu
        ariaLabel="Actions"
        onAction={setLast}
        items={[
          { value: "rename", label: "Rename", shortcutLabel: "⌘R" },
          { value: "duplicate", label: "Duplicate" },
          { value: "sep", label: "", kind: "separator" },
          { value: "archive", label: "Archive", disabled: true },
          { value: "delete", label: "Delete", tone: "danger" },
        ]}
        trigger={<Button variant="secondary">Open menu</Button>}
      />
      <span data-testid="menu-action">last: {last}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "menu", title: "Menu", render: () => <MenuSpecimen /> });

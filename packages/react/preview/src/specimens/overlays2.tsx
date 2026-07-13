import { useState } from "react";
import { Button, Dialog, Drawer, Text, Tooltip } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function DialogSpecimen() {
  const [open, setOpen] = useState(false);
  return (
    <SpecimenSection title="Dialog">
      <Row>
        <Button variant="primary" onClick={() => setOpen(true)}>
          Open dialog
        </Button>
      </Row>
      <Dialog
        open={open}
        title="Confirm change"
        description="This applies immediately."
        showCloseButton
        onOpenChange={setOpen}
        onRequestClose={() => setOpen(false)}
        actions={
          <Row>
            <Button variant="ghost" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button variant="primary" onClick={() => setOpen(false)}>
              Confirm
            </Button>
          </Row>
        }
      >
        <Text>Dialog body content</Text>
      </Dialog>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "dialog", title: "Dialog", render: () => <DialogSpecimen /> });

function DrawerSpecimen() {
  const [open, setOpen] = useState(false);
  return (
    <SpecimenSection title="Drawer">
      <Button variant="secondary" onClick={() => setOpen(true)}>
        Open drawer
      </Button>
      <Drawer open={open} title="Details" edge="right" onOpenChange={setOpen} onRequestClose={() => setOpen(false)}>
        <Text>Drawer body</Text>
      </Drawer>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "drawer", title: "Drawer", render: () => <DrawerSpecimen /> });

registerSpecimen({
  slug: "tooltip",
  title: "Tooltip",
  render: () => (
    <SpecimenSection title="Tooltip">
      <Row>
        <Tooltip content="Saves your work">
          <Button variant="secondary">Hover me</Button>
        </Tooltip>
      </Row>
    </SpecimenSection>
  ),
});

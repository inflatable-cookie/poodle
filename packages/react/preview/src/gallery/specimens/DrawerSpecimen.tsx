import { SpecimenGroup } from "../SpecimenGroup";
import { useState, type CSSProperties } from "react";
import { Drawer, Button } from "@inflatable-cookie/poodle-react";
import { SpecimenLayout } from "../SpecimenLayout";

export function DrawerSpecimen() {
  const [rightOpen, setRightOpen] = useState(false);
  const [leftOpen, setLeftOpen] = useState(false);
  const [axisOpen, setAxisOpen] = useState<Record<string, boolean>>({});

  const paragraph: CSSProperties = {
    margin: 0,
    fontSize: "0.875rem",
    color: "var(--poodle-color-text-secondary)",
  };

  const setAxis = (key: string, open: boolean) => setAxisOpen((prev) => ({ ...prev, [key]: open }));

  return (
    <SpecimenLayout
      sizes={(size) => (
        <>
          <Button variant="secondary" onClick={() => setAxis(`size-${size}`, true)}>
            Open {size} drawer
          </Button>
          <Drawer
            open={axisOpen[`size-${size}`] ?? false}
            onOpenChange={(open) => setAxis(`size-${size}`, open)}
            title="Settings"
            description="Configure your preferences."
            size={size}
          >
            <p style={paragraph}>Drawer content goes here. You can put forms, navigation, or any other content.</p>
          </Drawer>
        </>
      )}
      densities={(density) => (
        <>
          <Button variant="secondary" onClick={() => setAxis(`density-${density}`, true)}>
            Open {density} drawer
          </Button>
          <Drawer
            open={axisOpen[`density-${density}`] ?? false}
            onOpenChange={(open) => setAxis(`density-${density}`, open)}
            title="Settings"
            description="Configure your preferences."
            density={density}
          >
            <p style={paragraph}>Drawer content goes here. You can put forms, navigation, or any other content.</p>
          </Drawer>
        </>
      )}
    >
            <SpecimenGroup label="Right edge (default)">
        <Button variant="secondary" onClick={() => setRightOpen(true)}>Open right drawer</Button>
      </SpecimenGroup>

                <SpecimenGroup label="Left edge">
        <Button variant="secondary" onClick={() => setLeftOpen(true)}>Open left drawer</Button>
      </SpecimenGroup>

      <Drawer
        open={rightOpen}
        title="Settings"
        description="Configure your preferences."
        onOpenChange={(open) => setRightOpen(open)}
        actions={
          <>
            <Button variant="secondary" onClick={() => setRightOpen(false)}>Cancel</Button>
            <Button onClick={() => setRightOpen(false)}>Save</Button>
          </>
        }
      >
        <p style={paragraph}>Drawer content goes here. You can put forms, navigation, or any other content.</p>
      </Drawer>

      <Drawer
        open={leftOpen}
        edge="left"
        title="Navigation"
        onOpenChange={(open) => setLeftOpen(open)}
      >
        <p style={paragraph}>Side navigation or filters can live in a left-edge drawer.</p>
      </Drawer>
    </SpecimenLayout>
  );
}

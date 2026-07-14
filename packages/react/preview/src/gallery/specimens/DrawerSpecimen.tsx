import { useState, type CSSProperties } from "react";
import { Drawer, Button, Eyebrow, Surface } from "@poodle/react";

export function DrawerSpecimen() {
  const [rightOpen, setRightOpen] = useState(false);
  const [leftOpen, setLeftOpen] = useState(false);

  const paragraph: CSSProperties = {
    margin: 0,
    fontSize: "0.875rem",
    color: "var(--poodle-color-text-secondary)",
  };

  return (
    <>
      <Surface tone="panel" border="subtle" padding="md">
        <div className="poodle-specimen">
          <div className="poodle-specimen__row">
            <Eyebrow>Right edge (default)</Eyebrow>
            <Button variant="secondary" onClick={() => setRightOpen(true)}>Open right drawer</Button>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Left edge</Eyebrow>
            <Button variant="secondary" onClick={() => setLeftOpen(true)}>Open left drawer</Button>
          </div>
        </div>
      </Surface>

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
    </>
  );
}

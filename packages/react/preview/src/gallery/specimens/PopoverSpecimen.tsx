import { Popover, Button } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function PopoverSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Default (bottom-start)">
        <Popover ariaLabel="Quick settings" trigger={<Button variant="secondary">Open popover</Button>}>
          <div style={{ padding: "0.75rem", maxWidth: "16rem" }}>
            <strong style={{ display: "block", marginBottom: "0.25rem" }}>Quick settings</strong>
            <p style={{ margin: 0, fontSize: "0.8125rem", color: "var(--poodle-color-text-secondary)" }}>
              Adjust your display preferences or notification settings from this panel.
            </p>
          </div>
        </Popover>
      </SpecimenGroup>

      <SpecimenGroup label="Top placement">
        <Popover placement="top" ariaLabel="Help tip" trigger={<Button variant="secondary">Show help</Button>}>
          <div style={{ padding: "0.75rem", maxWidth: "16rem" }}>
            <p style={{ margin: 0, fontSize: "0.8125rem", color: "var(--poodle-color-text-secondary)" }}>
              Popovers can be anchored to any side of their trigger element.
            </p>
          </div>
        </Popover>
      </SpecimenGroup>
    </div>
  );
}

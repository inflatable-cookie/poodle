import { useState } from "react";
import { NavigationMenu, type NavigationMenuItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const items: NavigationMenuItem[] = [
  { value: "home", label: "Home" },
  { value: "components", label: "Components" },
  { value: "tokens", label: "Tokens" },
  { value: "guides", label: "Guides" },
  { value: "changelog", label: "Changelog", disabled: true },
];

export function NavigationMenuSpecimen() {
  const [active, setActive] = useState("components");

  const paragraphStyle = {
    margin: 0,
    fontSize: "0.875rem",
    color: "var(--poodle-color-text-secondary)",
  } as const;

  const frameStyle = {
    border: "1px dashed var(--poodle-color-border-subtle)",
    padding: "0.5rem",
  } as const;

  return (
    <SpecimenLayout
      sizes={(size) => (
        <NavigationMenu items={items} value="components" size={size} ariaLabel={size + " navigation"} />
      )}
      densities={(density) => (
        <NavigationMenu items={items} value="components" density={density} ariaLabel={density + " navigation"} />
      )}
    >
      <SpecimenGroup label="Horizontal navigation">
        <NavigationMenu
          items={items}
          value={active}
          ariaLabel="Main navigation"
          onValueChange={(value) => {
            if (value) setActive(value);
          }}
        >
          {() => (
            <p style={paragraphStyle}>
              Active section: <strong>{active}</strong>
            </p>
          )}
        </NavigationMenu>
      </SpecimenGroup>

      {/* The default trigger is borderless since g13.016; activeEdge opts the
          border/underline back in. Solid fill covers the open trigger with
          accent-base + text-inverse, and must survive hover. */}
      <SpecimenGroup label="Navigation menu (active outline)">
        <NavigationMenu items={items} value="components" activeEdge="outline" ariaLabel="Outlined main navigation" />
      </SpecimenGroup>

      <SpecimenGroup label="Navigation menu (active underline)">
        <NavigationMenu items={items} value="components" activeEdge="underline" ariaLabel="Underlined main navigation" />
      </SpecimenGroup>

      <SpecimenGroup label="Navigation menu (solid fill)">
        <NavigationMenu items={items} value="components" activeFill="solid" ariaLabel="Solid main navigation" />
      </SpecimenGroup>

      <SpecimenGroup label="Navigation menu (solid fill — hover the open trigger)">
        <div style={frameStyle}>
          <NavigationMenu
            items={items}
            value="components"
            activeFill="solid"
            ariaLabel="Solid hovered main navigation"
          />
        </div>
      </SpecimenGroup>

      {/* activeFill="none": the open trigger keeps its idle fill; the edge
          and the selected text colour mark selection alone. */}
      <SpecimenGroup label="Navigation menu (no fill)">
        <NavigationMenu
          items={items}
          value="components"
          activeFill="none"
          activeEdge="underline"
          ariaLabel="No-fill underlined main navigation"
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

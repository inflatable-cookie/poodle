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
    </SpecimenLayout>
  );
}

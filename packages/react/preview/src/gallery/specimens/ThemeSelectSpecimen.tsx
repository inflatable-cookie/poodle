import { useState } from "react";
import { ThemeSelect } from "@poodle/react";
import { themeOptions } from "@poodle/svelte-tokens";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const themes = themeOptions();

export function ThemeSelectSpecimen() {
  const [value, setValue] = useState("eclipse");
  const [sizeValue, setSizeValue] = useState("nord");
  const [densityValue, setDensityValue] = useState("rose");
  const selected = themes.find((t) => t.value === value);

  return (
    <SpecimenLayout
      sizes={(size) => <ThemeSelect themes={themes} size={size} value={sizeValue} onChange={setSizeValue} />}
      densities={(density) => (
        <ThemeSelect themes={themes} density={density} value={densityValue} onChange={setDensityValue} />
      )}
    >
      <SpecimenGroup label="Theme selector (standalone, live value)">
        <ThemeSelect themes={themes} value={value} onChange={setValue} />
        <pre style={{ margin: 0, fontSize: "0.75rem" }}>
          selected: {value} — {selected?.label}
        </pre>
      </SpecimenGroup>

      <SpecimenGroup label="Compact trigger (no label)">
        <ThemeSelect themes={themes} value="midnight" showLabel={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Four columns">
        <ThemeSelect themes={themes} value="solarized" columns={4} />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <ThemeSelect themes={themes} value="forest" disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

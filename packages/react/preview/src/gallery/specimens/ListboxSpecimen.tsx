import { Listbox, Surface, type ListboxItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const libraries: ListboxItem[] = [
  { value: "north", label: "Northstar" },
  { value: "poodle", label: "Poodle" },
  { value: "underlay", label: "Underlay" },
];
const cards: ListboxItem[] = [
  { value: "amber", label: "Amber Library" },
  { value: "blue", label: "Blue Library" },
  { value: "green", label: "Green Library" },
];
const multiple: ListboxItem[] = [
  { value: "audio", label: "Audio projects" },
  { value: "video", label: "Video projects" },
  { value: "image", label: "Image projects" },
];
const withDisabled: ListboxItem[] = [
  { value: "ready", label: "Ready to use" },
  { value: "archived", label: "Archived collection", disabled: true },
  { value: "recent", label: "Recently opened" },
];

export function ListboxSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Plain rows — single selection and typeahead">
        <Listbox items={libraries} defaultValue="north" ariaLabel="Libraries" />
      </SpecimenGroup>

      <SpecimenGroup label="Rich card rows — host content with Listbox selection">
        <Listbox
          items={cards}
          defaultValue="blue"
          ariaLabel="Library cards"
          renderItem={(item, selected, focused) => (
            <div
              className="poodle-listbox-card"
              data-selected={selected}
              data-focused={focused}
              style={{ borderRadius: "var(--poodle-radius-surface)", outline: selected ? "2px solid var(--poodle-color-accent-base)" : undefined, outlineOffset: focused ? 2 : undefined }}
            >
              <Surface padding="sm" border="subtle">
                <strong>{item.label}</strong>
                <div className="poodle-listbox-card-detail">Three collections · Updated today</div>
              </Surface>
            </div>
          )}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Multiple selection — Space, Shift+arrow, and Ctrl/Cmd+A">
        <Listbox items={multiple} selectionMode="multiple" defaultValues={["audio"]} ariaLabel="Project types" />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled option — skipped by focus and typeahead">
        <Listbox items={withDisabled} defaultValue="ready" ariaLabel="Collections" />
      </SpecimenGroup>
    </div>
  );
}

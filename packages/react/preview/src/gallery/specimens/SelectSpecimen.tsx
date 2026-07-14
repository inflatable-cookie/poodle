import { useState, type CSSProperties } from "react";
import { Select, Pill, Eyebrow, Surface, type SelectOption, type SelectOptionGroup } from "@poodle/react";
import { SpecimenLayout } from "../SpecimenLayout";

const fruitOptions: SelectOption[] = [
  { value: "apple", label: "Apple" },
  { value: "banana", label: "Banana" },
  { value: "cherry", label: "Cherry" },
  { value: "dragonfruit", label: "Dragonfruit" },
  { value: "elderberry", label: "Elderberry" },
];

const richOptions: SelectOption[] = [
  { value: "us", label: "United States", description: "North America", icon: "globe" },
  { value: "uk", label: "United Kingdom", description: "Europe", icon: "globe" },
  { value: "jp", label: "Japan", description: "Asia", icon: "globe" },
  { value: "au", label: "Australia", description: "Oceania", icon: "globe" },
  { value: "br", label: "Brazil", description: "South America", icon: "globe" },
];

const groupedOptions: SelectOptionGroup[] = [
  {
    label: "Fruits",
    options: [
      { value: "apple", label: "Apple" },
      { value: "banana", label: "Banana" },
      { value: "cherry", label: "Cherry" },
    ],
  },
  {
    label: "Vegetables",
    options: [
      { value: "carrot", label: "Carrot" },
      { value: "broccoli", label: "Broccoli" },
      { value: "spinach", label: "Spinach", disabled: true },
    ],
  },
];

const frameworkOptions: SelectOption[] = [
  { value: "svelte", label: "Svelte" },
  { value: "react", label: "React" },
  { value: "vue", label: "Vue" },
  { value: "angular", label: "Angular" },
  { value: "solid", label: "SolidJS" },
  { value: "astro", label: "Astro" },
];

const specimenStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.75rem",
};

const rowStyle: CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  gap: "0.75rem",
  alignItems: "center",
};

const fieldStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: "0.5rem",
  flex: 1,
  minWidth: "12rem",
};

const valueStyle: CSSProperties = {
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

const triggerStyle: CSSProperties = {
  width: "100%",
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: "0.5rem",
};

const optionStyle: CSSProperties = {
  width: "100%",
  display: "flex",
  alignItems: "flex-start",
  gap: "0.5rem",
};

const optionBodyStyle: CSSProperties = {
  flex: 1,
  display: "flex",
  flexDirection: "column",
  gap: "0.125rem",
  minWidth: 0,
};

const optionLabelStyle: CSSProperties = {
  minWidth: 0,
};

const secondaryTextStyle: CSSProperties = {
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

export function SelectSpecimen() {
  const [selectedFruit, setSelectedFruit] = useState<string | null>(null);
  const [selectedCountry, setSelectedCountry] = useState<string | null>(null);
  const [selectedFramework, setSelectedFramework] = useState<string | null>(null);
  const [freeformValue, setFreeformValue] = useState<string | null>(null);
  const [customCountry, setCustomCountry] = useState<string | null>(null);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <Select options={fruitOptions} placeholder="Select..." size={size} ariaLabel={`${size} select`} />
      )}
      densities={(density) => (
        <Select options={fruitOptions} placeholder="Select..." density={density} ariaLabel={`${density} select`} />
      )}
    >
      <Surface tone="panel" border="subtle" padding="md">
        <div style={specimenStyle}>
          <div style={rowStyle}>
            <Eyebrow>Native</Eyebrow>
            <div style={fieldStyle}>
              <Select
                options={fruitOptions}
                placeholder="Choose a fruit"
                ariaLabel="Fruit selection"
                onValueChange={(nextValue) => setSelectedFruit(nextValue)}
              />
              {selectedFruit ? <span style={valueStyle}>{selectedFruit}</span> : null}
            </div>
          </div>

          <div style={rowStyle}>
            <Eyebrow>Custom dropdown</Eyebrow>
            <div style={fieldStyle}>
              <Select
                options={richOptions}
                placeholder="Choose a country"
                native={false}
                ariaLabel="Country selection"
                onValueChange={(nextValue) => setSelectedCountry(nextValue)}
              />
              {selectedCountry ? <span style={valueStyle}>{selectedCountry}</span> : null}
            </div>
          </div>

          <div style={rowStyle}>
            <Eyebrow>Searchable</Eyebrow>
            <div style={fieldStyle}>
              <Select
                options={frameworkOptions}
                placeholder="Search frameworks..."
                searchable
                ariaLabel="Framework search"
                onValueChange={(nextValue) => setSelectedFramework(nextValue)}
              />
              {selectedFramework ? <span style={valueStyle}>{selectedFramework}</span> : null}
            </div>
          </div>

          <div style={rowStyle}>
            <Eyebrow>Freeform</Eyebrow>
            <div style={fieldStyle}>
              <Select
                options={frameworkOptions}
                placeholder="Type or select..."
                searchable
                freeform
                ariaLabel="Freeform"
                onValueChange={(nextValue) => setFreeformValue(nextValue)}
              />
              {freeformValue ? <span style={valueStyle}>{freeformValue}</span> : null}
            </div>
          </div>

          <div style={rowStyle}>
            <Eyebrow>Snippet rendering</Eyebrow>
            <div style={fieldStyle}>
              <Select
                options={richOptions}
                placeholder="Custom country"
                native={false}
                ariaLabel="Custom country selection"
                onValueChange={(nextValue) => setCustomCountry(nextValue)}
                trigger={({ selectedOption, placeholder }) => (
                  <span style={triggerStyle}>
                    <span>{selectedOption?.label ?? placeholder ?? ""}</span>
                    {selectedOption ? (
                      <Pill size="sm" appearance="subtle" tone="info">picked</Pill>
                    ) : null}
                  </span>
                )}
                option={({ option }) => (
                  <span style={optionStyle}>
                    <span style={optionBodyStyle}>
                      <span style={optionLabelStyle}>{option.label}</span>
                      {option.description ? (
                        <span style={secondaryTextStyle}>{option.description}</span>
                      ) : null}
                    </span>
                  </span>
                )}
                empty={({ query }) => (
                  <div style={secondaryTextStyle}>No match for "{query}"</div>
                )}
              />
              {customCountry ? <span style={valueStyle}>{customCountry}</span> : null}
            </div>
          </div>

          <div style={rowStyle}>
            <Eyebrow>Grouped</Eyebrow>
            <Select options={groupedOptions} placeholder="Choose a food" ariaLabel="Grouped food" />
          </div>

          <div style={rowStyle}>
            <Eyebrow>Clearable</Eyebrow>
            <Select
              options={fruitOptions}
              placeholder="All fruits"
              native={false}
              clearable
              ariaLabel="Clearable"
            />
          </div>

          <div style={rowStyle}>
            <Eyebrow>Disabled</Eyebrow>
            <Select options={fruitOptions} value="banana" disabled ariaLabel="Disabled" />
          </div>
        </div>
      </Surface>
    </SpecimenLayout>
  );
}

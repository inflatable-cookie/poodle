import { useState } from "react";
import { FilterBuilder, type FilterExpression, type FilterFieldDefinition } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const fields: FilterFieldDefinition[] = [
  {
    key: "format",
    label: "Format",
    kind: "multi-enum",
    options: [
      { value: "clap", label: "CLAP" },
      { value: "vst3", label: "VST3" },
      { value: "lv2", label: "LV2" },
      { value: "au", label: "AU" },
      { value: "vst2", label: "VST2" },
    ],
  },
  {
    key: "category",
    label: "Category",
    kind: "enum",
    options: [
      { value: "effect", label: "Effect" },
      { value: "instrument", label: "Instrument" },
      { value: "midi", label: "MIDI" },
    ],
  },
  { key: "name", label: "Name", kind: "text" },
  { key: "tag-count", label: "Tag count", kind: "number" },
  { key: "rating", label: "Rating", kind: "range" },
  { key: "hidden", label: "Hidden", kind: "boolean" },
  {
    key: "tag",
    label: "Tag",
    kind: "multi-enum",
    allowMultiple: true,
    options: [
      { value: "compressor", label: "Compressor" },
      { value: "mastering", label: "Mastering" },
      { value: "reverb", label: "Reverb" },
    ],
  },
];

export function FilterBuilderSpecimen() {
  const [value, setValue] = useState<FilterExpression>({
    combinator: "and",
    clauses: [
      { id: "format-1", key: "format", operator: "any_of", operand: { kind: "options", values: ["clap", "vst3"] } },
      { id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } },
      { id: "tag-count-1", key: "tag-count", operator: "gte", operand: { kind: "number", value: 3 } },
    ],
  });
  const [anyValue, setAnyValue] = useState<FilterExpression>({
    combinator: "or",
    clauses: [
      { id: "category-1", key: "category", operator: "is", operand: { kind: "options", values: ["effect"] } },
      { id: "name-1", key: "name", operator: "contains", operand: { kind: "text", value: "comp" } },
    ],
  });
  const [overflowValue, setOverflowValue] = useState<FilterExpression>({
    combinator: "and",
    clauses: [
      { id: "format-1", key: "format", operator: "any_of", operand: { kind: "options", values: ["clap"] } },
      { id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: true } },
      { id: "tag-count-1", key: "tag-count", operator: "gte", operand: { kind: "number", value: 2 } },
      { id: "rating-1", key: "rating", operator: "between", operand: { kind: "range", min: 3, max: 5 } },
      { id: "tag-1", key: "tag", operator: "all_of", operand: { kind: "options", values: ["mastering"] } },
      { id: "tag-2", key: "tag", operator: "none_of", operand: { kind: "options", values: ["reverb"] } },
    ],
  });
  const [sizeValue, setSizeValue] = useState<FilterExpression>({
    combinator: "and",
    clauses: [{ id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } }],
  });
  const [densityValue, setDensityValue] = useState<FilterExpression>({
    combinator: "and",
    clauses: [{ id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } }],
  });

  return (
    <SpecimenLayout
      sizes={(size) => <FilterBuilder fields={fields} size={size} value={sizeValue} onChange={setSizeValue} />}
      densities={(density) => (
        <FilterBuilder fields={fields} density={density} value={densityValue} onChange={setDensityValue} />
      )}
    >
      <SpecimenGroup label="Filter builder (controlled, live value, combinator on)">
        <FilterBuilder fields={fields} value={value} onChange={setValue} showCombinator />
        <pre style={{ margin: 0, fontSize: "0.75rem", maxHeight: "12rem", overflow: "auto" }}>
          {JSON.stringify(value, null, 2)}
        </pre>
      </SpecimenGroup>

      <SpecimenGroup label="Match any (showCombinator)">
        <FilterBuilder fields={fields} value={anyValue} onChange={setAnyValue} showCombinator />
      </SpecimenGroup>

      <SpecimenGroup label="AND-only (combinator toggle hidden — default)">
        <FilterBuilder fields={fields} value={anyValue} onChange={setAnyValue} />
      </SpecimenGroup>

      <SpecimenGroup label="Empty">
        <FilterBuilder fields={fields} value={{ combinator: "and", clauses: [] }} />
      </SpecimenGroup>

      <SpecimenGroup label="Overflowing pills + repeated field (allowMultiple)">
        <FilterBuilder fields={fields} value={overflowValue} onChange={setOverflowValue} />
      </SpecimenGroup>

      <SpecimenGroup label="Max 2 clauses">
        <FilterBuilder
          fields={fields}
          value={{
            combinator: "and",
            clauses: [
              { id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: true } },
              { id: "name-1", key: "name", operator: "contains", operand: { kind: "text", value: "bus" } },
            ],
          }}
          maxClauses={2}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <FilterBuilder
          fields={fields}
          value={{
            combinator: "and",
            clauses: [{ id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } }],
          }}
          disabled
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

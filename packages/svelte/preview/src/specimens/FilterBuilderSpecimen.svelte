<script lang="ts">
  import { FilterBuilder, type FilterExpression, type FilterFieldDefinition } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let value = $state<FilterExpression>({
    combinator: "and",
    clauses: [
      { id: "format-1", key: "format", operator: "any_of", operand: { kind: "options", values: ["clap", "vst3"] } },
      { id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } },
      { id: "tag-count-1", key: "tag-count", operator: "gte", operand: { kind: "number", value: 3 } },
    ],
  });

  let anyValue = $state<FilterExpression>({
    combinator: "or",
    clauses: [
      { id: "category-1", key: "category", operator: "is", operand: { kind: "options", values: ["effect"] } },
      { id: "name-1", key: "name", operator: "contains", operand: { kind: "text", value: "comp" } },
    ],
  });

  let overflowValue = $state<FilterExpression>({
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

  let sizeValue = $state<FilterExpression>({
    combinator: "and",
    clauses: [{ id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } }],
  });
  let densityValue = $state<FilterExpression>({
    combinator: "and",
    clauses: [{ id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } }],
  });
</script>

<SpecimenLayout>
  <SpecimenGroup
    label="Building filters"
    description="A controlled builder. The JSON is the live value."
  >
    <FilterBuilder {fields} bind:value showCombinator />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </SpecimenGroup>

  <SpecimenGroup
    label="Match all and match any"
    description="Match all requires every clause. Match any accepts the first match. Hide the toggle when the host is AND-only."
  >
    <FilterBuilder {fields} bind:value={anyValue} showCombinator />
    <FilterBuilder {fields} value={anyValue} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Empty and limited builders"
    description="An empty builder waits for the first clause. Capping at two hides the add row."
  >
    <FilterBuilder {fields} value={{ combinator: "and", clauses: [] }} />
    <FilterBuilder
      {fields}
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

  <SpecimenGroup
    label="Field types and overflow"
    description="Enum, multi-enum, boolean, text, number, and range, including a repeated field and overflowing pills."
  >
    <FilterBuilder {fields} bind:value={overflowValue} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled" description="Every control is inert.">
    <FilterBuilder
      {fields}
      value={{
        combinator: "and",
        clauses: [{ id: "hidden-1", key: "hidden", operator: "is", operand: { kind: "boolean", value: false } }],
      }}
      disabled
    />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <FilterBuilder {fields} {size} bind:value={sizeValue} />
  {/snippet}

  {#snippet densities(density)}
    <FilterBuilder {fields} {density} bind:value={densityValue} />
  {/snippet}
</SpecimenLayout>

<style>
  pre {
    margin: 0;
    font-size: 0.75rem;
    max-height: 12rem;
    overflow: auto;
  }
</style>

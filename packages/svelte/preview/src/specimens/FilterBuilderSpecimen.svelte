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
  <SpecimenGroup label="Filter builder (controlled, live value, combinator on)">
    <FilterBuilder {fields} bind:value showCombinator />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </SpecimenGroup>

  <SpecimenGroup label="Match any (showCombinator)">
    <FilterBuilder {fields} bind:value={anyValue} showCombinator />
  </SpecimenGroup>

  <SpecimenGroup label="AND-only (combinator toggle hidden — default)">
    <FilterBuilder {fields} value={anyValue} />
  </SpecimenGroup>

  <SpecimenGroup label="Empty">
    <FilterBuilder {fields} value={{ combinator: "and", clauses: [] }} />
  </SpecimenGroup>

  <SpecimenGroup label="Overflowing pills + repeated field (allowMultiple)">
    <FilterBuilder {fields} bind:value={overflowValue} />
  </SpecimenGroup>

  <SpecimenGroup label="Max 2 clauses">
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

  <SpecimenGroup label="Disabled">
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

<script lang="ts">
  import { Select, Eyebrow, type SelectOption, type SelectOptionGroup } from "@poodle/svelte-primitives";

  const fruitOptions: SelectOption[] = [
    { value: "apple", label: "Apple" },
    { value: "banana", label: "Banana" },
    { value: "cherry", label: "Cherry" },
    { value: "dragonfruit", label: "Dragonfruit" },
    { value: "elderberry", label: "Elderberry" },
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
        { value: "spinach", label: "Spinach", isDisabled: true },
      ],
    },
    {
      label: "Grains",
      options: [
        { value: "rice", label: "Rice" },
        { value: "wheat", label: "Wheat" },
      ],
    },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selectedFruit: string | null = null;
  let selectedGrouped: string | null = null;
  let selectedClearable: string | null = "all";
  let selectedLazy: string | null = "module-1";

  async function loadModuleGroups() {
    return [
      {
        label: "",
        items: [{ value: "all", label: "All modules" }],
      },
      {
        label: "Pathway A",
        items: [
          { value: "module-1", label: "FA1 Financial Reporting" },
          { value: "module-2", label: "TX1 Taxation" },
        ],
      },
      {
        label: "Pathway B",
        items: [{ value: "module-3", label: "AA1 Audit" }],
      },
    ];
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (flat options)</Eyebrow>
    <div class="specimen__field">
      <Select
        id="select-default"
        options={fruitOptions}
        placeholder="Choose a fruit"
        ariaLabel="Fruit selection"
        on:valueChange={(event) => (selectedFruit = event.detail.value)}
      />
      {#if selectedFruit}
        <p class="specimen__value">Selected: {selectedFruit}</p>
      {/if}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Grouped options</Eyebrow>
    <div class="specimen__field">
      <Select
        id="select-grouped"
        options={groupedOptions}
        placeholder="Choose a food"
        ariaLabel="Food selection with groups"
        on:valueChange={(event) => (selectedGrouped = event.detail.value)}
      />
      {#if selectedGrouped}
        <p class="specimen__value">Selected: {selectedGrouped}</p>
      {/if}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Select
          id={"select-size-" + size}
          options={fruitOptions}
          placeholder={size.toUpperCase()}
          ariaLabel={"Fruit at " + size}
          {size}
        />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Select
            id={"select-density-" + density}
            options={fruitOptions}
            placeholder="Choose a fruit"
            ariaLabel={"Fruit at " + density + " density"}
            {density}
          />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Select
      id="select-disabled"
      options={fruitOptions}
      value="banana"
      disabled
      ariaLabel="Disabled fruit selection"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Clearable</Eyebrow>
    <Select
      id="select-clearable"
      items={fruitOptions}
      clearable
      defaultValue="all"
      placeholder="All fruits"
      value={selectedClearable}
      ariaLabel="Clearable fruit selection"
      on:valueChange={(event) => (selectedClearable = event.detail.value)}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Lazy grouped options</Eyebrow>
    <Select
      id="select-lazy"
      value={selectedLazy}
      valueLabel="FA1 Financial Reporting"
      placeholder="All modules"
      clearable
      defaultValue="all"
      loadGroups={loadModuleGroups}
      ariaLabel="Lazy module selection"
      on:valueChange={(event) => (selectedLazy = event.detail.value)}
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 20rem;
  }

  .specimen__field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .specimen__value {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    margin: 0;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>

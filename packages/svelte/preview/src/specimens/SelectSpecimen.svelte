<script lang="ts">
  import { Select, Pill, type SelectOption, type SelectOptionGroup } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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

  let selectedFruit: string | null = null;
  let selectedCountry: string | null = null;
  let selectedFramework: string | null = null;
  let selectedGrouped: string | null = null;
  let freeformValue: string | null = null;
</script>

<div class="specimen">
  <SpecimenGroup label="Native (default)">
    <div class="specimen__field">
      <Select
        id="select-native"
        options={fruitOptions}
        placeholder="Choose a fruit"
        ariaLabel="Fruit selection"
        on:valueChange={(e) => (selectedFruit = e.detail.value)}
      />
      {#if selectedFruit}
        <p class="specimen__value">Selected: {selectedFruit}</p>
      {/if}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Custom dropdown (non-searchable)">
    <div class="specimen__field">
      <Select
        id="select-custom"
        options={richOptions}
        placeholder="Choose a country"
        native={false}
        ariaLabel="Country selection"
        on:valueChange={(e) => (selectedCountry = e.detail.value)}
      />
      {#if selectedCountry}
        <p class="specimen__value">Selected: {selectedCountry}</p>
      {/if}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Searchable">
    <div class="specimen__field">
      <Select
        id="select-searchable"
        options={frameworkOptions}
        placeholder="Search frameworks..."
        searchable
        ariaLabel="Framework search"
        on:valueChange={(e) => (selectedFramework = e.detail.value)}
      />
      {#if selectedFramework}
        <p class="specimen__value">Selected: {selectedFramework}</p>
      {/if}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Searchable with groups">
    <div class="specimen__field">
      <Select
        id="select-searchable-grouped"
        options={groupedOptions}
        placeholder="Search food..."
        searchable
        ariaLabel="Food search"
        on:valueChange={(e) => (selectedGrouped = e.detail.value)}
      />
      {#if selectedGrouped}
        <p class="specimen__value">Selected: {selectedGrouped}</p>
      {/if}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Freeform (autocomplete)">
    <div class="specimen__field">
      <Select
        id="select-freeform"
        options={frameworkOptions}
        placeholder="Type or select..."
        searchable
        freeform
        ariaLabel="Framework freeform"
        on:valueChange={(e) => (freeformValue = e.detail.value)}
      />
      {#if freeformValue}
        <p class="specimen__value">Value: {freeformValue}</p>
      {/if}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Rich option rendering (custom slot)">
    <Select
      id="select-rich"
      options={richOptions}
      placeholder="Choose a country"
      ariaLabel="Country with rich options"
    >
      <div slot="option" let:option let:selected class="rich-option">
        <span class="rich-option__flag">{option.icon ? "🌍" : ""}</span>
        <span class="rich-option__content">
          <span class="rich-option__label">{option.label}</span>
          {#if option.description}
            <span class="rich-option__desc">{option.description}</span>
          {/if}
        </span>
        {#if selected}
          <Pill tone="info" appearance="badge" sizeRole="chrome">✓</Pill>
        {/if}
      </div>
    </Select>
  </SpecimenGroup>

  <SpecimenGroup label="Clearable (custom)">
    <Select
      id="select-clearable"
      options={fruitOptions}
      placeholder="All fruits"
      native={false}
      clearable
      ariaLabel="Clearable fruit selection"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Native grouped">
    <Select
      id="select-native-grouped"
      options={groupedOptions}
      placeholder="Choose a food"
      ariaLabel="Grouped food selection"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Select
      id="select-disabled"
      options={fruitOptions}
      value="banana"
      disabled
      ariaLabel="Disabled"
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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

  .rich-option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
  }

  .rich-option__flag {
    font-size: 1rem;
    flex-shrink: 0;
  }

  .rich-option__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
    min-width: 0;
  }

  .rich-option__label {
    font-size: 0.8125rem;
  }

  .rich-option__desc {
    font-size: 0.6875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

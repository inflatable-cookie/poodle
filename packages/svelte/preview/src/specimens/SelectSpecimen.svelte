<script lang="ts">
  import { Select, Pill, Eyebrow, Surface, type SelectOption, type SelectOptionGroup } from "@poodle/svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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
  let freeformValue: string | null = null;
</script>

<SpecimenLayout>
  <Surface tone="panel" border="subtle" padding="md">
    <div class="poodle-specimen">
      <div class="poodle-specimen__row">
        <Eyebrow>Native</Eyebrow>
        <div class="poodle-specimen__field">
          <Select
            options={fruitOptions}
            placeholder="Choose a fruit"
            ariaLabel="Fruit selection"
            on:valueChange={(e) => (selectedFruit = e.detail.value)}
          />
          {#if selectedFruit}<span class="poodle-specimen__value">{selectedFruit}</span>{/if}
        </div>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Custom dropdown</Eyebrow>
        <div class="poodle-specimen__field">
          <Select
            options={richOptions}
            placeholder="Choose a country"
            native={false}
            ariaLabel="Country selection"
            on:valueChange={(e) => (selectedCountry = e.detail.value)}
          />
          {#if selectedCountry}<span class="poodle-specimen__value">{selectedCountry}</span>{/if}
        </div>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Searchable</Eyebrow>
        <div class="poodle-specimen__field">
          <Select
            options={frameworkOptions}
            placeholder="Search frameworks..."
            searchable
            ariaLabel="Framework search"
            on:valueChange={(e) => (selectedFramework = e.detail.value)}
          />
          {#if selectedFramework}<span class="poodle-specimen__value">{selectedFramework}</span>{/if}
        </div>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Freeform</Eyebrow>
        <div class="poodle-specimen__field">
          <Select
            options={frameworkOptions}
            placeholder="Type or select..."
            searchable
            freeform
            ariaLabel="Freeform"
            on:valueChange={(e) => (freeformValue = e.detail.value)}
          />
          {#if freeformValue}<span class="poodle-specimen__value">{freeformValue}</span>{/if}
        </div>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Grouped</Eyebrow>
        <Select
          options={groupedOptions}
          placeholder="Choose a food"
          ariaLabel="Grouped food"
        />
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Clearable</Eyebrow>
        <Select
          options={fruitOptions}
          placeholder="All fruits"
          native={false}
          clearable
          ariaLabel="Clearable"
        />
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Disabled</Eyebrow>
        <Select
          options={fruitOptions}
          value="banana"
          disabled
          ariaLabel="Disabled"
        />
      </div>
    </div>
  </Surface>

  <svelte:fragment slot="sizes" let:size>
    <Select options={fruitOptions} placeholder="Select..." {size} ariaLabel="{size} select" />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Select options={fruitOptions} placeholder="Select..." {density} ariaLabel="{density} select" />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .poodle-specimen__field {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    min-width: 12rem;
  }

  .poodle-specimen__value {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

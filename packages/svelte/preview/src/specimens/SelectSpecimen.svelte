<script lang="ts">
  import { Select, Pill, Eyebrow, Surface, type SelectOption, type SelectOptionGroup } from "@inflatable-cookie/poodle-svelte";
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

  let selectedFruit: string | null = $state(null);
  let selectedCountry: string | null = $state(null);
  let selectedFramework: string | null = $state(null);
  let freeformValue: string | null = $state(null);
  let customCountry: string | null = $state(null);
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
            onValueChange={(nextValue) => (selectedFruit = nextValue)}
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
            onValueChange={(nextValue) => (selectedCountry = nextValue)}
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
            onValueChange={(nextValue) => (selectedFramework = nextValue)}
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
            onValueChange={(nextValue) => (freeformValue = nextValue)}
          />
          {#if freeformValue}<span class="poodle-specimen__value">{freeformValue}</span>{/if}
        </div>
      </div>

      <div class="poodle-specimen__row">
        <Eyebrow>Snippet rendering</Eyebrow>
        <div class="poodle-specimen__field">
          <Select
            options={richOptions}
            placeholder="Custom country"
            native={false}
            ariaLabel="Custom country selection"
            onValueChange={(nextValue) => (customCountry = nextValue)}
          >
            {#snippet trigger({ selectedOption, placeholder })}
              <span class="poodle-specimen__trigger">
                <span>{selectedOption?.label ?? placeholder ?? ""}</span>
                {#if selectedOption}
                  <Pill size="sm" appearance="subtle" tone="info">picked</Pill>
                {/if}
              </span>
            {/snippet}

            {#snippet option({ option, selected })}
              <span class="poodle-specimen__option">
                <span class="poodle-specimen__option-body">
                  <span class="poodle-specimen__option-label">{option.label}</span>
                  {#if option.description}
                    <span class="poodle-specimen__option-description">{option.description}</span>
                  {/if}
                </span>
              </span>
            {/snippet}

            {#snippet empty({ query })}
              <div class="poodle-specimen__empty">No match for "{query}"</div>
            {/snippet}
          </Select>
          {#if customCountry}<span class="poodle-specimen__value">{customCountry}</span>{/if}
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

  {#snippet sizes(size)}
    <Select options={fruitOptions} placeholder="Select..." {size} ariaLabel="{size} select" />
  {/snippet}

  {#snippet densities(density)}
    <Select options={fruitOptions} placeholder="Select..." {density} ariaLabel="{density} select" />
  {/snippet}
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

  .poodle-specimen__trigger {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .poodle-specimen__option {
    width: 100%;
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .poodle-specimen__option-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .poodle-specimen__option-label {
    min-width: 0;
  }

  .poodle-specimen__option-description,
  .poodle-specimen__empty {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

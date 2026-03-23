<script lang="ts">
  import { Select, Eyebrow, type SelectOption, type SelectOptionGroup } from "@flint/svelte-primitives";

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

  let selectedFruit: string | null = null;
  let selectedGrouped: string | null = null;
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
    <Eyebrow>Disabled</Eyebrow>
    <Select
      id="select-disabled"
      options={fruitOptions}
      value="banana"
      isDisabled
      ariaLabel="Disabled fruit selection"
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
    color: var(--flint-color-text-secondary);
    margin: 0;
  }
</style>

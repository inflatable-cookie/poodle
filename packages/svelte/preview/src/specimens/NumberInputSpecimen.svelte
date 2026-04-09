<script lang="ts">
  import { NumberInput } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let quantity: number | null = 1;
  let price: number | null = 29.99;
  let ticketCode = "12";
</script>

<div class="specimen">
  <SpecimenGroup label="Numeric Value">
    <NumberInput
      id="qty"
      bind:value={quantity}
      min={0}
      max={100}
      ariaLabel="Quantity"
    />
    <p>Quantity: <strong>{quantity ?? "none"}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <NumberInput id={"size-" + size} value={1} ariaLabel={"Number at " + size} {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <NumberInput id={"density-" + density} value={1} ariaLabel={"Number at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With Steppers">
    <NumberInput
      id="price"
      bind:value={price}
      min={0}
      step={0.01}
      precision={2}
      showSteppers
      ariaLabel="Price"
    />
    <p>Price: <strong>{price == null ? "none" : `$${price.toFixed(2)}`}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="String Form Binding">
    <NumberInput
      id="ticket-code"
      bind:value={ticketCode}
      prefix="A"
      min={1}
      max={999}
      ariaLabel="Ticket code"
    />
    <p>Ticket code: <strong>{ticketCode}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <NumberInput id="disabled-num" value={42} ariaLabel="Disabled" disabled />
  </SpecimenGroup>

  <SpecimenGroup label="Invalid">
    <NumberInput id="invalid-num" value={-5} min={0} ariaLabel="Invalid number" validationState="invalid" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 16rem;
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

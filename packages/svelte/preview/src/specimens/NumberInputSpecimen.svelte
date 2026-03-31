<script lang="ts">
  import { NumberInput, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let quantity: number | null = 1;
  let price: number | null = 29.99;
  let ticketCode = "12";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Numeric Value</Eyebrow>
    <NumberInput
      id="qty"
      bind:value={quantity}
      min={0}
      max={100}
      ariaLabel="Quantity"
    />
    <p>Quantity: <strong>{quantity ?? "none"}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <NumberInput id={"size-" + size} value={1} ariaLabel={"Number at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <NumberInput id={"density-" + density} value={1} ariaLabel={"Number at " + density + " density"} {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With Steppers</Eyebrow>
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
  </div>

  <div class="specimen__group">
    <Eyebrow>String Form Binding</Eyebrow>
    <NumberInput
      id="ticket-code"
      bind:value={ticketCode}
      prefix="A"
      min={1}
      max={999}
      ariaLabel="Ticket code"
    />
    <p>Ticket code: <strong>{ticketCode}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <NumberInput id="disabled-num" value={42} ariaLabel="Disabled" disabled />
  </div>

  <div class="specimen__group">
    <Eyebrow>Invalid</Eyebrow>
    <NumberInput id="invalid-num" value={-5} min={0} ariaLabel="Invalid number" validationState="invalid" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 16rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
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

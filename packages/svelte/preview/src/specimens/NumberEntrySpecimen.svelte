<script lang="ts">
  import { NumberEntry, Eyebrow } from "@poodle/svelte-primitives";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let quantity = 1;
  let price = 29.99;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <NumberEntry
      id="qty"
      value={quantity}
      min={0}
      max={100}
      ariaLabel="Quantity"
      on:valueChange={(e) => { if (e.detail.value != null) quantity = e.detail.value; }}
    />
    <p>Quantity: <strong>{quantity}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__stack">
      {#each controlSizes as size}
        <NumberEntry id={"size-" + size} value={1} ariaLabel={"Number at " + size} {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With steppers</Eyebrow>
    <NumberEntry
      id="price"
      value={price}
      min={0}
      step={0.01}
      precision={2}
      showSteppers
      ariaLabel="Price"
      on:valueChange={(e) => { if (e.detail.value != null) price = e.detail.value; }}
    />
    <p>Price: <strong>${price.toFixed(2)}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <NumberEntry id="disabled-num" value={42} ariaLabel="Disabled" disabled />
  </div>

  <div class="specimen__group">
    <Eyebrow>Invalid</Eyebrow>
    <NumberEntry id="invalid-num" value={-5} min={0} ariaLabel="Invalid number" validationState="invalid" />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 14rem;
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
</style>

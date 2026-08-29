<script lang="ts">
  import { NumberInput } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let quantity: number | null = $state(1);
  let price: number | null = $state(29.99);
  let empty: number | null = $state(null);
  let emptyDraft: string | null = $state(null);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Numeric Value">
    <div class="poodle-specimen__control">
      <NumberInput
        id="qty"
        bind:value={quantity}
        min={0}
        max={100}
        ariaLabel="Quantity"
      />
    </div>
    <p>Quantity: <strong>{quantity ?? "none"}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Precision And Steppers">
    <div class="poodle-specimen__control">
      <NumberInput
        id="price"
        bind:value={price}
        min={0}
        step={0.01}
        precision={2}
        prefix="$"
        showSteppers
        ariaLabel="Price"
      />
    </div>
    <p>Price: <strong>{price == null ? "none" : `$${price.toFixed(2)}`}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Empty With Draft Channel">
    <div class="poodle-specimen__control">
      <NumberInput
        id="empty-num"
        bind:value={empty}
        bind:draftValue={emptyDraft}
        placeholder="Type a number"
        ariaLabel="Optional amount"
      />
    </div>
    <p>
      Value: <strong>{empty ?? "none"}</strong>
      · Draft: <strong>{emptyDraft === null ? "adapter-owned" : JSON.stringify(emptyDraft)}</strong>
    </p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <NumberInput id="disabled-num" value={42} ariaLabel="Disabled" disabled />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Invalid Presentation">
    <div class="poodle-specimen__control">
      <NumberInput id="invalid-num" value={-5} min={0} ariaLabel="Invalid number" validationState="invalid" />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <NumberInput id={"size-" + size} value={1} {size} ariaLabel={"Number at " + size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <NumberInput id={"density-" + density} value={1} {density} ariaLabel={"Number at " + density + " density"} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>

<script lang="ts">
  import { NumberInput } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let quantity: number | null = 1;
  let price: number | null = 29.99;
  let ticketCode = "12";
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

  <SpecimenGroup label="With Steppers">
    <div class="poodle-specimen__control">
      <NumberInput
        id="price"
        bind:value={price}
        min={0}
        step={0.01}
        precision={2}
        showSteppers
        ariaLabel="Price"
      />
    </div>
    <p>Price: <strong>{price == null ? "none" : `$${price.toFixed(2)}`}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="String Form Binding">
    <div class="poodle-specimen__control">
      <NumberInput
        id="ticket-code"
        bind:value={ticketCode}
        prefix="A"
        min={1}
        max={999}
        ariaLabel="Ticket code"
      />
    </div>
    <p>Ticket code: <strong>{ticketCode}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
      <NumberInput id="disabled-num" value={42} ariaLabel="Disabled" disabled />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Invalid">
    <div class="poodle-specimen__control">
      <NumberInput id="invalid-num" value={-5} min={0} ariaLabel="Invalid number" validationState="invalid" />
    </div>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <div class="poodle-specimen__control">
      <NumberInput id={"size-" + size} value={1} {size} ariaLabel={"Number at " + size} />
    </div>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <div class="poodle-specimen__control">
      <NumberInput id={"density-" + density} value={1} {density} ariaLabel={"Number at " + density + " density"} />
    </div>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>

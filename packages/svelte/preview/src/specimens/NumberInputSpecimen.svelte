<script lang="ts">
  import { NumberInput } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let quantity: number | null = 1;
  let price: number | null = 29.99;
  let ticketCode = "12";
</script>

<SpecimenLayout>
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

  <svelte:fragment slot="sizes" let:size>
    <NumberInput id={"size-" + size} value={1} ariaLabel={"Number at " + size} {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <NumberInput id={"density-" + density} value={1} ariaLabel={"Number at " + density + " density"} {density} />
  </svelte:fragment>
</SpecimenLayout>

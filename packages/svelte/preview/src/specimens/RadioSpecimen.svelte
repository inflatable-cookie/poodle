<script lang="ts">
  import { Radio } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let shipping = "standard";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Radio
      name="specimen-shipping"
      value="standard"
      label="Standard shipping"
      checked={shipping === "standard"}
      onCheckedChange={(checked) => checked && (shipping = "standard")}
    />
    <Radio
      name="specimen-shipping"
      value="express"
      label="Express shipping"
      checked={shipping === "express"}
      onCheckedChange={(checked) => checked && (shipping = "express")}
    />
    <Radio
      name="specimen-shipping"
      value="overnight"
      label="Overnight shipping"
      checked={shipping === "overnight"}
      onCheckedChange={(checked) => checked && (shipping = "overnight")}
    />
  </SpecimenGroup>

  <SpecimenGroup label="States">
    <Radio name="specimen-states-a" label="Disabled unchecked" disabled />
    <Radio name="specimen-states-b" label="Disabled checked" checked={true} disabled />
    <Radio name="specimen-states-c" label="Read-only checked" checked={true} readOnly />
    <Radio name="specimen-states-d" label="Custom selected color" checked={true} selectedColor="#22c55e" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Radio name={"specimen-size-" + size} label="Option" checked={true} {size} />
  {/snippet}

  {#snippet densities(density)}
    <Radio name={"specimen-density-" + density} label="Option" {density} />
  {/snippet}
</SpecimenLayout>

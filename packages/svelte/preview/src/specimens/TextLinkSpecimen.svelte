<script lang="ts">
  import { Text, TextLink } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Inline prose">
    <Text>
      Review the <TextLink href="#contract">component contract</TextLink> before
      wiring the production route.
    </Text>
  </SpecimenGroup>

  <SpecimenGroup label="Tones">
    <div class="poodle-specimen__row">
      <TextLink href="#accent">Accent link</TextLink>
      <TextLink href="#secondary" tone="secondary">Secondary link</TextLink>
      <TextLink href="#inherit" tone="inherit">Inherited link</TextLink>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Button action">
    <TextLink onClick={() => undefined}>Open inline action</TextLink>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__row">
      <TextLink href="#disabled" disabled>Disabled anchor</TextLink>
      <TextLink disabled>Disabled action</TextLink>
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
</style>

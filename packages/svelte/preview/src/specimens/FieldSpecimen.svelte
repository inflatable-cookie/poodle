<script lang="ts">
  import { Field, TextInput } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Default with description">
        <Field label="Display name" description="This is how your name appears to other users." id="field-name">
          {#snippet control({ describedBy })}
            <TextInput
              id="field-name"
              placeholder="Enter your name"
              ariaLabel="Display name"
              {describedBy}
            />
          {/snippet}
        </Field>
      </SpecimenGroup>

      <SpecimenGroup label="Required">
        <Field label="Email address" required id="field-email">
          <TextInput id="field-email" placeholder="you@example.com" ariaLabel="Email address" />
        </Field>
      </SpecimenGroup>

      <SpecimenGroup label="With error">
        <Field label="Username" error="This username is already taken." validationState="invalid" id="field-user">
          {#snippet control({ describedBy, validationState })}
            <TextInput
              id="field-user"
              value="admin"
              ariaLabel="Username"
              {describedBy}
              {validationState}
            />
          {/snippet}
        </Field>
      </SpecimenGroup>

      <SpecimenGroup label="Optional">
        <Field label="Phone number" optionalLabel="optional" id="field-phone">
          <TextInput id="field-phone" placeholder="+1 (555) 000-0000" ariaLabel="Phone number" />
        </Field>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-field-specimen__variant-block">
      <div class="poodle-field-specimen__label">{size.toUpperCase()}</div>
      <div class="poodle-field-specimen__variant">
        <Field
          id={`field-size-${size}`}
          label="Display name"
          description="This is how your name appears to other users."
          size={size}
        >
          <TextInput
            id={`field-size-${size}`}
            placeholder="Enter your name"
            ariaLabel="Display name"
          />
        </Field>
      </div>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-field-specimen__variant-block">
      <div class="poodle-field-specimen__label">{density.toUpperCase()}</div>
      <div class="poodle-field-specimen__variant">
        <Field
          id={`field-density-${density}`}
          label="Display name"
          description="This is how your name appears to other users."
          density={density}
        >
          <TextInput
            id={`field-density-${density}`}
            placeholder="Enter your name"
            ariaLabel="Display name"
          />
        </Field>
      </div>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 24rem;
  }

  .poodle-field-specimen__variant {
    width: min(24rem, 100%);
  }

  .poodle-field-specimen__variant-block {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: min(24rem, 100%);
  }

  .poodle-field-specimen__label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
</style>

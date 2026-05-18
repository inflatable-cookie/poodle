<script lang="ts">
  import { PasswordRequirements, TextInput } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let password = $state("");

  const requirements = {
    minLength: 12,
    requireMixedCase: true,
    requireDigit: true,
    requireSpecial: true,
    minStrengthScore: 3,
    description: "Use a long password with varied character types."
  };
</script>

<SpecimenLayout showSizes={true} showDensities={false}>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Default">
        <div class="poodle-password-requirements-specimen__field">
          <label for="password-requirements-specimen-password">Password</label>
          <TextInput id="password-requirements-specimen-password" bind:value={password} type="password" />
        </div>
        <PasswordRequirements {password} {requirements} />
      </SpecimenGroup>

      <SpecimenGroup label="Loading">
        <PasswordRequirements password="" requirements={null} loading />
      </SpecimenGroup>

      <SpecimenGroup label="Error">
        <PasswordRequirements
          password=""
          requirements={null}
          error="Could not load password requirements."
        />
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-password-requirements-specimen__variant">
      <PasswordRequirements password="Example123!" {requirements} {size} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 28rem;
  }

  .poodle-password-requirements-specimen__field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .poodle-password-requirements-specimen__field label {
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-password-requirements-specimen__variant {
    width: min(100%, 28rem);
  }
</style>

<script lang="ts">
  import { TextInput, Field } from "@inflatable-cookie/poodle-svelte";
  import type { InputValidationStatus, ValidationState } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let name = $state("");
  let email = $state("invalid-email");
  let validationState: ValidationState = $state("invalid");
  let workspace = "acme-admin";
  let workspaceStatus: InputValidationStatus = "idle";
  let workspaceError: string | null = null;
  let slug = $state("");
  let slugStatus: InputValidationStatus = $state("idle");
  let searchQuery = $state("");

  async function validateSlug(value: string) {
    await new Promise((resolve) => setTimeout(resolve, 250));
    if (value === "northstar") return { valid: false, message: "That slug is already in use." };
    return { valid: true };
  }

  async function validateWorkspace(value: string) {
    await new Promise((resolve) => setTimeout(resolve, 250));
    if (["admin", "settings", "support"].includes(value.trim().toLowerCase())) {
      return { valid: false, message: "That workspace handle is reserved." };
    }
    return { valid: true };
  }
</script>

<SpecimenLayout>
    <SpecimenGroup label="Default">
    <div class="poodle-specimen__control">
          <Field id="name-field" label="Name" description="Enter your full name.">
            <TextInput id="name-field" placeholder="Jane Doe" onValueChange={(nextValue) => (name = nextValue)} />
          </Field>
        </div>
  </SpecimenGroup>

        <SpecimenGroup label="With validation">
    <div class="poodle-specimen__control">
          <Field id="email-field" label="Email" description="A valid email address is required." validationState={validationState} error={validationState === "invalid" ? "Please enter a valid email address." : null}>
            <TextInput id="email-field" value={email} {validationState} onValueChange={(nextValue) => { email = nextValue; validationState = nextValue.includes("@") ? "valid" : "invalid"; }} />
          </Field>
        </div>
  </SpecimenGroup>

        <SpecimenGroup label="Slug">
    <div class="poodle-specimen__control">
          <Field id="slug-field" label="Slug" description="Generates from the title until the user edits it." validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"} error={slugStatus === "invalid" ? "That slug is not available." : null}>
            <TextInput id="slug-field" type="slug" value={slug} source="Northstar Launch Plan" prefix="/projects/" maxLength={64} validate={validateSlug} onValueChange={(nextValue) => (slug = nextValue)} onValidationChange={(detail) => { slugStatus = detail.status; }} />
          </Field>
        </div>
  </SpecimenGroup>

        <SpecimenGroup label="Search">
    <div class="poodle-specimen__control">
          <TextInput id="search-field" type="search" placeholder="Search..." value={searchQuery} onValueChange={(nextValue) => (searchQuery = nextValue)} onClear={() => (searchQuery = "")} />
        </div>
  </SpecimenGroup>

        <SpecimenGroup label="Prefix and suffix">
    <div class="poodle-specimen__control">
          <TextInput id="price-field" prefix="$" suffix="USD" placeholder="0.00" inputMode="decimal" />
        </div>
  </SpecimenGroup>

        <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__control">
          <Field id="disabled-field" label="API key">
            <TextInput id="disabled-field" value="sk-xxxx-xxxx-xxxx" disabled />
          </Field>
        </div>
  </SpecimenGroup>

        <SpecimenGroup label="Multiline">
    <div class="poodle-specimen__control">
          <Field id="multiline-field" label="Description">
            <TextInput id="multiline-field" type="multiline" rows={3} maxLength={280} showCharCount placeholder="Enter a description..." />
          </Field>
        </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__control">
      <TextInput id={"size-" + size} {size} placeholder={size.toUpperCase()} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__control">
      <TextInput id={"density-" + density} {density} placeholder="Type here" />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__control {
    max-width: 20rem;
  }
</style>

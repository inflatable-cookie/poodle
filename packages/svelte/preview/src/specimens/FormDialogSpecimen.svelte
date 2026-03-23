<script lang="ts">
  import { FormDialog } from "@poodle/svelte-composites";
  import { Button, Eyebrow, TextInput, Field, Select } from "@poodle/svelte-primitives";

  let basicOpen: boolean | null = null;
  let errorOpen: boolean | null = null;
  let submitting = false;
  let error: string | null = null;
  let name = "";
  let role = "";
  let lastAction = "";

  const roleOptions = [
    { value: "admin", label: "Admin" },
    { value: "editor", label: "Editor" },
    { value: "viewer", label: "Viewer" },
  ];

  function handleBasicSubmit(): void {
    submitting = true;
    setTimeout(() => {
      submitting = false;
      lastAction = `Created user: ${name} (${role || "viewer"})`;
      basicOpen = false;
      name = "";
      role = "";
    }, 1200);
  }

  function handleErrorSubmit(): void {
    submitting = true;
    setTimeout(() => {
      submitting = false;
      error = "A user with this email already exists.";
    }, 800);
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic form dialog</Eyebrow>
    <Button variant="primary" on:click={() => (basicOpen = true)}>Add user</Button>
    <FormDialog
      open={basicOpen}
      title="Add new user"
      description="Invite a user to this workspace."
      submitLabel="Add user"
      {submitting}
      on:submit={handleBasicSubmit}
      on:cancel={() => (basicOpen = false)}
      on:openChange={(e) => (basicOpen = e.detail.open ? true : null)}
    >
      <Field label="Full name">
        <TextInput bind:value={name} placeholder="Enter name" />
      </Field>
      <Field label="Role">
        <Select options={roleOptions} bind:value={role} placeholder="Select role" />
      </Field>
    </FormDialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>With error state</Eyebrow>
    <Button variant="secondary" on:click={() => { errorOpen = true; error = null; }}>Try with error</Button>
    <FormDialog
      open={errorOpen}
      title="Create account"
      submitLabel="Create"
      {submitting}
      {error}
      on:submit={handleErrorSubmit}
      on:cancel={() => { errorOpen = false; error = null; }}
      on:openChange={(e) => { if (!e.detail.open) { errorOpen = null; error = null; } }}
    >
      <Field label="Email">
        <TextInput value="existing@example.com" placeholder="Enter email" />
      </Field>
    </FormDialog>
  </div>

  {#if lastAction}
    <div class="specimen__group">
      <Eyebrow>Last action</Eyebrow>
      <p>{lastAction}</p>
    </div>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  p {
    margin: 0;
  }
</style>

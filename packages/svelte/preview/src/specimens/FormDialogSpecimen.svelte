<script lang="ts">
  import { FormDialog } from "@poodle/svelte-composites";
  import { Button, Eyebrow, TextInput, Field, Select, FormActions } from "@poodle/svelte-primitives";

  let basicOpen: boolean | null = null;
  let errorOpen: boolean | null = null;
  let shellOpen: boolean | null = null;
  let submitting = false;
  let error: string | null = null;
  let success: string | null = null;
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

  function handleShellSubmit(): void {
    submitting = true;
    success = null;
    setTimeout(() => {
      submitting = false;
      success = "Settings saved successfully.";
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

  <div class="specimen__group">
    <Eyebrow>Shell mode with custom actions</Eyebrow>
    <Button variant="ghost" on:click={() => { shellOpen = true; success = null; }}>Open settings shell</Button>
    <FormDialog
      open={shellOpen}
      title="Edit workspace settings"
      subtitle="Update shared defaults for this workspace."
      width="40rem"
      {submitting}
      success={success}
      showDefaultActions={false}
      on:cancel={() => { shellOpen = false; success = null; }}
      on:openChange={(e) => { shellOpen = e.detail.open ? true : null; }}
    >
      <Field label="Workspace name">
        <TextInput value="Northstar" disabled={submitting} />
      </Field>
      <Field label="Default role">
        <Select options={roleOptions} value="editor" disabled={submitting} />
      </Field>
      <svelte:fragment slot="actions">
        <FormActions align="end">
          <Button variant="ghost" on:click={() => { shellOpen = false; success = null; }} disabled={submitting}>Cancel</Button>
          <Button variant="primary" on:click={handleShellSubmit} disabled={submitting}>
            {submitting ? "Saving..." : "Save changes"}
          </Button>
        </FormActions>
      </svelte:fragment>
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

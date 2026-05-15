<script lang="ts">
  import { FormDialog } from "@poodle/svelte";
  import { Button, TextInput, Field, Select, FormActions } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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

<div class="poodle-specimen">
  <SpecimenGroup label="Basic form dialog">
    <Button variant="primary" onClick={() => (basicOpen = true)}>Add user</Button>
    <FormDialog
      open={basicOpen}
      title="Add new user"
      description="Invite a user to this workspace."
      submitLabel="Add user"
      {submitting}
      onSubmit={handleBasicSubmit}
      onCancel={() => (basicOpen = false)}
      onOpenChange={(open) => (basicOpen = open ? true : null)}
    >
      <Field label="Full name" id="form-dialog-full-name">
        <TextInput bind:value={name} placeholder="Enter name" />
      </Field>
      <Field label="Role" id="form-dialog-role">
        <Select options={roleOptions} bind:value={role} placeholder="Select role" />
      </Field>
    </FormDialog>
  </SpecimenGroup>

  <SpecimenGroup label="With error state">
    <Button variant="secondary" onClick={() => { errorOpen = true; error = null; }}>Try with error</Button>
    <FormDialog
      open={errorOpen}
      title="Create account"
      submitLabel="Create"
      {submitting}
      {error}
      onSubmit={handleErrorSubmit}
      onCancel={() => { errorOpen = false; error = null; }}
      onOpenChange={(open) => { if (!open) { errorOpen = null; error = null; } }}
    >
      <Field label="Email" id="form-dialog-email">
        <TextInput value="existing@example.com" placeholder="Enter email" />
      </Field>
    </FormDialog>
  </SpecimenGroup>

  <SpecimenGroup label="Shell mode with custom actions">
    <Button variant="ghost" onClick={() => { shellOpen = true; success = null; }}>Open settings shell</Button>
    <FormDialog
      open={shellOpen}
      title="Edit workspace settings"
      subtitle="Update shared defaults for this workspace."
      width="40rem"
      {submitting}
      success={success}
      showDefaultActions={false}
      onCancel={() => { shellOpen = false; success = null; }}
      onOpenChange={(open) => { shellOpen = open ? true : null; }}
    >
      {#snippet body(submitting)}
        <Field label="Workspace name" id="form-dialog-workspace-name">
          <TextInput value="Northstar" disabled={submitting} />
        </Field>
        <Field label="Default role" id="form-dialog-default-role">
          <Select options={roleOptions} value="editor" disabled={submitting} />
        </Field>
      {/snippet}
      {#snippet actions(submitting)}
        <FormActions align="end">
          <Button variant="ghost" onClick={() => { shellOpen = false; success = null; }} disabled={submitting}>Cancel</Button>
          <Button variant="primary" onClick={handleShellSubmit} disabled={submitting}>
            {submitting ? "Saving..." : "Save changes"}
          </Button>
        </FormActions>
      {/snippet}
    </FormDialog>
  </SpecimenGroup>

  {#if lastAction}
    <SpecimenGroup label="Last action">
      <p>{lastAction}</p>
    </SpecimenGroup>
  {/if}
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  p {
    margin: 0;
  }
</style>

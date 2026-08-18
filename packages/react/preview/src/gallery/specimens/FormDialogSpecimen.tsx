import { useState } from "react";
import { FormDialog, Button, TextInput, Field, Select, FormActions } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const roleOptions = [
  { value: "admin", label: "Admin" },
  { value: "editor", label: "Editor" },
  { value: "viewer", label: "Viewer" },
];

export function FormDialogSpecimen() {
  const [basicOpen, setBasicOpen] = useState<boolean | null>(null);
  const [errorOpen, setErrorOpen] = useState<boolean | null>(null);
  const [shellOpen, setShellOpen] = useState<boolean | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [role, setRole] = useState("");
  const [lastAction, setLastAction] = useState("");
  const [axisOpen, setAxisOpen] = useState<Record<string, boolean>>({});

  const setAxis = (key: string, open: boolean) => setAxisOpen((prev) => ({ ...prev, [key]: open }));

  function handleBasicSubmit(): void {
    setSubmitting(true);
    setTimeout(() => {
      setSubmitting(false);
      setLastAction(`Created user: ${name} (${role || "viewer"})`);
      setBasicOpen(false);
      setName("");
      setRole("");
    }, 1200);
  }

  function handleErrorSubmit(): void {
    setSubmitting(true);
    setTimeout(() => {
      setSubmitting(false);
      setError("A user with this email already exists.");
    }, 800);
  }

  function handleShellSubmit(): void {
    setSubmitting(true);
    setSuccess(null);
    setTimeout(() => {
      setSubmitting(false);
      setSuccess("Settings saved successfully.");
    }, 800);
  }

  return (
    <SpecimenLayout
      sizes={(size) => (
        <>
          <Button variant="secondary" onClick={() => setAxis(`size-${size}`, true)}>
            Open {size} dialog
          </Button>
          <FormDialog
            open={axisOpen[`size-${size}`] ?? false}
            onOpenChange={(open) => setAxis(`size-${size}`, open)}
            title="Add new user"
            description="Invite a user to this workspace."
            submitLabel="Add user"
            size={size}
            onSubmit={() => {}}
          >
            <Field label="Full name" id={`form-dialog-axis-name-${size}`}>
              <TextInput placeholder="Enter name" />
            </Field>
          </FormDialog>
        </>
      )}
      densities={(density) => (
        <>
          <Button variant="secondary" onClick={() => setAxis(`density-${density}`, true)}>
            Open {density} dialog
          </Button>
          <FormDialog
            open={axisOpen[`density-${density}`] ?? false}
            onOpenChange={(open) => setAxis(`density-${density}`, open)}
            title="Add new user"
            description="Invite a user to this workspace."
            submitLabel="Add user"
            density={density}
            onSubmit={() => {}}
          >
            <Field label="Full name" id={`form-dialog-axis-name-${density}`}>
              <TextInput placeholder="Enter name" />
            </Field>
          </FormDialog>
        </>
      )}
    >
      <SpecimenGroup label="Basic form dialog">
        <Button variant="primary" onClick={() => setBasicOpen(true)}>
          Add user
        </Button>
        <FormDialog
          open={basicOpen}
          title="Add new user"
          description="Invite a user to this workspace."
          submitLabel="Add user"
          submitting={submitting}
          onSubmit={handleBasicSubmit}
          onCancel={() => setBasicOpen(false)}
          onOpenChange={(open) => setBasicOpen(open ? true : null)}
        >
          <Field label="Full name" id="form-dialog-full-name">
            <TextInput value={name} onValueChange={setName} placeholder="Enter name" />
          </Field>
          <Field label="Role" id="form-dialog-role">
            <Select options={roleOptions} value={role} onValueChange={setRole} placeholder="Select role" />
          </Field>
        </FormDialog>
      </SpecimenGroup>

      <SpecimenGroup label="With error state">
        <Button
          variant="secondary"
          onClick={() => {
            setErrorOpen(true);
            setError(null);
          }}
        >
          Try with error
        </Button>
        <FormDialog
          open={errorOpen}
          title="Create account"
          submitLabel="Create"
          submitting={submitting}
          error={error}
          onSubmit={handleErrorSubmit}
          onCancel={() => {
            setErrorOpen(false);
            setError(null);
          }}
          onOpenChange={(open) => {
            if (!open) {
              setErrorOpen(null);
              setError(null);
            }
          }}
        >
          <Field label="Email" id="form-dialog-email">
            <TextInput value="existing@example.com" placeholder="Enter email" />
          </Field>
        </FormDialog>
      </SpecimenGroup>

      <SpecimenGroup label="Shell mode with custom actions">
        <Button
          variant="ghost"
          onClick={() => {
            setShellOpen(true);
            setSuccess(null);
          }}
        >
          Open settings shell
        </Button>
        <FormDialog
          open={shellOpen}
          title="Edit workspace settings"
          subtitle="Update shared defaults for this workspace."
          width="40rem"
          submitting={submitting}
          success={success}
          showDefaultActions={false}
          onCancel={() => {
            setShellOpen(false);
            setSuccess(null);
          }}
          onOpenChange={(open) => setShellOpen(open ? true : null)}
          body={(bodySubmitting) => (
            <>
              <Field label="Workspace name" id="form-dialog-workspace-name">
                <TextInput value="Northstar" disabled={bodySubmitting} />
              </Field>
              <Field label="Default role" id="form-dialog-default-role">
                <Select options={roleOptions} value="editor" disabled={bodySubmitting} />
              </Field>
            </>
          )}
          actions={(actionsSubmitting) => (
            <FormActions align="end">
              <Button
                variant="ghost"
                onClick={() => {
                  setShellOpen(false);
                  setSuccess(null);
                }}
                disabled={actionsSubmitting}
              >
                Cancel
              </Button>
              <Button variant="primary" onClick={handleShellSubmit} disabled={actionsSubmitting}>
                {actionsSubmitting ? "Saving..." : "Save changes"}
              </Button>
            </FormActions>
          )}
        />
      </SpecimenGroup>

      {lastAction ? (
        <SpecimenGroup label="Last action">
          <p style={{ margin: 0 }}>{lastAction}</p>
        </SpecimenGroup>
      ) : null}
    </SpecimenLayout>
  );
}

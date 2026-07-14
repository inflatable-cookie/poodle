import { useState } from "react";
import {
  Button,
  ConfirmAction,
  ContextMenu,
  DebugDialog,
  ErrorBoundary,
  FormDialog,
  FormLayout,
  IconProvider,
  Icon,
  SplitButton,
  TextInput,
  type MenuItem,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const menuItems: MenuItem[] = [
  { value: "open", label: "Open" },
  { value: "rename", label: "Rename" },
  { kind: "separator", label: "", value: "sep" },
  { value: "delete", label: "Delete" },
];

function Exploder() {
  const [broken, setBroken] = useState(false);
  if (broken) {
    throw new Error("Intentional specimen crash");
  }
  return (
    <Button variant="secondary" onClick={() => setBroken(true)}>
      Crash this subtree
    </Button>
  );
}

function SweepDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [formOpen, setFormOpen] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  return (
    <>
      <SpecimenSection title="ConfirmAction">
        <ConfirmAction
          title="Delete project?"
          description="This cannot be undone."
          triggerLabel="Delete project"
          onConfirm={() => setLastEvent("confirm:yes")}
          onCancel={() => setLastEvent("confirm:no")}
        />
      </SpecimenSection>

      <SpecimenSection title="ContextMenu">
        <ContextMenu items={menuItems} ariaLabel="File actions" onAction={(value) => setLastEvent(`ctx:${value}`)}>
          <div
            style={{ padding: "1.5rem", border: "1px dashed var(--poodle-color-border-default)" }}
            data-testid="ctx-target"
          >
            Right-click me
          </div>
        </ContextMenu>
      </SpecimenSection>

      <SpecimenSection title="DebugDialog">
        <DebugDialog value={{ build: "g12.007", components: 131, ok: true }} />
      </SpecimenSection>

      <SpecimenSection title="ErrorBoundary">
        <ErrorBoundary>
          <Exploder />
        </ErrorBoundary>
      </SpecimenSection>

      <SpecimenSection title="FormDialog + FormLayout">
        <Button onClick={() => setFormOpen(true)}>Open form</Button>
        <FormDialog
          open={formOpen}
          title="New project"
          subtitle="Projects group related work."
          submitting={submitting}
          error={lastEvent === "form:fail" ? "Name already taken." : null}
          onSubmit={() => {
            setSubmitting(true);
            setTimeout(() => {
              setSubmitting(false);
              setFormOpen(false);
              setLastEvent("form:submitted");
            }, 150);
          }}
          onCancel={() => setLastEvent("form:cancel")}
          onOpenChange={setFormOpen}
        >
          <TextInput ariaLabel="Project name" placeholder="Project name" />
        </FormDialog>
        <FormLayout
          description="Standalone layout"
          fieldErrors={{ name: "Required", slug: "Already exists" }}
          actions={<Button size="sm">Save</Button>}
        >
          <TextInput ariaLabel="Field A" placeholder="Field A" />
        </FormLayout>
      </SpecimenSection>

      <SpecimenSection title="IconProvider">
        <IconProvider icons={{ "custom-dot": [["circle", { cx: "12", cy: "12", r: "6" }]] }}>
          <span data-testid="custom-icon">
            <Icon name="custom-dot" ariaLabel="Custom dot" />
          </span>
        </IconProvider>
      </SpecimenSection>

      <SpecimenSection title="SplitButton">
        <SplitButton
          items={menuItems}
          onClick={() => setLastEvent("split:primary")}
          onAction={(value) => setLastEvent(`split:${value}`)}
        >
          Save
        </SplitButton>
        <SplitButton items={menuItems} loading>
          Saving
        </SplitButton>
      </SpecimenSection>

      {lastEvent ? (
        <SpecimenSection title="Last event">
          <p data-testid="last-event">{lastEvent}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "sweep",
  title: "ConfirmAction / ContextMenu / FormDialog / SplitButton",
  render: () => <SweepDemo />,
});

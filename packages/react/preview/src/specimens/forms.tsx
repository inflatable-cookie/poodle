import { useState } from "react";
import { CodeInput, Field, Popover, TextInput, Button } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function PopoverSpecimen() {
  const [openCount, setOpenCount] = useState(0);
  return (
    <SpecimenSection title="Popover">
      <Row>
        <Popover
          ariaLabel="Details"
          trigger={<Button variant="secondary">Open popover</Button>}
          onOpenChange={(open) => open && setOpenCount((n) => n + 1)}
        >
          <div style={{ padding: "0.75rem", display: "grid", gap: "0.5rem" }}>
            <p style={{ margin: 0 }}>Popover content</p>
            <Button variant="primary">Focusable</Button>
          </div>
        </Popover>
        <span data-testid="popover-opens">opens: {openCount}</span>
      </Row>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "popover", title: "Popover", render: () => <PopoverSpecimen /> });

registerSpecimen({
  slug: "field",
  title: "Field",
  render: () => (
    <SpecimenSection title="Field">
      <Field id="field-demo" label="Workspace name" description="Shown in the sidebar" required>
        <TextInput id="field-demo" placeholder="Acme Inc" ariaLabel="Workspace name" />
      </Field>
      <Field id="field-err" label="Email" error="Not a valid address" validationState="invalid">
        <TextInput id="field-err" defaultValue="nope@" ariaLabel="Email" />
      </Field>
    </SpecimenSection>
  ),
});

function CodeInputSpecimen() {
  const [code, setCode] = useState("");
  const [done, setDone] = useState("");
  return (
    <SpecimenSection title="CodeInput">
      <CodeInput value={code} onValueChange={setCode} onComplete={setDone} label="One-time code" />
      <span data-testid="code-value">code: {code}</span>
      <span data-testid="code-complete">complete: {done}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "code-input", title: "CodeInput", render: () => <CodeInputSpecimen /> });

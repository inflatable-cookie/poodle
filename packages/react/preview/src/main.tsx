import { StrictMode, useState } from "react";
import { createRoot } from "react-dom/client";
import { Button, Checkbox, Tabs } from "@poodle/react";

import "@poodle/svelte-tokens/styles.css";
import "@poodle/svelte-tokens/theme-dark.css";

function App() {
  const [clicks, setClicks] = useState(0);
  const [agreed, setAgreed] = useState(false);
  const [tab, setTab] = useState("overview");

  return (
    <main style={{ padding: "2rem", display: "grid", gap: "2rem", maxWidth: "40rem", color: "var(--poodle-color-text-primary)", background: "var(--poodle-color-background-canvas)", minHeight: "100vh", fontFamily: "var(--poodle-typography-body-family)" }}>
      <h1>Poodle React pilot</h1>

      <section data-testid="buttons" style={{ display: "flex", gap: "0.75rem" }}>
        <Button variant="primary" onClick={() => setClicks((count) => count + 1)}>
          Primary ({clicks})
        </Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="ghost">Ghost</Button>
        <Button variant="secondary" tone="danger">
          Danger
        </Button>
        <Button variant="primary" disabled>
          Disabled
        </Button>
      </section>

      <section data-testid="checkboxes" style={{ display: "grid", gap: "0.5rem" }}>
        <Checkbox label="Uncontrolled default" defaultChecked />
        <Checkbox label={`Controlled: ${agreed ? "agreed" : "not agreed"}`} checked={agreed} onCheckedChange={setAgreed} />
        <Checkbox label="Read-only checked" checked readOnly />
        <Checkbox label="Mixed" mixed />
        <Checkbox label="Disabled" disabled />
      </section>

      <section data-testid="tabs">
        <Tabs
          ariaLabel="Pilot sections"
          items={[
            { value: "overview", label: "Overview" },
            { value: "spec", label: "Spec", disabled: true },
            { value: "usage", label: "Usage" },
            { value: "notes", label: "Notes" },
          ]}
          value={tab}
          onValueChange={setTab}
        >
          {(active) => <p>Active panel: {active}</p>}
        </Tabs>
      </section>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);

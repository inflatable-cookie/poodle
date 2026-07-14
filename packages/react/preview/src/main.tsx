import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "@poodle/svelte-tokens/styles.css";
import "@poodle/svelte-tokens/theme-dark.css";

import { specimens, useHashRoute } from "./harness";
// Specimen modules self-register on import.
import "./specimens/button";
import "./specimens/checkbox";
import "./specimens/tabs";
import "./specimens/display";
import "./specimens/layout";
import "./specimens/controls";
import "./specimens/controls2";
import "./specimens/text-entry";
import "./specimens/text-entry2";
import "./specimens/forms";
import "./specimens/forms2";
import "./specimens/order-by";
import "./specimens/overlays";
import "./specimens/overlays2";
import "./specimens/misc";
import "./specimens/overlays3";
import "./specimens/data";
import "./specimens/date-pickers";
import "./specimens/editable-list";
import "./specimens/log-list";
import "./specimens/tree";
import "./specimens/data-table";

function App() {
  const slug = useHashRoute();
  const all = specimens();
  const visible = slug ? all.filter((s) => s.slug === slug) : all;

  return (
    <main
      className="poodle-react-preview"
      style={{
        padding: "2rem",
        display: "grid",
        gap: "1.25rem",
        alignContent: "start",
        maxWidth: "56rem",
        margin: "0 auto",
        color: "var(--poodle-color-text-primary)",
        background: "var(--poodle-color-background-canvas)",
        minHeight: "100vh",
        fontFamily: "var(--poodle-typography-body-family)",
      }}
    >
      <header style={{ display: "flex", alignItems: "baseline", gap: "1rem" }}>
        <h1 style={{ margin: 0, fontSize: "1.125rem" }}>Poodle React preview</h1>
        <nav style={{ display: "flex", flexWrap: "wrap", gap: "0.5rem", fontSize: "0.8125rem" }}>
          <a href="#" style={{ color: "var(--poodle-color-text-secondary)" }}>
            all
          </a>
          {all.map((s) => (
            <a key={s.slug} href={`#components/${s.slug}`} style={{ color: "var(--poodle-color-accent-base)" }}>
              {s.slug}
            </a>
          ))}
        </nav>
      </header>
      {visible.length === 0 ? <p>No specimen registered for “{slug}”.</p> : visible.map((s) => <div key={s.slug}>{s.render()}</div>)}
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);

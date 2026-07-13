import { useEffect, useState, type ReactNode } from "react";

/**
 * Specimen harness: hash-routed component gallery mirroring the Svelte
 * preview's `#components/<kebab-slug>` scheme so the same Playwright
 * probes drive both frameworks.
 */

export interface Specimen {
  slug: string;
  title: string;
  render: () => ReactNode;
}

const registry: Specimen[] = [];

export function registerSpecimen(specimen: Specimen): void {
  registry.push(specimen);
  registry.sort((a, b) => a.slug.localeCompare(b.slug));
}

export function specimens(): readonly Specimen[] {
  return registry;
}

function slugFromHash(): string | null {
  const match = window.location.hash.match(/^#components\/([a-z0-9-]+)$/);
  return match ? match[1] : null;
}

export function useHashRoute(): string | null {
  const [slug, setSlug] = useState<string | null>(slugFromHash);
  useEffect(() => {
    const onChange = () => setSlug(slugFromHash());
    window.addEventListener("hashchange", onChange);
    return () => window.removeEventListener("hashchange", onChange);
  }, []);
  return slug;
}

export function SpecimenSection({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section
      style={{
        display: "grid",
        gap: "0.75rem",
        padding: "1rem",
        border: "0.0625rem solid var(--poodle-color-border-subtle)",
        borderRadius: "var(--poodle-radius-surface)",
        background: "var(--poodle-color-background-surface)",
      }}
    >
      <h2 style={{ margin: 0, fontSize: "0.8125rem", color: "var(--poodle-color-text-secondary)", textTransform: "uppercase", letterSpacing: "0.04em" }}>
        {title}
      </h2>
      {children}
    </section>
  );
}

export function Row({ children }: { children: ReactNode }) {
  return <div style={{ display: "flex", flexWrap: "wrap", gap: "0.75rem", alignItems: "center" }}>{children}</div>;
}

// `import.meta.glob` is Vite's, and the anatomy smoke test uses it to
// enumerate every component module so new ones are covered automatically.
// Vite resolves only under node_modules/.bun here, so `types: ["vite/client"]`
// cannot find it. Declare the one member the suite uses rather than depending
// on a path that varies with the installer. Mirrors
// packages/svelte/components/src/components.d.ts.
interface ImportMeta {
  glob: (pattern: string, options?: { eager?: boolean }) => Record<string, unknown>;
}

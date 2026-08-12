// Components import their own stylesheet as a side effect. Vite resolves these;
// the type layer needs telling. Mirrors packages/svelte/preview/src/app.d.ts.
declare module "*.css";

// `import.meta.glob` is Vite's, and the suite uses it to enumerate components
// for the smoke test. Vite resolves only under node_modules/.bun here, so
// `types: ["vite/client"]` cannot find it. Declare the one member we use
// rather than depending on a path that varies with the installer.
interface ImportMeta {
  glob: (pattern: string, options?: { eager?: boolean }) => Record<string, unknown>;
}

// Components import their own stylesheet as a side effect. Vite resolves these;
// the type layer needs telling. Mirrors packages/svelte/preview/src/app.d.ts.
declare module "*.css";

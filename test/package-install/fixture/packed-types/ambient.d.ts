// The packed Svelte package imports its own token stylesheets for side effects.
// A bundler resolves those; `tsc` needs to be told they exist. This is the only
// ambient declaration the type proof adds — nothing here touches `HistoryEntry`.
declare module "*.css";

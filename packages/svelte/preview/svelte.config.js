import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

const config = {
  preprocess: vitePreprocess(),
  compilerOptions: {
    // Runes everywhere: legacy syntax fails at compile time instead of a
    // rune silently flipping a file's mode and killing its plain `let`s
    // (the Dialog specimen regression).
    runes: true,
  },
};

export default config;

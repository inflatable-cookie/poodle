import { fileURLToPath } from "node:url";

const core = fileURLToPath(new URL("../../packages/core/src/index.ts", import.meta.url));
const coreStyles = fileURLToPath(new URL("../../packages/core/src/styles", import.meta.url));

export default {
  root: fileURLToPath(new URL(".", import.meta.url)),
  resolve: {
    alias: {
      "@inflatable-cookie/poodle-core/styles": coreStyles,
      "@inflatable-cookie/poodle-core": core,
    },
  },
  server: {
    host: "127.0.0.1",
    strictPort: true,
  },
};

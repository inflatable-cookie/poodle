import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeParityReport } from "./parity-report";

writeParityReport({
  artifact: "packages/svelte/preview/artifacts/parity-report.json",
  previewDir: path.resolve(path.dirname(fileURLToPath(import.meta.url)), ".."),
});

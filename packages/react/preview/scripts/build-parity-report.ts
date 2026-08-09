import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeParityReport } from "../../../svelte/preview/scripts/parity-report";

writeParityReport({
  artifact: "packages/react/preview/artifacts/parity-report.json",
  previewDir: path.resolve(path.dirname(fileURLToPath(import.meta.url)), ".."),
  frameworks: ["@inflatable-cookie/poodle-react", "@inflatable-cookie/poodle-svelte"],
  sharedContractNote:
    "@inflatable-cookie/poodle-react is a reference-faithful re-implementation of the same component contracts as @inflatable-cookie/poodle-svelte, running the same @inflatable-cookie/poodle-core machines. Suite-level parity data is authored canonically in the Svelte preview and applies to both DOM runtimes.",
});
